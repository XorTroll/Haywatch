use eframe::{egui::{self, Context}, epaint::Stroke};
use egui_plot::{BarChart, Bar, Plot, Text, PlotPoint, GridMark};
use std::{sync::{Arc, Mutex}, ops::RangeInclusive};
use std::error::Error;
use std::time::Duration;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType, Characteristic, ValueNotification};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::StreamExt;
use tokio::time;
use ls02::{self, common::{Date, AlertType, INVALID_HEART_RATE}};

mod db;

struct WatchContext {
    peripheral: Option<Peripheral>,
    is_connected: bool,
    battery: Option<u8>,
    pair_key: Option<ls02::common::PairKey>,
    firmware: String,
    hr_db_dates: Vec<Date>,
    rs_db_dates: Vec<Date>
}

impl WatchContext {
    pub fn new() -> Self {
        Self {
            peripheral: None,
            is_connected: false,
            battery: None,
            pair_key: None,
            firmware: String::new(),
            hr_db_dates: Vec::new(),
            rs_db_dates: Vec::new()
        }
    }
}

struct MainApp {
    started: bool,
    ctx: Arc<Mutex<WatchContext>>,
    text: String,
    text_type: AlertType,
    cur_hr_db_date_idx: usize,
    cur_hr_db_date_chart_bars: Option<Vec<Bar>>,
    cur_hr_db_day_entry: Option<db::HeartRateDailyDatabaseEntry>,
    cur_rs_db_date_idx: usize,
    show_msg_window: bool,
    show_hr_window: bool,
    show_rs_window: bool
}

async fn request_hr_data(watch: &Peripheral) -> Result<(), Box<dyn Error>> {
    ls02::write::<ls02::data2::req::HeartRateDataRequest>(watch, ls02::data2::req::HeartRateDataRequest::new()).await?;

    Ok(())
}

async fn request_rs_data(watch: &Peripheral) -> Result<(), Box<dyn Error>> {
    ls02::write::<ls02::data2::req::RecordedStepsRequest>(watch, ls02::data2::req::RecordedStepsRequest::new()).await?;

    Ok(())
}

impl MainApp {
    pub fn new() -> Self {
        Self {
            started: false,
            ctx: Arc::new(Mutex::new(WatchContext::new())),
            text: String::new(),
            text_type: AlertType::Message,
            cur_hr_db_date_idx: 0,
            cur_hr_db_date_chart_bars: None,
            cur_hr_db_day_entry: None,
            cur_rs_db_date_idx: 0,
            show_msg_window: false,
            show_hr_window: false,
            show_rs_window: false
        }
    }

    fn msg_window(&mut self, ctx: &Context) {
        if let Ok(watch_ctx) = self.ctx.lock() {
            egui::Window::new("Message sender")
            .open(&mut self.show_msg_window)
            .resizable(true)
            .show(ctx, |ui| {
                ui.text_edit_multiline(&mut self.text);
                egui::ComboBox::from_label("Message type")
                    .selected_text(format!("{:?}", self.text_type))
                    .show_ui(ui, |ui| {
                        macro_rules! foreach {
                            ($val:expr) => {
                                ui.selectable_value(&mut self.text_type, $val, format!("{:?}", $val));
                            };
                        }

                        foreach!(AlertType::Call);
                        foreach!(AlertType::QQ);
                        foreach!(AlertType::WeChat);
                        foreach!(AlertType::Message);
                        foreach!(AlertType::Facebook);
                        foreach!(AlertType::Twitter);
                        foreach!(AlertType::WhatsApp);
                        foreach!(AlertType::Skype);
                        foreach!(AlertType::Messenger);
                        foreach!(AlertType::Hangouts);
                        foreach!(AlertType::LINE);
                        foreach!(AlertType::LinkedIn);
                        foreach!(AlertType::Instagram);
                        foreach!(AlertType::Viber);
                        foreach!(AlertType::KakaoTalk);
                        foreach!(AlertType::VK);
                        foreach!(AlertType::Snapchat);
                        foreach!(AlertType::GooglePlus);
                        foreach!(AlertType::Email);
                        foreach!(AlertType::Flickr);
                        foreach!(AlertType::Tumblr);
                        foreach!(AlertType::Pinterest);
                        foreach!(AlertType::YouTube);
                    });
                if ui.button("Send").clicked() {
                    let per = watch_ctx.peripheral.clone().unwrap();
                    let text_c = self.text.clone();
                    let text_type_c = self.text_type.clone();
                    tokio::spawn(async move {
                        ls02::general::req::write_send_alert(&per, text_type_c, text_c).await.unwrap()
                    });
                }
            });
        }
    }

    fn hr_window(&mut self, ctx: &Context) {
        if let Ok(watch_ctx) = self.ctx.lock() {
            egui::Window::new("Heart rate data")
            .open(&mut self.show_hr_window)
            .resizable(true)
            .show(ctx, |ui| {
                if ui.button("Retrieve heart rate data").clicked() {
                    println!("{:?}", db::HeartRateDailyDatabase::list_dates());
                    let per = watch_ctx.peripheral.clone().unwrap();
                    tokio::spawn(async move {
                        request_hr_data(&per).await.unwrap()
                    });
                }

                ui.separator();

                if !watch_ctx.hr_db_dates.is_empty() {
                    egui::ComboBox::from_label("Day")
                        .show_index(ui, &mut self.cur_hr_db_date_idx, watch_ctx.hr_db_dates.len(), |i| format!("{:?}", watch_ctx.hr_db_dates[i]));

                    if ui.button("Display data").clicked() {
                        let date = watch_ctx.hr_db_dates[self.cur_hr_db_date_idx];
                        let db = db::HeartRateDailyDatabase::create_load_by_date(date);
                        println!("{:?}", db);

                        let mut bars: Vec<Bar> = Vec::new();
                        self.cur_hr_db_day_entry = None;
                        for entry in db.entries {
                            if entry.heart_rate != INVALID_HEART_RATE {
                                let x = (entry.hour as u32 * 60 + entry.min as u32) as f64;
                                let bar = Bar::new(x, entry.heart_rate as f64);
                                bars.push(bar);
                            }
                            else if entry.avg_heart_rate != INVALID_HEART_RATE {
                                self.cur_hr_db_day_entry = Some(entry.clone());
                            }
                        }
                        self.cur_hr_db_date_chart_bars = Some(bars);
                    }
                }

                if let Some(hr_chart) = self.cur_hr_db_date_chart_bars.as_ref() {
                    let x_fmt = |y: GridMark, x, _range: &RangeInclusive<f64>| {
                        let real_x = y.value as u32;
                        let min = real_x % 60;
                        let hour = real_x / 60;
                        format!("{:02}:{:02}", hour, min)
                    };

                    Plot::new("hr_plot")
                        .x_axis_formatter(x_fmt)
                        .show(ui, |plot_ui| {
                            let elm_fmt = |a: &Bar, b: &BarChart| {
                                let real_x = a.argument as u32;
                                let min = real_x % 60;
                                let hour = real_x / 60;
                                format!("{} ({:02}:{:02})", a.value, hour, min)
                            };

                            let bar_ch = BarChart::new(hr_chart.clone())
                                .element_formatter(Box::new(elm_fmt));
                            plot_ui.bar_chart(bar_ch);

                            if let Some(cur_day_entry) = self.cur_hr_db_day_entry.as_ref() {
                                let avg_hr_msg = format!("Average heart rate: {}", cur_day_entry.avg_heart_rate);
                                let max_hr_msg = format!("Max. heart rate: {}", cur_day_entry.max_heart_rate);
                                let min_hr_msg = format!("Min. heart rate: {}", cur_day_entry.min_heart_rate);
                                plot_ui.text(Text::new(PlotPoint::new(60.0, -10.0), format!("{}\n{}\n{}", avg_hr_msg, max_hr_msg, min_hr_msg)));
                            }
                        });
                }
            });
        }
    }

    fn rs_window(&mut self, ctx: &Context) {
        if let Ok(watch_ctx) = self.ctx.lock() {
            egui::Window::new("Recorded step data")
            .open(&mut self.show_rs_window)
            .resizable(true)
            .show(ctx, |ui| {
                if ui.button("Retrieve recorded step data").clicked() {
                    println!("{:?}", db::RecordedStepsDailyDatabase::list_dates());
                    let per = watch_ctx.peripheral.clone().unwrap();
                    tokio::spawn(async move {
                        request_rs_data(&per).await.unwrap()
                    });
                }

                ui.separator();

                if !watch_ctx.hr_db_dates.is_empty() {
                    egui::ComboBox::from_label("Day")
                        .show_index(ui, &mut self.cur_rs_db_date_idx, watch_ctx.rs_db_dates.len(), |i| format!("{:?}", watch_ctx.rs_db_dates[i]));

                    if ui.button("Display data").clicked() {
                        /*
                        let date = watch_ctx.hr_db_dates[self.cur_hr_db_date_idx];
                        let db = db::HeartRateDailyDatabase::create_load_by_date(date);
                        println!("{:?}", db);

                        let mut bars: Vec<Bar> = Vec::new();
                        self.cur_hr_db_day_entry = None;
                        for entry in db.entries {
                            if entry.heart_rate != INVALID_HEART_RATE {
                                let x = (entry.hour as u32 * 60 + entry.min as u32) as f64;
                                let bar = Bar::new(x, entry.heart_rate as f64);
                                bars.push(bar);
                            }
                            else if entry.avg_heart_rate != INVALID_HEART_RATE {
                                self.cur_hr_db_day_entry = Some(entry.clone());
                            }
                        }
                        self.cur_hr_db_date_chart_bars = Some(bars);
                        */
                    }
                }

                if let Some(hr_chart) = self.cur_hr_db_date_chart_bars.as_ref() {
                    let x_fmt = |y: GridMark, x, _range: &RangeInclusive<f64>| {
                        let real_x = y.value as u32;
                        let min = real_x % 60;
                        let hour = real_x / 60;
                        format!("{:02}:{:02}", hour, min)
                    };

                    Plot::new("hr_plot")
                        .x_axis_formatter(x_fmt)
                        .show(ui, |plot_ui| {
                            let elm_fmt = |a: &Bar, b: &BarChart| {
                                let real_x = a.argument as u32;
                                let min = real_x % 60;
                                let hour = real_x / 60;
                                format!("{} ({:02}:{:02})", a.value, hour, min)
                            };

                            let bar_ch = BarChart::new(hr_chart.clone())
                                .element_formatter(Box::new(elm_fmt));
                            plot_ui.bar_chart(bar_ch);

                            if let Some(cur_day_entry) = self.cur_hr_db_day_entry.as_ref() {
                                let avg_hr_msg = format!("Average heart rate: {}", cur_day_entry.avg_heart_rate);
                                let max_hr_msg = format!("Max. heart rate: {}", cur_day_entry.max_heart_rate);
                                let min_hr_msg = format!("Min. heart rate: {}", cur_day_entry.min_heart_rate);
                                plot_ui.text(Text::new(PlotPoint::new(60.0, -10.0), format!("{}\n{}\n{}", avg_hr_msg, max_hr_msg, min_hr_msg)));
                            }
                        });
                }
            });
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.started {
            let watch_ctx_c = self.ctx.clone();
            let egui_ctx_c = ctx.clone();
            tokio::spawn(async move {
                watch_discover_thread(watch_ctx_c, egui_ctx_c).await.unwrap()
            });
            
            self.started = true;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                if let Ok(ctx) = self.ctx.lock() {
                    if ctx.is_connected {
                        ui.label("LS02 (connected)");

                        ui.separator();

                        ui.label(if let Some(battery) = ctx.battery { format!("Battery: {}%", battery) } else { "<unk battery>".to_string() });
                        ui.label(if let Some(pair_key) = ctx.pair_key { format!("Pair key: {:?}", pair_key) } else { "<unk pair key>".to_string() });
                        ui.label(if ctx.firmware.is_empty() { "<unk firmware>".to_string() } else { format!("Firmware: {}", ctx.firmware) });

                        ui.separator();

                        if ui.button("Message sender").clicked() {
                            self.show_msg_window = true;
                        }
                        if ui.button("Heart rate data viewer").clicked() {
                            self.show_hr_window = true;
                        }
                        if ui.button("Recorded step data viewer").clicked() {
                            self.show_rs_window = true;
                        }
                    }
                    else {
                        ui.label("LS02 NOT connected...");
                    }
                }
            });
        });

        self.msg_window(ctx);
        self.hr_window(ctx);
        self.rs_window(ctx);
    }
}

const WATCH_PAIR_KEY: ls02::common::PairKey = [1, 2, 3, 4];

async fn watch_initial_send_thread(watch: Peripheral, watch_ctx: Arc<Mutex<WatchContext>>, egui_ctx: egui::Context) -> Result<(), Box<dyn Error>> {
    watch_ctx.lock().unwrap().peripheral = Some(watch.clone());

    // First of all, pair
    ls02::write::<ls02::general::req::PairRequest>(&watch,ls02::general::req::PairRequest::new(WATCH_PAIR_KEY)).await?;

    // Retrieve the key used to pair (just in case, but not necessary)
    ls02::write::<ls02::general::req::PairKeyRequest>(&watch, ls02::general::req::PairKeyRequest::new()).await?;

    // Get current battery
    ls02::write::<ls02::general::req::BatteryRequest>(&watch, ls02::general::req::BatteryRequest::new()).await?;

    // Get watch firmware
    ls02::write::<ls02::general::req::FirmwareRequest>(&watch, ls02::general::req::FirmwareRequest::new()).await?;

    // Heart rate data
    request_hr_data(&watch).await?;
    
    // Recorded steps data
    request_rs_data(&watch).await?;

    Ok(())
}

async fn watch_receive_thread(watch: Peripheral, watch_ctx: Arc<Mutex<WatchContext>>, egui_ctx: egui::Context, notif_char: Characteristic, name: &str) -> Result<(), Box<dyn Error>> {
    watch.subscribe(&notif_char).await?;

    let mut notif_stream = watch.notifications().await?;
    loop {
        if let Some(value) = notif_stream.next().await {
            if let ls02::CommandResponseResult::Ok(res) = ls02::read::<ls02::general::res::BatteryResponse>(&value, &notif_char).await {
                watch_ctx.lock().unwrap().battery = Some(res.battery_percentage);
                egui_ctx.request_repaint();
            }

            if let ls02::CommandResponseResult::Ok(res) = ls02::read::<ls02::general::res::PairKeyResponse>(&value, &notif_char).await {
                watch_ctx.lock().unwrap().pair_key = Some(res.cur_pair_key);
                egui_ctx.request_repaint();
            }
            
            if let ls02::CommandResponseResult::Ok(res) = ls02::read::<ls02::general::res::FirmwareResponse>(&value, &notif_char).await {
                watch_ctx.lock().unwrap().firmware = String::from_utf8(res.name.to_vec()).unwrap();
                egui_ctx.request_repaint();
            }

            if let ls02::CommandResponseResult::Ok(res) = ls02::read::<ls02::data2::res::HeartRateDataTodayResponse>(&value, &notif_char).await {
                let mut db = db::HeartRateDailyDatabase::create_load_by_date(res.date);

                let entry = db::HeartRateDailyDatabaseEntry {
                    hour: res.hour,
                    min: res.min,
                    heart_rate: INVALID_HEART_RATE,
                    max_heart_rate: res.max_heart_rate,
                    min_heart_rate: res.min_heart_rate,
                    avg_heart_rate: res.avg_heart_rate
                };
                db.push(entry);

                db.save(res.date);
                if let Ok(mut ctx) = watch_ctx.lock() {
                    ctx.hr_db_dates = db::HeartRateDailyDatabase::list_dates();
                }
            }

            if let ls02::CommandResponseResult::Ok(res) = ls02::read::<ls02::data2::res::HeartRateDataDayHourEntryResponse>(&value, &notif_char).await {
                let mut db = db::HeartRateDailyDatabase::create_load_by_date(res.date);

                let mut cur_time = res.hour as u32 * 60;
                for heart_rate in res.heart_rates {
                    let entry = db::HeartRateDailyDatabaseEntry {
                        hour: (cur_time / 60) as u8,
                        min: (cur_time % 60) as u8,
                        heart_rate,
                        max_heart_rate: INVALID_HEART_RATE,
                        min_heart_rate: INVALID_HEART_RATE,
                        avg_heart_rate: INVALID_HEART_RATE
                    };
                    // println!("Got hourly heart rate entry: {:02}:{:02}", entry.hour, entry.min);
                    db.push(entry);

                    // Entries are separated by 10min
                    cur_time += 10;
                }

                db.save(res.date);
                if let Ok(mut ctx) = watch_ctx.lock() {
                    ctx.hr_db_dates = db::HeartRateDailyDatabase::list_dates();
                }
            }

            if let ls02::CommandResponseResult::Ok(res) = ls02::read::<ls02::data2::res::RecordedStepsEntryResponse>(&value, &notif_char).await {
                let mut db = db::RecordedStepsDailyDatabase::create_load_by_date(res.entry.date);

                let new_walk_step_count = res.entry.new_walk_step_count_be.swap_bytes();
                if new_walk_step_count > 0 {
                    let entry = db::RecordedStepsDailyDatabaseEntry {
                        hour: res.entry.hour,
                        min: res.entry.last_new_walk_step_min,
                        new_step_kind: db::RecordedStepKind::Walk,
                        new_step_count: new_walk_step_count
                    };

                    println!("Walked {} steps at {:?} at {:02}:{:02}", new_walk_step_count, res.entry.date, entry.hour, entry.min);
                    db.push(entry);
                }

                let new_run_step_count = res.entry.new_run_step_count_be.swap_bytes();
                if new_run_step_count > 0 {
                    let entry = db::RecordedStepsDailyDatabaseEntry {
                        hour: res.entry.hour,
                        min: res.entry.last_new_run_step_min,
                        new_step_kind: db::RecordedStepKind::Run,
                        new_step_count: new_run_step_count
                    };

                    println!("Ran {} steps at {:?} at {:02}:{:02}", new_run_step_count, res.entry.date, entry.hour, entry.min);
                    db.push(entry);
                }

                db.save(res.entry.date);
                if let Ok(mut ctx) = watch_ctx.lock() {
                    ctx.rs_db_dates = db::RecordedStepsDailyDatabase::list_dates();
                }
            }
        }
    }
}

async fn watch_discover_thread(watch_ctx: Arc<Mutex<WatchContext>>, egui_ctx: egui::Context) -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        panic!("No Bluetooth adapters found...");
    }

    loop {
        let res: Result<(), Box<dyn Error>> = {
            for adapter in adapter_list.iter() {
                println!("Starting scan...");
                let scan_filter = ScanFilter {
                    services: vec![ls02::SERVICE_GENERAL, ls02::SERVICE_DATA2]
                };
                adapter.start_scan(scan_filter).await.expect("Can't scan BLE adapter for connected devices...");
    
                time::sleep(Duration::from_secs(2)).await;
    
                let peripherals = adapter.peripherals().await?;
                if peripherals.is_empty() {
                    eprintln!("[haywatch.Discover] ERROR: BLE peripheral devices were not found...");
                }
                else {
                    for peripheral in &peripherals {
                        let properties = peripheral.properties().await?;
                        let is_connected = peripheral.is_connected().await?;

                        match &properties.as_ref().unwrap().local_name {
                            Some(name) => {
                                if name == ls02::DEVICE_NAME {
                                    println!("[haywatch.Discover] Found LS02 '{}'! (already connected: {}) --- MAC: {}", name, is_connected, properties.as_ref().unwrap().address);
        
                                    let is_connected = peripheral.is_connected().await?;
                                    if !is_connected {
                                        peripheral.connect().await?;
                                    }
        
                                    println!("[haywatch.Discover] Discovering services...");
                                    peripheral.discover_services().await?;
        
                                    watch_ctx.lock().unwrap().is_connected = true;
                                    egui_ctx.request_repaint();
        
                                    let peripheral_c = peripheral.clone();
                                    let watch_ctx_c = watch_ctx.clone();
                                    let egui_ctx_c = egui_ctx.clone();
                                    tokio::spawn(async move {
                                        watch_receive_thread(peripheral_c, watch_ctx_c, egui_ctx_c, ls02::CHAR_GENERAL_N_1, "General").await.unwrap()
                                    });
        
                                    let peripheral_c = peripheral.clone();
                                    let watch_ctx_c = watch_ctx.clone();
                                    let egui_ctx_c = egui_ctx.clone();
                                    tokio::spawn(async move {
                                        watch_receive_thread(peripheral_c, watch_ctx_c, egui_ctx_c, ls02::CHAR_DATA2_N, "Data2").await.unwrap()
                                    });
        
                                    let peripheral_c = peripheral.clone();
                                    let watch_ctx_c = watch_ctx.clone();
                                    let egui_ctx_c = egui_ctx.clone();
                                    tokio::spawn(async move {
                                        watch_initial_send_thread(peripheral_c, watch_ctx_c, egui_ctx_c).await.unwrap()
                                    });
        
                                    while peripheral.is_connected().await? {}
        
                                    println!("[haywatch.Main] Watch disconnected...");
                                    peripheral.disconnect().await?;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
    
            Ok(())
        };
        if res.is_ok() {
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "haywatch",
        options,
        Box::new(|_cc| Box::new(MainApp::new())),
    )
}