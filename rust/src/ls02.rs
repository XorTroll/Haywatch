use btleplug::api::{Characteristic, CharPropFlags};
use uuid::uuid;

pub const WATCH_DEVICE_NAME: &'static str = "Haylou Smart Watch 2";

pub const CHAR_NOTIF_GENERIC: Characteristic = Characteristic { uuid: uuid!("00002A05-0000-1000-8000-00805F9B34FB"), service_uuid: uuid!("00001801-0000-1000-8000-00805F9B34FB"), properties: CharPropFlags::INDICATE };
pub const CHAR_NOTIF_MAIN: Characteristic = Characteristic { uuid: uuid!("000033F2-0000-1000-8000-00805F9B34FB"), service_uuid: uuid!("000055FF-0000-1000-8000-00805F9B34FB"), properties: CharPropFlags::NOTIFY };
pub const CHAR_NOTIF_FETCH_DATA: Characteristic = Characteristic { uuid: uuid!("00006002-0000-1000-8000-00805F9B34FB"), service_uuid: uuid!("000060FF-0000-1000-8000-00805F9B34FB"), properties: CharPropFlags::NOTIFY };
pub const CHAR_NOTIF_FETCH_DATA_SLEEP: Characteristic = Characteristic { uuid: uuid!("00006102-0000-1000-8000-00805F9B34FB"), service_uuid: uuid!("000061FF-0000-1000-8000-00805F9B34FB"), properties: CharPropFlags::NOTIFY };
pub const CHAR_NOTIF_FETCH_FIRMWARE: Characteristic = Characteristic { uuid: uuid!("000034F2-0000-1000-8000-00805F9B34FB"), service_uuid: uuid!("000056FF-0000-1000-8000-00805F9B34FB"), properties: CharPropFlags::NOTIFY };

pub const CHAR_WS_01: Characteristic = Characteristic { uuid: uuid!("000033F1-0000-1000-8000-00805F9B34FB"), service_uuid: uuid!("000055FF-0000-1000-8000-00805F9B34FB"), properties: CharPropFlags::WRITE };

pub const MAX_WRITE_SIZE: usize = 48;

pub mod cmd;