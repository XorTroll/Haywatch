use super::*;
use super::common::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]

pub struct HeartRateDataRequest {
    #[bw(assert(*id == CommandId::HeartRate))]
    pub id: CommandId,
    #[bw(assert(*sub_id == HeartRateCommandSubId::DataRequest))]
    pub sub_id: HeartRateCommandSubId
}

impl HeartRateDataRequest {
    pub const fn new() -> Self {
        Self {
            id: CommandId::HeartRate,
            sub_id: HeartRateCommandSubId::DataRequest
        }
    }
}

impl CommandRequest for HeartRateDataRequest {
    const WRITE_TYPE: WriteType = WriteType::WithoutResponse;
    const CHAR: Characteristic = super::CHAR_DATA2_RW;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]

pub struct HeartRateEnableRequest {
    #[bw(assert(*id == CommandId::HeartRate))]
    pub id: CommandId,
    #[bw(assert(*sub_id == HeartRateCommandSubId::Enable))]
    pub sub_id: HeartRateCommandSubId
}

impl HeartRateEnableRequest {
    pub const fn new() -> Self {
        Self {
            id: CommandId::HeartRate,
            sub_id: HeartRateCommandSubId::Enable
        }
    }
}

impl CommandRequest for HeartRateEnableRequest {
    const WRITE_TYPE: WriteType = WriteType::WithoutResponse;
    const CHAR: Characteristic = super::CHAR_DATA2_RW;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]

pub struct HeartRateDisableRequest {
    #[bw(assert(*id == CommandId::HeartRate))]
    pub id: CommandId,
    #[bw(assert(*sub_id == HeartRateCommandSubId::Disable))]
    pub sub_id: HeartRateCommandSubId
}

impl HeartRateDisableRequest {
    pub const fn new() -> Self {
        Self {
            id: CommandId::HeartRate,
            sub_id: HeartRateCommandSubId::Disable
        }
    }
}

impl CommandRequest for HeartRateDisableRequest {
    const WRITE_TYPE: WriteType = WriteType::WithoutResponse;
    const CHAR: Characteristic = super::CHAR_DATA2_RW;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinWrite)]

pub struct RecordedStepsRequest {
    #[bw(assert(*id == CommandId::RecordedStepsData2))]
    pub id: CommandId,
    #[bw(assert(*sub_id == RecordedStepsCommandSubId::Request))]
    pub sub_id: RecordedStepsCommandSubId,
    #[bw(assert(*unk == 1))]
    pub unk: u8
}

impl RecordedStepsRequest {
    pub const fn new() -> Self {
        Self {
            id: CommandId::RecordedStepsData2,
            sub_id: RecordedStepsCommandSubId::Request,
            unk: 1
        }
    }
}

impl CommandRequest for RecordedStepsRequest {
    const WRITE_TYPE: WriteType = WriteType::WithoutResponse;
    const CHAR: Characteristic = super::CHAR_DATA2_RW;
}