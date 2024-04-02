use super::*;
use super::common::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateDataTodayResponse {
    #[br(assert(id == CommandId::HeartRate))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateCommandSubId::DataToday))]
    pub sub_id: HeartRateCommandSubId,
    pub date: Date,
    pub hour: u8,
    pub min: u8,
    pub max_heart_rate: u8,
    pub min_heart_rate: u8,
    pub avg_heart_rate: u8
}

impl CommandResponse for HeartRateDataTodayResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateDataEndResponse {
    #[br(assert(id == CommandId::HeartRate))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateCommandSubId::DataEnd))]
    pub sub_id: HeartRateCommandSubId,
    pub unk: u8
}

impl CommandResponse for HeartRateDataEndResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateDataDayHourEntryResponse {
    #[br(assert(id == CommandId::HeartRate))]
    pub id: CommandId,
    pub date: Date,
    pub hour: u8,
    pub heart_rates: [u8; 12]
}

impl CommandResponse for HeartRateDataDayHourEntryResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateDataPeriodicResponse {
    #[br(assert(id == CommandId::HeartRate))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateCommandSubId::DataPeriodic))]
    pub sub_id: HeartRateCommandSubId,
    pub date: Date,
    pub hour: u8,
    pub min: u8,
    pub heart_rate: u8
}

impl CommandResponse for HeartRateDataPeriodicResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateEnableResponse {
    #[br(assert(id == CommandId::HeartRate))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateCommandSubId::Enable))]
    pub sub_id: HeartRateCommandSubId
}

impl CommandResponse for HeartRateEnableResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateDisableResponse {
    #[br(assert(id == CommandId::HeartRate))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateCommandSubId::Disable))]
    pub sub_id: HeartRateCommandSubId
}

impl CommandResponse for HeartRateDisableResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct StepsResponse {
    #[br(assert(id == CommandId::Steps))]
    pub id: CommandId,
    pub date: Date,
    pub hour: u8,
    pub min: u8,
    pub new_step_count: u8,
    pub unk:[u8; 10]
}

impl CommandResponse for StepsResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateMenuPeriodicResponse {
    #[br(assert(id == CommandId::HeartRateMenuData))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateMenuCommandSubId::InMenu))]
    pub sub_id: HeartRateMenuCommandSubId,
    pub unk_maybe_pad: u8,
    pub heart_rate: u8
}

impl CommandResponse for HeartRateMenuPeriodicResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateMenuMoveDownResponse {
    #[br(assert(id == CommandId::HeartRateMenuData))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateMenuCommandSubId::InMenu))]
    pub sub_id: HeartRateMenuCommandSubId
}

impl CommandResponse for HeartRateMenuMoveDownResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateDataTodayUnk2Response {
    #[br(assert(id == CommandId::HeartRateUnk2))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateCommandSubId::DataToday))]
    pub sub_id: HeartRateCommandSubId,
    pub date: Date,
    pub hour: u8,
    pub min: u8,
    pub max_heart_rate: u8,
    pub min_heart_rate: u8,
    pub avg_heart_rate: u8
}

impl CommandResponse for HeartRateDataTodayUnk2Response {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct StepsUnk2Response {
    #[br(assert(id == CommandId::StepsUnk2))]
    pub id: CommandId,
    pub entry: StepsEntry
}

impl CommandResponse for StepsUnk2Response {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct HeartRateDataPeriodicUnk2Response {
    #[br(assert(id == CommandId::HeartRateUnk2))]
    pub id: CommandId,
    #[br(assert(sub_id == HeartRateCommandSubId::DataPeriodic))]
    pub sub_id: HeartRateCommandSubId,
    pub date: Date,
    pub hour: u8,
    pub min: u8,
    pub heart_rate: u8
}

impl CommandResponse for HeartRateDataPeriodicUnk2Response {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct DevicePulseUnk2Response {
    #[br(assert(id == CommandId::DevicePulseUnk2))]
    pub id: CommandId,
    pub pulse_type: DevicePulseType
}

impl CommandResponse for DevicePulseUnk2Response {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct RecordedStepsEntryResponse {
    #[br(assert(id == CommandId::RecordedStepsData2))]
    pub id: CommandId,
    pub entry: StepsEntry
}

impl CommandResponse for RecordedStepsEntryResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead)]
pub struct RecordedStepsEndResponse {
    #[br(assert(id == CommandId::RecordedStepsData2))]
    pub id: CommandId,
    #[br(assert(sub_id == RecordedStepsCommandSubId::End))]
    pub sub_id: RecordedStepsCommandSubId,
    pub unk: u8
}

impl CommandResponse for RecordedStepsEndResponse {
    const CHAR: Characteristic = super::CHAR_DATA2_N;
}