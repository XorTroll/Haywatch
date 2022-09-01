use std::collections::VecDeque;
use std::error::Error;
use std::mem::size_of;
use binrw::{BinRead, BinReaderExt, BinWrite, BinWriterExt, io::Cursor};
use btleplug::api::{Peripheral as _, WriteType, Characteristic, ValueNotification};
use btleplug::platform::Peripheral;

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum CommandId {
    Config = 0x01,
    DateTime = 0x04,
    UserInfo = 0x05,
    Reset = 0x07,
    Alert = 0x0F,
    Weather = 0x11,
    Pair = 0x20,
    Firmware = 0xA1,
    Battery = 0xA2,
    DevicePulse = 0xD1
}

pub trait CommandRequest: Sized + BinWrite where <Self as BinWrite>::Args: Default {
    const ID: CommandId;
    const WRITE_TYPE: WriteType;
    const CHAR: Characteristic;

    fn build(&self) -> Vec<u8> {
        let mut cmd: Vec<u8> = Vec::new();
        let mut cmd_writer = Cursor::new(&mut cmd);

        cmd_writer.write_le(&Self::ID).unwrap();
        cmd_writer.write_le(self).unwrap();

        cmd
    }

    fn len(&self) -> usize {
        self.build().len()
    }
}

pub trait CommandResponse: Sized + BinRead where <Self as BinRead>::Args: Default {
    const ID: CommandId;

    fn try_parse(raw: &Vec<u8>) -> Option<Self> {
        let mut cmd_reader = Cursor::new(raw);
        if let Ok(id) = cmd_reader.read_le::<CommandId>() {
            if id == Self::ID {
                if let Ok(cmd) = cmd_reader.read_le::<Self>() {
                    Some(cmd)
                }
                else {
                    None
                }
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}

#[inline]
pub async fn write<C: CommandRequest>(watch: &Peripheral, cmd: C) -> Result<(), Box<dyn Error>> where <C as BinWrite>::Args: Default {
    watch.write(&C::CHAR, &cmd.build(), C::WRITE_TYPE).await?;
    Ok(())
}

#[inline]
pub async fn read<C: CommandResponse>(value_notif: &ValueNotification) -> Option<C> where <C as BinRead>::Args: Default {
    C::try_parse(&value_notif.value)
}

// Actual commands

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum PairOperationType {
    Pair = 2,
    CurrentPairKey = 3
}

pub type PairKey = [u8; 4];

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct PairRequest {
    #[bw(assert(*op_type == PairOperationType::Pair))]
    pub op_type: PairOperationType,
    pub pair_key: PairKey
}

impl PairRequest {
    pub const fn new(pair_key: PairKey) -> Self {
        Self {
            op_type: PairOperationType::Pair,
            pair_key
        }
    }
}

impl CommandRequest for PairRequest {
    const ID: CommandId = CommandId::Pair;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct PairKeyRequest {
    #[bw(assert(*op_type == PairOperationType::CurrentPairKey))]
    pub op_type: PairOperationType
}

impl PairKeyRequest {
    pub const fn new() -> Self {
        Self {
            op_type: PairOperationType::CurrentPairKey
        }
    }
}

impl CommandRequest for PairKeyRequest {
    const ID: CommandId = CommandId::Pair;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct PairKeyResponse {
    #[br(assert(op_type == PairOperationType::CurrentPairKey))]
    pub op_type: PairOperationType,
    pub cur_pair_key: PairKey
}

impl CommandResponse for PairKeyResponse {
    const ID: CommandId = CommandId::Pair;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct BatteryRequest {
}

impl BatteryRequest {
    pub const fn new() -> Self {
        Self {}
    }
}

impl CommandRequest for BatteryRequest {
    const ID: CommandId = CommandId::Battery;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct BatteryResponse {
    pub battery_percentage: u8
}

impl CommandResponse for BatteryResponse {
    const ID: CommandId = CommandId::Battery;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct SetDateTimeRequest {
    pub year_be: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8
}

impl SetDateTimeRequest {
    pub const fn new(year: u16, month: u8, day: u8, hour: u8, min: u8, sec: u8) -> Self {
        Self {
            year_be: year.swap_bytes(),
            month,
            day,
            hour,
            min,
            sec
        }
    }
}

impl CommandRequest for SetDateTimeRequest {
    const ID: CommandId = CommandId::DateTime;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct SetDateTimeResponse {
    pub year_be: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8
}

impl CommandResponse for SetDateTimeResponse {
    const ID: CommandId = CommandId::DateTime;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct FirmwareRequest {
}

impl FirmwareRequest {
    pub const fn new() -> Self {
        Self {}
    }
}

impl CommandRequest for FirmwareRequest {
    const ID: CommandId = CommandId::Firmware;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct FirmwareResponse {
    pub name: [u8; 13]
}

impl CommandResponse for FirmwareResponse {
    const ID: CommandId = CommandId::Firmware;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum ResetOperationType {
    ResetAndReboot = 0,
    ResetAndPowerOff = 1
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct ResetRequest {
    pub op_type: ResetOperationType
}

impl ResetRequest {
    pub const fn new(op_type: ResetOperationType) -> Self {
        Self {
            op_type
        }
    }
}

impl CommandRequest for ResetRequest {
    const ID: CommandId = CommandId::Reset;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum DevicePulseType {
    HangCall = 2,
    MusicPauseResume = 7,
    MusicNext = 8,
    MusicPrevious = 9,
    Ring = 10
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct DevicePulseResponse {
    pub pulse_type: DevicePulseType
}

impl CommandResponse for DevicePulseResponse {
    const ID: CommandId = CommandId::DevicePulse;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum TimeFormat {
    H24 = 1,
    H12 = 2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum DistanceUnit {
    Metric = 1,
    Imperial = 2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct ConfigDisplayFormatsRequest {
    pub distance_unit: DistanceUnit,
    pub time_fmt: TimeFormat
}

impl ConfigDisplayFormatsRequest {
    pub const fn new(distance_unit: DistanceUnit, time_fmt: TimeFormat) -> Self {
        Self {
            distance_unit,
            time_fmt
        }
    }
}

impl CommandRequest for ConfigDisplayFormatsRequest {
    const ID: CommandId = CommandId::Config;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum AlertType {
    Call = 0,
    QQ = 1,
    WeChat = 2,
    Message = 4,
    Facebook = 5,
    Twitter = 6,
    WhatsApp = 7,
    Skype = 8,
    Messenger = 9,
    Hangouts = 10,
    LINE = 11,
    LinkedIn = 12,
    Instagram = 13,
    Viber = 14,
    KakaoTalk = 15,
    VK = 16,
    Snapchat = 17,
    GooglePlus = 18,
    Email = 19,
    Flickr = 20,
    Tumblr = 21,
    Pinterest = 22,
    YouTube = 23
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct AlertStartBatchRequest {
    pub batch_idx: u8,
    pub alert_type: AlertType,
    pub msg_full_len_bytes: u8,
    pub msg_start_text_utf16be: Vec<u16>
}

impl AlertStartBatchRequest {
    pub const fn new(alert_type: AlertType, msg_full_len_bytes: u8, msg_start_text_utf16be: Vec<u16>) -> Self {
        Self {
            batch_idx: 0,
            alert_type,
            msg_full_len_bytes,
            msg_start_text_utf16be
        }
    }
}

impl CommandRequest for AlertStartBatchRequest {
    const ID: CommandId = CommandId::Alert;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct AlertNextBatchRequest {
    pub batch_idx: u8,
    pub msg_next_text_utf16be: Vec<u16>
}

impl AlertNextBatchRequest {
    pub const fn new(batch_idx: u8, msg_next_text_utf16be: Vec<u16>) -> Self {
        Self {
            batch_idx,
            msg_next_text_utf16be
        }
    }
}

impl CommandRequest for AlertNextBatchRequest {
    const ID: CommandId = CommandId::Alert;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct AlertPushRequest {
    pub push_ident: u8
}

impl AlertPushRequest {
    pub const fn new() -> Self {
        Self {
            push_ident: 0xFD
        }
    }
}

impl CommandRequest for AlertPushRequest {
    const ID: CommandId = CommandId::Alert;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

pub async fn write_send_alert(watch: &Peripheral, alert_type: AlertType, text: String) -> Result<(), Box<dyn Error>> {
    let mut text_utf16_be = text.encode_utf16().map(|ch| ch.swap_bytes()).collect::<VecDeque<_>>();

    let mut alert_batch_0_req = AlertStartBatchRequest::new(alert_type, (text_utf16_be.len() * size_of::<u16>()) as u8, Vec::new());
    while alert_batch_0_req.len() < super::MAX_WRITE_SIZE {
        if let Some(next_ch) = text_utf16_be.pop_front() {
            alert_batch_0_req.msg_start_text_utf16be.push(next_ch);
        }
        else {
            break;
        }
    }
    write(watch, alert_batch_0_req).await?;

    let mut batch_idx = 1u8;
    while !text_utf16_be.is_empty() {
        let mut alert_batch_i_req = AlertNextBatchRequest::new(batch_idx, Vec::new());

        while alert_batch_i_req.len() < super::MAX_WRITE_SIZE {
            if let Some(next_ch) = text_utf16_be.pop_front() {
                alert_batch_i_req.msg_next_text_utf16be.push(next_ch);
            }
            else {
                break;
            }
        }
        write(watch, alert_batch_i_req).await?;
        batch_idx += 1;
    }

    write(watch, AlertPushRequest::new()).await?;
    Ok(())
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum UserGender {
    Male = 1,
    Female = 2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum UserLiftWristMode {
    Off = 0,
    On = 1
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct SetUserInfoRequest {
    pub unk0: u8,
    pub height_cm: u8,
    pub unk1: u8,
    pub weight_kg: u8,
    pub screen_show_timeout_seconds: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub step_goal_be: u16,
    pub lift_wrist_mode: UserLiftWristMode,
    pub unk4: u8,
    pub unk5: u8,
    pub age: u8,
    pub gender: UserGender,
    pub unk6: u8,
    pub unk7: u8,
    pub unk8: u8,
    pub unk9: u8
}

impl CommandRequest for SetUserInfoRequest {
    const ID: CommandId = CommandId::UserInfo;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum WeatherDate {
    Today = 1,
    FollowingDays = 2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[br(repr = u8)]
#[bw(repr = u8)]
pub enum WeatherType {
    Sunny = 1,
    SunnyCloudy = 2,
    Cloudy = 3,
    SunnyRainy = 4,
    Stormy = 5,
    Rainy = 6,
    SlighlyRainy = 7,
    VeryRainy = 8,
    Snowy = 9,
    S = 10,
    Foggy = 11,
    Windy = 12,
    Night = 13,
    CloudyNight = 14,
    RainyNight = 15
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct SetWeatherTodayRequest {
    #[bw(assert(*weather_date == WeatherDate::Today))]
    pub weather_date: WeatherDate,
    pub weather_type: WeatherType,
    pub unk: u8,
    pub cur_temperature: u8,
    pub max_temperature: u8,
    pub min_temperature: u8
}

impl SetWeatherTodayRequest {
    pub const fn new(weather_type: WeatherType, cur_temperature: u8, max_temperature: u8, min_temperature: u8) -> Self {
        Self {
            weather_date: WeatherDate::Today,
            weather_type,
            unk: 0,
            cur_temperature,
            max_temperature,
            min_temperature
        }
    }
}

impl CommandRequest for SetWeatherTodayRequest {
    const ID: CommandId = CommandId::Weather;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct SetWeatherFollowingDaysRequest {
    #[bw(assert(*weather_date == WeatherDate::FollowingDays))]
    pub weather_date: WeatherDate,

    pub next_1_weather_type: WeatherType,
    pub next_1_unk: u8,
    pub next_1_max_temperature: u8,
    pub next_1_min_temperature: u8,

    pub next_2_weather_type: WeatherType,
    pub next_2_unk: u8,
    pub next_2_max_temperature: u8,
    pub next_2_min_temperature: u8,

    pub next_3_weather_type: WeatherType,
    pub next_3_unk: u8,
    pub next_3_max_temperature: u8,
    pub next_3_min_temperature: u8
}

impl SetWeatherFollowingDaysRequest {
    pub const fn new(next_1_weather_type: WeatherType, next_1_max_temperature: u8, next_1_min_temperature: u8, next_2_weather_type: WeatherType, next_2_max_temperature: u8, next_2_min_temperature: u8, next_3_weather_type: WeatherType, next_3_max_temperature: u8, next_3_min_temperature: u8) -> Self {
        Self {
            weather_date: WeatherDate::FollowingDays,
            
            next_1_weather_type,
            next_1_unk: 0,
            next_1_max_temperature,
            next_1_min_temperature,

            next_2_weather_type,
            next_2_unk: 0,
            next_2_max_temperature,
            next_2_min_temperature,

            next_3_weather_type,
            next_3_unk: 0,
            next_3_max_temperature,
            next_3_min_temperature
        }
    }
}

impl CommandRequest for SetWeatherFollowingDaysRequest {
    const ID: CommandId = CommandId::Weather;
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_WS_01;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct SetWeatherResponse {
    pub weather_date: WeatherDate
}

impl CommandResponse for SetWeatherResponse {
    const ID: CommandId = CommandId::Weather;
}