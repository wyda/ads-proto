use crate::proto::proto_traits::{ReadFrom, WriteTo};
use bitfield::Bit;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[derive(Debug)]
pub enum NetProto {
    Tcp,
    Udp,
}

#[derive(Debug, Clone)]
pub struct StateFlags {
    value: u16,
}

impl StateFlags {
    pub fn new(response: bool, ads_command: bool, net_proto: NetProto) -> Self {
        let mut state_flags: u16 = 0;
        state_flags.set_bit(0, response);
        state_flags.set_bit(2, ads_command);

        state_flags.set_bit(
            6,
            match net_proto {
                NetProto::Tcp => false,
                NetProto::Udp => true,
            },
        );

        StateFlags { value: state_flags }
    }

    ///default for response (response=true, ads_command=true, net_proto=Tcp)
    pub fn resp_default() -> Self {
        StateFlags::new(true, true, NetProto::Tcp)
    }

    ///default for response (response=false, ads_command=true, net_proto=Tcp)
    pub fn req_default() -> Self {
        StateFlags::new(false, true, NetProto::Tcp)
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn is_tcp(&self) -> bool {
        !self.value.bit(6)
    }

    pub fn is_response(&self) -> bool {
        self.value.bit(0)
    }

    pub fn is_ads_command(&self) -> bool {
        self.value.bit(2)
    }
}

///from u16 may results in an invalid state flag
impl From<u16> for StateFlags {
    fn from(value: u16) -> StateFlags {
        StateFlags { value }
    }
}

impl WriteTo for StateFlags {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u16::<LittleEndian>(self.value())?;
        Ok(())
    }
}

impl ReadFrom for StateFlags {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(StateFlags::from(read.read_u16::<LittleEndian>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let state_flags = StateFlags::new(false, true, NetProto::Tcp);
        assert_eq!(state_flags.value, 4);

        let state_flags = StateFlags::new(true, true, NetProto::Tcp);
        assert_eq!(state_flags.value, 5);

        let state_flags = StateFlags::new(false, false, NetProto::Tcp);
        assert_eq!(state_flags.value, 0);

        let state_flags = StateFlags::new(true, false, NetProto::Tcp);
        assert_eq!(state_flags.value, 1);

        let state_flags = StateFlags::new(false, true, NetProto::Udp);
        assert_eq!(state_flags.value, 68);

        let state_flags = StateFlags::new(true, true, NetProto::Udp);
        assert_eq!(state_flags.value, 69);

        let state_flags = StateFlags::new(false, false, NetProto::Udp);
        assert_eq!(state_flags.value, 64);

        let state_flags = StateFlags::new(true, false, NetProto::Udp);
        assert_eq!(state_flags.value, 65);
    }

    #[test]
    fn resp_default_test() {
        let state_flags = StateFlags::resp_default();
        assert_eq!(state_flags.value, 5);
    }

    #[test]
    fn req_default_test() {
        let state_flags = StateFlags::req_default();
        assert_eq!(state_flags.value, 4);
    }

    #[test]
    fn write_to_test() {
        let state_flags = StateFlags::resp_default();
        let mut buffer: Vec<u8> = Vec::with_capacity(16);
        state_flags.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, [5, 0]);

        let state_flags = StateFlags::new(true, true, NetProto::Udp);
        let mut buffer: Vec<u8> = Vec::with_capacity(16);
        state_flags.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, [69, 0]);
    }

    #[test]
    fn read_from_test() {
        let buffer: Vec<u8> = vec![5, 0, 1, 99];
        let state_flags = StateFlags::read_from(&mut buffer.as_slice()).unwrap();

        assert_eq!(state_flags.value(), 5);
    }

    #[test]
    fn is_tcp_test() {
        let buffer: Vec<u8> = vec![5, 0, 1, 99, 4];
        let state_flags = StateFlags::read_from(&mut buffer.as_slice()).unwrap();

        assert_eq!(state_flags.is_tcp(), true);
    }

    #[test]
    fn is_udp_test() {
        let buffer: Vec<u8> = vec![69, 0, 1, 99, 4];
        let state_flags = StateFlags::read_from(&mut buffer.as_slice()).unwrap();

        assert_eq!(state_flags.is_tcp(), false);
    }

    #[test]
    fn is_response_test() {
        let buffer: Vec<u8> = vec![5, 0, 1, 99, 4];
        let state_flags = StateFlags::read_from(&mut buffer.as_slice()).unwrap();

        assert_eq!(state_flags.is_response(), true);
    }

    #[test]
    fn is_ads_command_test() {
        let buffer: Vec<u8> = vec![4, 0, 1, 99, 4];
        let state_flags = StateFlags::read_from(&mut buffer.as_slice()).unwrap();

        assert_eq!(state_flags.is_ads_command(), true);
        assert_eq!(state_flags.is_response(), false);
    }
}
