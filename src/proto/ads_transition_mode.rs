use crate::proto::proto_traits::{ReadFrom, WriteTo};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AdsTransMode {
    None,
    ClientCylcle,
    ClientOnChange,
    Cyclic,
    OnChange,
    CyclicInContext,
    OnChangeInContext,
}

impl From<u32> for AdsTransMode {
    fn from(state_value: u32) -> Self {
        match state_value {
            0 => AdsTransMode::None,
            1 => AdsTransMode::ClientCylcle,
            2 => AdsTransMode::ClientOnChange,
            3 => AdsTransMode::Cyclic,
            4 => AdsTransMode::OnChange,
            5 => AdsTransMode::CyclicInContext,
            6 => AdsTransMode::OnChangeInContext,
            _ => AdsTransMode::None,
        }
    }
}

impl WriteTo for AdsTransMode {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.as_u32())?;
        Ok(())
    }
}

impl ReadFrom for AdsTransMode {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(AdsTransMode::from(read.read_u32::<LittleEndian>()?))
    }
}

impl AdsTransMode {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ads_trans_mode_from_test() {
        assert_eq!(AdsTransMode::from(0), AdsTransMode::None);
        assert_eq!(AdsTransMode::from(1), AdsTransMode::ClientCylcle);
        assert_eq!(AdsTransMode::from(2), AdsTransMode::ClientOnChange);
        assert_eq!(AdsTransMode::from(3), AdsTransMode::Cyclic);
        assert_eq!(AdsTransMode::from(4), AdsTransMode::OnChange);
        assert_eq!(AdsTransMode::from(5), AdsTransMode::CyclicInContext);
        assert_eq!(AdsTransMode::from(6), AdsTransMode::OnChangeInContext);
        assert_eq!(AdsTransMode::from(98765), AdsTransMode::None);
    }

    #[test]
    fn ads_trans_mode_as_u32_test() {
        assert_eq!(0, AdsTransMode::None.as_u32());
        assert_eq!(1, AdsTransMode::ClientCylcle.as_u32());
        assert_eq!(2, AdsTransMode::ClientOnChange.as_u32());
        assert_eq!(3, AdsTransMode::Cyclic.as_u32());
        assert_eq!(4, AdsTransMode::OnChange.as_u32());
        assert_eq!(5, AdsTransMode::CyclicInContext.as_u32());
        assert_eq!(6, AdsTransMode::OnChangeInContext.as_u32());
    }

    #[test]
    fn ads_trans_mode_write_to_test() {
        let compare: [u8; 4] = [3, 0, 0, 0];
        let mut buffer: Vec<u8> = Vec::new();
        AdsTransMode::Cyclic.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, compare);
    }

    #[test]
    fn ads_trans_mode_read_from_test() {
        let data: Vec<u8> = vec![3, 0, 0, 0];
        let trans_mode = AdsTransMode::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(trans_mode, AdsTransMode::Cyclic);
    }
}
