use super::*;
use super::common::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct PairKeyResponse {
    #[br(assert(id == CommandId::Pair))]
    pub id: CommandId,
    #[br(assert(sub_id == PairCommandSubId::CurrentPairKey))]
    pub sub_id: PairCommandSubId,
    pub cur_pair_key: PairKey
}

impl CommandResponse for PairKeyResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct BatteryResponse {
    #[br(assert(id == CommandId::Battery))]
    pub id: CommandId,
    pub battery_percentage: u8
}

impl CommandResponse for BatteryResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct SetDateTimeResponse {
    #[br(assert(id == CommandId::DateTime))]
    pub id: CommandId,
    pub year_be: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8
}

impl CommandResponse for SetDateTimeResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct FirmwareResponse {
    #[br(assert(id == CommandId::Firmware))]
    pub id: CommandId,
    pub name: [u8; 13]
}

impl CommandResponse for FirmwareResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct DevicePulseResponse {
    #[br(assert(id == CommandId::DevicePulse))]
    pub id: CommandId,
    pub pulse_type: DevicePulseType
}

impl CommandResponse for DevicePulseResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct SetWeatherResponse {
    #[br(assert(id == CommandId::Weather))]
    pub id: CommandId,
    pub weather_date: WeatherDate
}

impl CommandResponse for SetWeatherResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateMenuDataPeriodicResponse {
    #[br(assert(id == CommandId::HeartRateMenuGeneral))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateMenuCommandSubId::InMenu))]
    pub sub_id: HeartRateMenuCommandSubId,
    pub unk_maybe_pad: u8,
    pub heart_rate: u8
}

impl CommandResponse for HeartRateMenuDataPeriodicResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateMenuDataLeavingResponse {
    #[br(assert(id == CommandId::HeartRateMenuGeneral))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateMenuCommandSubId::LeavingMenu))]
    pub sub_id: HeartRateMenuCommandSubId,
    pub unk_maybe_pad: u8,
    pub heart_rate: u8
}

impl CommandResponse for HeartRateMenuDataLeavingResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateMenuChangeResponse {
    #[br(assert(id == CommandId::HeartRateMenuGeneral))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateMenuCommandSubId::InMenu))]
    pub sub_id: HeartRateMenuCommandSubId
}

impl CommandResponse for HeartRateMenuChangeResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct SilentModeChangeResponse {
    #[br(assert(id == CommandId::SilentMode))]
    pub id: CommandId,
    #[br(assert(sub_id == SilentModeCommandSubId::ModeChanged))]
    pub sub_id: SilentModeCommandSubId,
    pub mode: SilentModeStatus,
    pub unk: [u8; 0x11]
}

impl CommandResponse for SilentModeChangeResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct SportStatusResponse {
    #[br(assert(id == CommandId::Sport))]
    pub id: CommandId,
    pub sub_id: SportCommandSubId,
    pub kind: SportKind,
    pub unk: u8
}

impl CommandResponse for SportStatusResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct RecordedStepsEntryResponse {
    #[br(assert(id == CommandId::RecordedStepsGeneral))]
    pub id: CommandId,
    pub entry: StepsEntry
}

impl CommandResponse for RecordedStepsEntryResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct RecordedStepsEndResponse {
    #[br(assert(id == CommandId::RecordedStepsGeneral))]
    pub id: CommandId,
    #[br(assert(sub_id == RecordedStepsCommandSubId::End))]
    pub sub_id: RecordedStepsCommandSubId,
    pub unk: u8
}

impl CommandResponse for RecordedStepsEndResponse {
    const CHAR: Characteristic = super::CHAR_GENERAL_N_1;
}