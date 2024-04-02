use super::*;
use super::common::*;
use std::collections::VecDeque;
use std::mem::size_of;

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct PairRequest {
    #[bw(assert(*id == CommandId::Pair))]
    pub id: CommandId,
    #[bw(assert(*sub_id == PairCommandSubId::Pair))]
    pub sub_id: PairCommandSubId,
    pub pair_key: PairKey
}

impl PairRequest {
    pub const fn new(pair_key: PairKey) -> Self {
        Self {
            id: CommandId::Pair,
            sub_id: PairCommandSubId::Pair,
            pair_key
        }
    }
}

impl CommandRequest for PairRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct PairKeyRequest {
    #[bw(assert(*id == CommandId::Pair))]
    pub id: CommandId,
    #[bw(assert(*sub_id == PairCommandSubId::CurrentPairKey))]
    pub sub_id: PairCommandSubId
}

impl PairKeyRequest {
    pub const fn new() -> Self {
        Self {
            id: CommandId::Pair,
            sub_id: PairCommandSubId::CurrentPairKey
        }
    }
}

impl CommandRequest for PairKeyRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct BatteryRequest {
    #[bw(assert(*id == CommandId::Battery))]
    pub id: CommandId
}

impl BatteryRequest {
    pub const fn new() -> Self {
        Self {
            id: CommandId::Battery
        }
    }
}

impl CommandRequest for BatteryRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct SetDateTimeRequest {
    #[bw(assert(*id == CommandId::DateTime))]
    pub id: CommandId,
    pub date: Date,
    pub hour: u8,
    pub min: u8,
    pub sec: u8
}

impl SetDateTimeRequest {
    pub const fn new(date: Date, hour: u8, min: u8, sec: u8) -> Self {
        Self {
            id: CommandId::DateTime,
            date,
            hour,
            min,
            sec
        }
    }
}

impl CommandRequest for SetDateTimeRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct FirmwareRequest {
    #[bw(assert(*id == CommandId::Firmware))]
    pub id: CommandId
}

impl FirmwareRequest {
    pub const fn new() -> Self {
        Self {
            id: CommandId::Firmware
        }
    }
}

impl CommandRequest for FirmwareRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct ResetRequest {
    #[bw(assert(*id == CommandId::Firmware))]
    pub id: CommandId,
    pub sub_id: ResetCommandSubId
}

impl ResetRequest {
    pub const fn new(sub_id: ResetCommandSubId) -> Self {
        Self {
            id: CommandId::Reset,
            sub_id
        }
    }
}

impl CommandRequest for ResetRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct ConfigDisplayFormatsRequest {
    #[bw(assert(*id == CommandId::Config))]
    pub id: CommandId,
    pub distance_unit: DistanceUnit,
    pub time_fmt: TimeFormat
}

impl ConfigDisplayFormatsRequest {
    pub const fn new(distance_unit: DistanceUnit, time_fmt: TimeFormat) -> Self {
        Self {
            id: CommandId::Config,
            distance_unit,
            time_fmt
        }
    }
}

impl CommandRequest for ConfigDisplayFormatsRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct AlertStartBatchRequest {
    #[bw(assert(*id == CommandId::Alert))]
    pub id: CommandId,
    #[bw(assert(*batch_idx == 0))]
    pub batch_idx: u8,
    pub alert_type: AlertType,
    pub msg_full_len_bytes: u8,
    pub msg_start_text_utf16be: Vec<u16>
}

impl AlertStartBatchRequest {
    pub const fn new(alert_type: AlertType, msg_full_len_bytes: u8, msg_start_text_utf16be: Vec<u16>) -> Self {
        Self {
            id: CommandId::Alert,
            batch_idx: 0,
            alert_type,
            msg_full_len_bytes,
            msg_start_text_utf16be
        }
    }
}

impl CommandRequest for AlertStartBatchRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite, Default)]
pub struct AlertNextBatchRequest {
    #[bw(assert(*id == CommandId::Alert))]
    pub id: CommandId,
    #[bw(assert(*batch_idx > 0))]
    pub batch_idx: u8,
    pub msg_next_text_utf16be: Vec<u16>
}

impl AlertNextBatchRequest {
    pub const fn new(batch_idx: u8, msg_next_text_utf16be: Vec<u16>) -> Self {
        Self {
            id: CommandId::Alert,
            batch_idx,
            msg_next_text_utf16be
        }
    }
}

impl CommandRequest for AlertNextBatchRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct AlertPushRequest {
    #[bw(assert(*id == CommandId::Alert))]
    pub id: CommandId,
    #[bw(assert(*push_ident == Self::PUSH_IDENT_VALUE))]
    pub push_ident: u8
}

impl AlertPushRequest {
    pub const PUSH_IDENT_VALUE: u8 = 0xFD;

    pub const fn new() -> Self {
        Self {
            id: CommandId::Alert,
            push_ident: Self::PUSH_IDENT_VALUE
        }
    }
}

impl CommandRequest for AlertPushRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
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
    write::<AlertStartBatchRequest>(watch, alert_batch_0_req).await?;

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
        write::<AlertNextBatchRequest>(watch, alert_batch_i_req).await?;
        batch_idx += 1;
    }

    write::<AlertPushRequest>(watch, AlertPushRequest::new()).await?;
    Ok(())
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct SetUserInfoRequest {
    #[bw(assert(*id == CommandId::UserInfo))]
    pub id: CommandId,
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

impl SetUserInfoRequest {
    pub const fn new(height_cm: u8, weight_kg: u8, screen_show_timeout_seconds: u8, step_goal: u16, lift_wrist_mode: UserLiftWristMode, age: u8, gender: UserGender) -> Self {
        // Note: default unk values are the same ones Hello Haylou app uses
        // (TODO: are they any meaningful fields as well? probably...)
        Self {
            id: CommandId::UserInfo,
            unk0: 0,
            height_cm,
            unk1: 0,
            weight_kg,
            screen_show_timeout_seconds,
            unk2: 0,
            unk3: 0,
            step_goal_be: step_goal.swap_bytes(),
            lift_wrist_mode,
            unk4: 160,
            unk5: 0,
            age,
            gender,
            unk6: 0,
            unk7: 1,
            unk8: 1,
            unk9: 40
        }
    }
}

impl CommandRequest for SetUserInfoRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct SetWeatherTodayRequest {
    #[bw(assert(*id == CommandId::Weather))]
    pub id: CommandId,
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
            id: CommandId::Weather,
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
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Clone, PartialEq, Eq, Debug, BinWrite)]
pub struct SetWeatherFollowingDaysRequest {
    #[bw(assert(*id == CommandId::Weather))]
    pub id: CommandId,
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
            id: CommandId::Weather,
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
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]

pub struct RecordedStepsRequest {
    #[bw(assert(*id == CommandId::RecordedStepsGeneral))]
    pub id: CommandId,
    #[bw(assert(*sub_id == RecordedStepsCommandSubId::Request))]
    pub sub_id: RecordedStepsCommandSubId,
    #[bw(assert(*unk == 1))]
    pub unk: u8
}

impl RecordedStepsRequest {
    pub const fn new() -> Self {
        Self {
            id: CommandId::RecordedStepsGeneral,
            sub_id: RecordedStepsCommandSubId::Request,
            unk: 1
        }
    }
}

impl CommandRequest for RecordedStepsRequest {
    const WRITE_TYPE: WriteType = WriteType::WithResponse;
    const CHAR: Characteristic = super::CHAR_GENERAL_RW_1;
}