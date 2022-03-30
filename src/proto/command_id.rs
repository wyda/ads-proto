use crate::proto::proto_traits::{ReadFrom, WriteTo};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CommandID {
    Invalid,
    ReadDeviceInfo,
    Read,
    Write,
    ReadState,
    WriteControl,
    AddDeviceNotification,
    DeleteDeviceNotification,
    DeviceNotification,
    ReadWrite,
}

impl WriteTo for CommandID {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u16::<LittleEndian>(*self as u16)?;
        Ok(())
    }
}

impl ReadFrom for CommandID {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(CommandID::from(read.read_u16::<LittleEndian>()?))
    }
}

impl From<u16> for CommandID {
    fn from(command_value: u16) -> Self {
        match command_value {
            0 => CommandID::Invalid,
            1 => CommandID::ReadDeviceInfo,
            2 => CommandID::Read,
            3 => CommandID::Write,
            4 => CommandID::ReadState,
            5 => CommandID::WriteControl,
            6 => CommandID::AddDeviceNotification,
            7 => CommandID::DeleteDeviceNotification,
            8 => CommandID::DeviceNotification,
            9 => CommandID::ReadWrite,
            _ => CommandID::Invalid,
        }
    }
}

impl CommandID {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn command_id_write_to_test() {
        let command = CommandID::AddDeviceNotification;
        let mut buffer: Vec<u8> = Vec::new();
        command.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, [6, 0]);
    }

    #[test]
    fn command_id_write_to2_test() {
        let mut buffer: Vec<u8> = Vec::new();
        CommandID::Read.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, [2, 0]);
    }

    #[test]
    fn command_id_read_from_test() {
        let data: Vec<u8> = vec![3, 0];
        let command_id = CommandID::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(command_id, CommandID::Write);
    }

    #[test]
    fn command_id_as_u16_test() {
        assert_eq!(CommandID::Invalid.as_u16(), 0);
        assert_eq!(CommandID::ReadDeviceInfo.as_u16(), 1);
        assert_eq!(CommandID::Read.as_u16(), 2);
        assert_eq!(CommandID::Write.as_u16(), 3);
        assert_eq!(CommandID::ReadState.as_u16(), 4);
        assert_eq!(CommandID::WriteControl.as_u16(), 5);
        assert_eq!(CommandID::AddDeviceNotification.as_u16(), 6);
        assert_eq!(CommandID::DeleteDeviceNotification.as_u16(), 7);
        assert_eq!(CommandID::DeviceNotification.as_u16(), 8);
        assert_eq!(CommandID::ReadWrite.as_u16(), 9);
    }

    #[test]
    fn command_id_from_test() {
        assert_eq!(CommandID::from(0), CommandID::Invalid);
        assert_eq!(CommandID::from(1), CommandID::ReadDeviceInfo);
        assert_eq!(CommandID::from(2), CommandID::Read);
        assert_eq!(CommandID::from(3), CommandID::Write);
        assert_eq!(CommandID::from(4), CommandID::ReadState);
        assert_eq!(CommandID::from(5), CommandID::WriteControl);
        assert_eq!(CommandID::from(6), CommandID::AddDeviceNotification);
        assert_eq!(CommandID::from(7), CommandID::DeleteDeviceNotification);
        assert_eq!(CommandID::from(8), CommandID::DeviceNotification);
        assert_eq!(CommandID::from(9), CommandID::ReadWrite);
        assert_eq!(CommandID::from(9654), CommandID::Invalid);
    }
}
