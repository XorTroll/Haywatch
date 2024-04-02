use uuid::{Uuid, uuid};
use std::collections::BTreeSet;
use std::error::Error;
use binrw::{BinRead, BinReaderExt, BinWrite, BinWriterExt, io::Cursor};
use btleplug::api::{Peripheral as _, WriteType, Characteristic, CharPropFlags, ValueNotification};
use btleplug::platform::Peripheral;

pub const DEVICE_NAME: &str = "Haylou Smart Watch 2";

pub const SERVICE_GENERAL: Uuid = uuid!("000055FF-0000-1000-8000-00805F9B34FB");

pub const CHAR_GENERAL_RW_1: Characteristic = Characteristic {
    uuid: uuid!("000033F1-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_GENERAL,
    properties: /*CharPropFlags::READ |*/ CharPropFlags::WRITE,
    descriptors: BTreeSet::new()
};

pub const CHAR_GENERAL_N_1: Characteristic = Characteristic {
    uuid: uuid!("000033F2-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_GENERAL,
    properties: CharPropFlags::NOTIFY,
    descriptors: BTreeSet::new()
};

pub const CHAR_GENERAL_RW_2: Characteristic = Characteristic {
    uuid: uuid!("0000B003-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_GENERAL,
    properties: /*CharPropFlags::READ |*/ CharPropFlags::WRITE,
    descriptors: BTreeSet::new()
};

pub const CHAR_GENERAL_N_2: Characteristic = Characteristic {
    uuid: uuid!("0000B004-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_GENERAL,
    properties: CharPropFlags::NOTIFY,
    descriptors: BTreeSet::new()
};

pub const SERVICE_DATA1: Uuid = uuid!("000056FF-0000-1000-8000-00805F9B34FB");

pub const CHAR_DATA1_RW: Characteristic = Characteristic {
    uuid: uuid!("000034F1-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_DATA1,
    properties: /*CharPropFlags::READ |*/ CharPropFlags::WRITE_WITHOUT_RESPONSE,
    descriptors: BTreeSet::new()
};

pub const CHAR_DATA1_N: Characteristic = Characteristic {
    uuid: uuid!("000034F2-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_DATA1,
    properties: CharPropFlags::NOTIFY,
    descriptors: BTreeSet::new()
};

pub const SERVICE_DATA2: Uuid = uuid!("000060FF-0000-1000-8000-00805F9B34FB");

pub const CHAR_DATA2_RW: Characteristic = Characteristic {
    uuid: uuid!("00006001-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_DATA2,
    properties: /*CharPropFlags::READ |*/ CharPropFlags::WRITE_WITHOUT_RESPONSE,
    descriptors: BTreeSet::new()
};

pub const CHAR_DATA2_N: Characteristic = Characteristic {
    uuid: uuid!("00006002-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_DATA2,
    properties: CharPropFlags::NOTIFY,
    descriptors: BTreeSet::new()
};

pub const SERVICE_DATA3: Uuid = uuid!("000061FF-0000-1000-8000-00805F9B34FB");

pub const CHAR_DATA3_RW: Characteristic = Characteristic {
    uuid: uuid!("00006101-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_DATA3,
    properties: /*CharPropFlags::READ |*/ CharPropFlags::WRITE_WITHOUT_RESPONSE,
    descriptors: BTreeSet::new()
};

pub const CHAR_DATA3_N: Characteristic = Characteristic {
    uuid: uuid!("00006102-0000-1000-8000-00805F9B34FB"),
    service_uuid: SERVICE_DATA3,
    properties: CharPropFlags::NOTIFY,
    descriptors: BTreeSet::new()
};

pub const MAX_WRITE_SIZE: usize = 48;

pub trait CommandRequest: Sized + BinWrite where for<'a> <Self as BinWrite>::Args<'a>: Default {
    const WRITE_TYPE: WriteType;
    const CHAR: Characteristic;

    fn build(&self) -> Vec<u8> {
        let mut cmd: Vec<u8> = Vec::new();
        let mut cmd_writer = Cursor::new(&mut cmd);

        cmd_writer.write_le(self).unwrap();

        cmd
    }

    fn len(&self) -> usize {
        self.build().len()
    }
}

pub trait CommandResponse: Sized + BinRead where <Self as BinRead>::Args<'static>: Default {
    const CHAR: Characteristic;

    fn try_parse(raw: &Vec<u8>) -> Option<Self> {
        let mut cmd_reader = Cursor::new(raw);
        let res_opt = cmd_reader.read_le::<Self>().ok();
        if cmd_reader.read_le::<u8>().is_err() {
            res_opt
        }
        else {
            None
        }
    }
}

#[inline]
pub async fn write<C: CommandRequest>(watch: &Peripheral, cmd: C) -> Result<(), Box<dyn Error>> where for<'a> <C as BinWrite>::Args<'a>: Default {
    watch.write(&C::CHAR, &cmd.build(), C::WRITE_TYPE).await?;
    Ok(())
}

pub enum CommandResponseResult<C: CommandResponse> where <C as BinRead>::Args<'static>: Default {
    Invalid,
    ImproperCharacteristic,
    Ok(C)
}

#[inline]
pub async fn read<'a, C: CommandResponse>(value_notif: &ValueNotification, notif_char: &Characteristic) -> CommandResponseResult<C> where <C as BinRead>::Args<'static>: Default {
    match C::try_parse(&value_notif.value) {
        Some(res) => if C::CHAR.uuid.eq(&notif_char.uuid) { CommandResponseResult::Ok(res) } else { CommandResponseResult::ImproperCharacteristic },
        None => CommandResponseResult::Invalid
    }
}

pub mod common;

pub mod general;

pub mod data2;