use std::fmt::{Debug, Formatter, Result};

use super::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum CommandId {
    Invalid = 0x00,
    Config = 0x01,
    DateTime = 0x04,
    UserInfo = 0x05,
    Reset = 0x07,
    Steps = 0x09,
    RecordedStepsData2 = 0x0A,
    Alert = 0x0F,
    Weather = 0x11,
    DevicePulseUnk2 = 0x12,
    HeartRateMenuData = 0x16,
    HeartRate = 0x18,
    Pair = 0x20,
    Firmware = 0xA1,
    Battery = 0xA2,
    StepsUnk2 = 0xB1,
    RecordedStepsGeneral = 0xB2,
    SilentMode = 0xBE,
    DevicePulse = 0xD1,
    HeartRateMenuGeneral = 0xE5,
    HeartRateUnk2 = 0xF7,
    Sport = 0xFD
}

impl Default for CommandId {
    fn default() -> Self {
        CommandId::Invalid
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum PairCommandSubId {
    Pair = 0x02,
    CurrentPairKey = 0x03
}

pub type PairKey = [u8; 4];

#[derive(Copy, Clone, PartialEq, Eq, BinRead, BinWrite)]
pub struct Date {
    pub year_be: u16,
    pub month: u8,
    pub day: u8
}

impl Date {
    pub const fn new(year: u16, month: u8, day: u8) -> Self {
        Self {
            year_be: year.swap_bytes(),
            month,
            day
        }
    }

    pub fn year(&self) -> u16 {
        self.year_be.swap_bytes()
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:04}/{:02}/{:02}", self.year_be.swap_bytes(), self.month, self.day)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum ResetCommandSubId {
    ResetAndReboot = 0x00,
    ResetAndPowerOff = 0x01
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum DevicePulseType {
    HangCall = 2,
    MusicPauseResume = 7,
    MusicNext = 8,
    MusicPrevious = 9,
    Ring = 10
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum TimeFormat {
    H24 = 1,
    H12 = 2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum DistanceUnit {
    Metric = 1,
    Imperial = 2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
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

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum UserGender {
    Male = 1,
    Female = 2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum UserLiftWristMode {
    Off = 0,
    On = 1
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum WeatherDate {
    Today = 1,
    FollowingDays = 2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
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

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum HeartRateCommandSubId {
    Enable = 0x01,
    Disable = 0x02,
    DataPeriodic = 0x03,
    DataToday = 0x04,
    DataRequest = 0xFA,
    DataEnd = 0xFD
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum HeartRateMenuCommandSubId {
    LeavingMenu = 0x00,
    InMenu = 0x11,
}

pub const INVALID_HEART_RATE: u8 = 255;

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum SilentModeCommandSubId {
    ModeChanged = 2
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum SilentModeStatus {
    Off = 0x0,
    On = 0x8,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum SportCommandSubId {
    Finish = 0x00,
    Start = 0x11
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum SportKind {
    Jogging = 1,
    Biking = 2,
    Climbing = 8,
    FastRunning = 9,
    Basketball = 10,
    Football = 11,
    Spinning = 18,
    Yoga = 19,
    IndoorRunning = 21,
    Gimnastics = 22,
    Rowing = 23,
    IntegratedTraining = 25
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
pub struct StepsEntry {
    pub date: Date,
    pub hour: u8,
    pub new_all_step_count_be: u16,
    pub unk_1: u8,
    pub last_new_run_step_min: u8,
    pub unk_2: u8,
    pub new_run_step_count_be: u16,
    pub unk_3: u8,
    pub last_new_walk_step_min: u8,
    pub unk_4: u8,
    pub new_walk_step_count_be: u16
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum RecordedStepsCommandSubId {
    Request = 0x03,
    End = 0xFD
}