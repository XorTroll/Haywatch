use btleplug::api::{
    Central, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Manager, Peripheral};
use futures::stream::StreamExt;
use std::error::Error;
use std::time::Duration;
use tokio::time;
use uuid::uuid;

mod ls02;

const WATCH_PAIR_KEY: ls02::cmd::PairKey = [1, 2, 3, 4];

async fn send_thread(watch: Peripheral) -> Result<(), Box<dyn Error>> {
    println!("[haywatch.Send] Sending pair request...");
    ls02::cmd::write(&watch, ls02::cmd::PairRequest::new(WATCH_PAIR_KEY)).await?;

    time::sleep(Duration::from_secs(1)).await;

    ls02::cmd::write(&watch, ls02::cmd::PairKeyRequest::new()).await?;

    time::sleep(Duration::from_secs(1)).await;

    println!("[haywatch.Send] Test cmd...");

    watch
        .write(
            &ls02::CHAR_WS_01,
            &[6, 0, 1, 1, 2, 1, 1, 1],
            WriteType::WithResponse,
        )
        .await?;

    Ok(())
}

async fn notification_receive_thread(
    watch: Peripheral,
    notif_char: Characteristic,
) -> Result<(), Box<dyn Error>> {
    watch.subscribe(&notif_char).await?;

    let mut notif_stream = watch.notifications().await?;
    loop {
        if let Some(data) = notif_stream.next().await {
            if let Some(pair_key_resp) = ls02::cmd::read::<ls02::cmd::PairKeyResponse>(&data).await
            {
                println!(
                    "[haywatch.Notify:{:?}] Watch is paired with key {:?}!",
                    notif_char.uuid, pair_key_resp.cur_pair_key
                );
            } else if let Some(battery_resp) =
                ls02::cmd::read::<ls02::cmd::BatteryResponse>(&data).await
            {
                println!(
                    "[haywatch.Notify:{:?}] Watch has {}% battery!",
                    notif_char.uuid, battery_resp.battery_percentage
                );
            } else if let Some(fw_resp) =
                ls02::cmd::read::<ls02::cmd::FirmwareResponse>(&data).await
            {
                println!(
                    "[haywatch.Notify:{:?}] Watch has firmware '{}'",
                    notif_char.uuid,
                    std::str::from_utf8(&fw_resp.name).unwrap()
                );
            } else if let Some(dev_pulse_resp) =
                ls02::cmd::read::<ls02::cmd::DevicePulseResponse>(&data).await
            {
                println!(
                    "[haywatch.Notify:{:?}] Watch sent a {:?} pulse to us!",
                    notif_char.uuid, dev_pulse_resp.pulse_type
                );
            } else if let Some(weather_resp) =
                ls02::cmd::read::<ls02::cmd::SetWeatherResponse>(&data).await
            {
                println!(
                    "[haywatch.Notify:{:?}] Successfully set {:?} weather data!",
                    notif_char.uuid, weather_resp.weather_date
                );
            } else {
                println!(
                    "[haywatch.Notify:{:?}] Watch sent unrecognized data: {:?}",
                    notif_char.uuid, data.value
                );
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("[haywatch.Main] ERROR: No Bluetooth adapters found...");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        let scan_filter = ScanFilter {
            services: vec![uuid!("000055FF-0000-1000-8000-00805F9B34FB")],
        };
        adapter
            .start_scan(scan_filter)
            .await
            .expect("[haywatch.Main] ERROR: Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(2)).await;

        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            eprintln!("[haywatch.Main] ERROR: BLE peripheral devices were not found...");
        } else {
            for peripheral in &peripherals {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;

                if let Some(local_name) = &properties.as_ref().unwrap().local_name && local_name == ls02::WATCH_DEVICE_NAME {
                    println!("[haywatch.Main] Found {}! (already connected: {}) --- MAC: {}", ls02::WATCH_DEVICE_NAME, is_connected, properties.as_ref().unwrap().address);

                    let is_connected = peripheral.is_connected().await?;
                    if !is_connected {
                        peripheral.connect().await?;
                    }

                    println!("[haywatch.Main] Discovering services...");
                    peripheral.discover_services().await?;

                    /*
                    for characteristic in peripheral.characteristics() {
                        println!("CHAR: {:?}", characteristic);
                    }
                    */
                    
                    let notif_generic_watch = peripheral.clone();
                    tokio::spawn(async move {
                        notification_receive_thread(notif_generic_watch, ls02::CHAR_NOTIF_MAIN.clone()).await.unwrap()
                    });
                    let notif_f_watch = peripheral.clone();
                    tokio::spawn(async move {
                        notification_receive_thread(notif_f_watch, ls02::CHAR_NOTIF_FETCH_FIRMWARE.clone()).await.unwrap()
                    });
                    let notif_d_watch = peripheral.clone();
                    tokio::spawn(async move {
                        notification_receive_thread(notif_d_watch, ls02::CHAR_NOTIF_FETCH_DATA.clone()).await.unwrap()
                    });

                    let send_watch = peripheral.clone();
                    tokio::spawn(async move {
                        send_thread(send_watch).await.unwrap()
                    });

                    while peripheral.is_connected().await? {}

                    println!("[haywatch.Main] {} disconnected...", ls02::WATCH_DEVICE_NAME);
                    peripheral.disconnect().await?;
                    break;
                }
            }
        }
    }

    Ok(())
}
