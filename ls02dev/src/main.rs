use std::error::Error;
use std::time::Duration;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType, Characteristic};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::StreamExt;
use tokio::time;
use ls02;

const WATCH_PAIR_KEY: ls02::common::PairKey = [1, 2, 3, 4];

async fn send_thread(watch: Peripheral) -> Result<(), Box<dyn Error>> {
    println!("[ls02dev.Send] Starting programmed commands...");
    ls02::write::<ls02::general::req::PairRequest>(&watch, ls02::general::req::PairRequest::new(WATCH_PAIR_KEY)).await?;

    time::sleep(Duration::from_secs(1)).await;

    ls02::write::<ls02::general::req::PairKeyRequest>(&watch, ls02::general::req::PairKeyRequest::new()).await?;

    time::sleep(Duration::from_secs(1)).await;

    // ls02::write::<ls02::general::req::RecordedStepsRequest>(&watch, ls02::general::req::RecordedStepsRequest::new()).await?;

    // time::sleep(Duration::from_secs(3)).await;

    // ls02::write::<ls02::data2::req::RecordedStepsRequest>(&watch, ls02::data2::req::RecordedStepsRequest::new()).await?;
    // watch.write(&ls02::CHAR_GENERAL_RW_1, &[37, 171, 2], WriteType::WithoutResponse).await?;

    ls02::write(&watch, ls02::data2::req::RecordedStepsRequest::new()).await?;

    Ok(())
}

async fn notification_receive_thread(watch: Peripheral, notif_char: Characteristic, name: &str) -> Result<(), Box<dyn Error>> {
    watch.subscribe(&notif_char).await?;

    let mut notif_stream = watch.notifications().await?;
    loop {
        if let Some(data) = notif_stream.next().await {
            let mut unk_res = true;

            macro_rules! do_response {
                ($t:ty) => {
                    match ls02::read::<$t>(&data, &notif_char).await {
                        ls02::CommandResponseResult::Ok(res) => {
                            println!("[ls02dev.Notify.{}] {:?}", name, res);
                            unk_res = false;
                        },
                        ls02::CommandResponseResult::ImproperCharacteristic => unk_res = false,
                        _ => {}
                    };
                };
            }

            do_response!(ls02::general::res::PairKeyResponse);
            do_response!(ls02::general::res::BatteryResponse);
            do_response!(ls02::general::res::SetDateTimeResponse);
            do_response!(ls02::general::res::FirmwareResponse);
            do_response!(ls02::general::res::DevicePulseResponse);
            do_response!(ls02::general::res::SetWeatherResponse);
            do_response!(ls02::general::res::HeartRateMenuDataPeriodicResponse);
            do_response!(ls02::general::res::HeartRateMenuDataLeavingResponse);
            do_response!(ls02::general::res::HeartRateMenuChangeResponse);
            do_response!(ls02::general::res::SilentModeChangeResponse);
            do_response!(ls02::general::res::SportStatusResponse);
            do_response!(ls02::general::res::RecordedStepsEntryResponse);
            do_response!(ls02::general::res::RecordedStepsEndResponse);

            do_response!(ls02::data2::res::HeartRateDataTodayResponse);
            do_response!(ls02::data2::res::HeartRateDataDayHourEntryResponse);
            do_response!(ls02::data2::res::HeartRateDataEndResponse);
            do_response!(ls02::data2::res::HeartRateDataPeriodicResponse);
            do_response!(ls02::data2::res::HeartRateEnableResponse);
            do_response!(ls02::data2::res::HeartRateDisableResponse);
            do_response!(ls02::data2::res::StepsResponse);
            do_response!(ls02::data2::res::HeartRateMenuPeriodicResponse);
            do_response!(ls02::data2::res::HeartRateMenuMoveDownResponse);
            do_response!(ls02::data2::res::HeartRateDataTodayUnk2Response);
            do_response!(ls02::data2::res::StepsUnk2Response);
            do_response!(ls02::data2::res::HeartRateDataPeriodicUnk2Response);
            do_response!(ls02::data2::res::DevicePulseUnk2Response);
            do_response!(ls02::data2::res::RecordedStepsEntryResponse);
            do_response!(ls02::data2::res::RecordedStepsEndResponse);

            if unk_res {
                println!("[ls02dev.Notify.{}] Unknown response: {:?}", name, &data.value);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("[ls02dev.Main] ERROR: No Bluetooth adapters found...");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        let scan_filter = ScanFilter {
            services: vec![ls02::SERVICE_GENERAL, ls02::SERVICE_DATA2]
        };
        adapter.start_scan(scan_filter).await.expect("[ls02dev.Main] ERROR: Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(2)).await;

        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            eprintln!("[ls02dev.Main] ERROR: BLE peripheral devices were not found...");
        }
        else {
            for peripheral in &peripherals {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;

                // TODO: we already check for LS02's services on the filter, do we really need to also check the device name?
                match &properties.as_ref().unwrap().local_name {
                    Some(name) => {
                        if name == ls02::DEVICE_NAME {
                            println!("[ls02dev.Main] Found {}! (already connected: {}) --- MAC: {}", ls02::DEVICE_NAME, is_connected, properties.as_ref().unwrap().address);
        
                            let is_connected = peripheral.is_connected().await?;
                            if !is_connected {
                                peripheral.connect().await?;
                            }
        
                            println!("[ls02dev.Main] Discovering services...");
                            peripheral.discover_services().await?;
        
                            let peripheral_c = peripheral.clone();
                            tokio::spawn(async move {
                                notification_receive_thread(peripheral_c, ls02::CHAR_GENERAL_N_1, "General").await.unwrap()
                            });
        
                            let peripheral_c = peripheral.clone();
                            tokio::spawn(async move {
                                notification_receive_thread(peripheral_c, ls02::CHAR_DATA2_N, "Data2").await.unwrap()
                            });
        
                            let peripheral_c = peripheral.clone();
                            tokio::spawn(async move {
                                send_thread(peripheral_c).await.unwrap()
                            });
        
                            while peripheral.is_connected().await? {}
        
                            println!("[ls02dev.Main] Watch disconnected...");
                            peripheral.disconnect().await?;
                            break;
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(())
}