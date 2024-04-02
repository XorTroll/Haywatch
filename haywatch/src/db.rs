use binrw::{BinRead, BinWrite, BinReaderExt, BinWriterExt};
use ls02::common::Date;
use std::{env, path::PathBuf, fs::{File, OpenOptions}};

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(magic = b"HRDE")]
pub struct HeartRateDailyDatabaseEntry {
    pub hour: u8,
    pub min: u8,
    pub heart_rate: u8,
    pub max_heart_rate: u8,
    pub min_heart_rate: u8,
    pub avg_heart_rate: u8
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(magic = b"HRDB")]
pub struct HeartRateDailyDatabase {
    pub entry_count: u32,
    #[br(count = entry_count)]
    pub entries: Vec<HeartRateDailyDatabaseEntry>
}

fn ensure_parent_path(path: PathBuf) -> PathBuf {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    path
}

fn ensure_path(path: PathBuf) -> PathBuf {
    std::fs::create_dir_all(&path).unwrap();
    path
}

impl HeartRateDailyDatabase {
    pub fn new() -> Self {
        Self {
            entry_count: 0,
            entries: Vec::new()
        }
    }

    fn get_base_db_path() -> PathBuf {
        ensure_path(env::current_exe().unwrap().parent().unwrap().join("db").join("hr"))
    }

    fn get_db_path(db_date: Date) -> PathBuf {
        ensure_parent_path(Self::get_base_db_path().join(format!("{}-{}-{}", db_date.year(), db_date.month(), db_date.day())))
    }

    pub fn create_load_by_date(date: Date) -> Self {
        let path = Self::get_db_path(date);
        if let Ok(mut file) = File::open(path) {
            file.read_le().unwrap()
        }
        else {
            Self::new()
        }
    }

    pub fn list_dates() -> Vec<Date> {
        let mut dates: Vec<Date> = Vec::new();
        let base_path = Self::get_base_db_path();
        for entry_res in std::fs::read_dir(base_path).unwrap() {
            let entry = entry_res.unwrap();
            let entry_name = entry.file_name();
            let w: Vec<&str> = entry_name.to_str().unwrap().split("-").collect();
            if w.len() == 3 {
                let year = u16::from_str_radix(w[0], 10).unwrap();
                let month = u8::from_str_radix(w[1], 10).unwrap();
                let day = u8::from_str_radix(w[2], 10).unwrap();
                dates.push(Date::new(year, month, day))
            }
        }
        dates
    }

    pub fn push(&mut self, entry: HeartRateDailyDatabaseEntry) {
        self.entries.retain(|old_entry| (old_entry.hour != entry.hour) || (old_entry.min != entry.min));
        self.entries.push(entry);
        self.entry_count = self.entries.len() as u32;
    }

    pub fn save(&self, date: Date) {
        let path = Self::get_db_path(date);
        // println!("Saving heart rate db: {:?}", path);
        let mut file = OpenOptions::new().create(true).write(true).open(path).unwrap();
        file.write_le(self).unwrap();
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, BinRead, BinWrite)]
#[repr(u8)]
#[brw(repr = u8)]
pub enum RecordedStepKind {
    Walk,
    Run
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(magic = b"RSDE")]
pub struct RecordedStepsDailyDatabaseEntry {
    pub hour: u8,
    pub min: u8,
    pub new_step_kind: RecordedStepKind,
    pub new_step_count: u16
}

#[derive(BinRead, BinWrite, Clone, Debug)]
#[brw(magic = b"RSDB")]
pub struct RecordedStepsDailyDatabase {
    pub entry_count: u32,
    #[br(count = entry_count)]
    pub entries: Vec<RecordedStepsDailyDatabaseEntry>
}

impl RecordedStepsDailyDatabase {
    pub fn new() -> Self {
        Self {
            entry_count: 0,
            entries: Vec::new()
        }
    }

    fn get_base_db_path() -> PathBuf {
        ensure_path(env::current_exe().unwrap().parent().unwrap().join("db").join("rs"))
    }

    fn get_db_path(db_date: Date) -> PathBuf {
        ensure_parent_path(Self::get_base_db_path().join(format!("{}-{}-{}", db_date.year(), db_date.month(), db_date.day())))
    }

    pub fn create_load_by_date(date: Date) -> Self {
        let path = Self::get_db_path(date);
        if let Ok(mut file) = File::open(path) {
            file.read_le().unwrap()
        }
        else {
            Self::new()
        }
    }

    pub fn list_dates() -> Vec<Date> {
        let mut dates: Vec<Date> = Vec::new();
        let base_path = Self::get_base_db_path();
        for entry_res in std::fs::read_dir(base_path).unwrap() {
            let entry = entry_res.unwrap();
            let entry_name = entry.file_name();
            let w: Vec<&str> = entry_name.to_str().unwrap().split("-").collect();
            if w.len() == 3 {
                let year = u16::from_str_radix(w[0], 10).unwrap();
                let month = u8::from_str_radix(w[1], 10).unwrap();
                let day = u8::from_str_radix(w[2], 10).unwrap();
                dates.push(Date::new(year, month, day))
            }
        }
        dates
    }

    pub fn push(&mut self, entry: RecordedStepsDailyDatabaseEntry) {
        self.entries.retain(|old_entry| (old_entry.hour != entry.hour) || (old_entry.min != entry.min));
        self.entries.push(entry);
        self.entry_count = self.entries.len() as u32;
    }

    pub fn save(&self, date: Date) {
        let path = Self::get_db_path(date);
        println!("Saving recorded steps db: {:?}", path);
        let mut file = OpenOptions::new().create(true).write(true).open(path).unwrap();
        file.write_le(self).unwrap();
    }
}