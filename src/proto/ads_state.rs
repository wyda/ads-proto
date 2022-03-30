use crate::proto::proto_traits::{ReadFrom, WriteTo};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AdsState {
    AdsStateInvalid,
    AdsStateIdle,
    AdsStateReset,
    AdsStateInit,
    AdsStateStart,
    AdsStateRun,
    AdsStateStop,
    AdsStateSaveCFG,
    AdsStateLoadCFG,
    AdsStatePowerFailure,
    AdsStatePowerGood,
    AdsStateError,
    AdsStateShutDown,
    AdsStateSuspend,
    AdsStateResume,
    AdsStateConfig,
    AdsStateReconfig,
}

impl From<u16> for AdsState {
    fn from(state_value: u16) -> Self {
        match state_value {
            0 => AdsState::AdsStateInvalid,
            1 => AdsState::AdsStateIdle,
            2 => AdsState::AdsStateReset,
            3 => AdsState::AdsStateInit,
            4 => AdsState::AdsStateStart,
            5 => AdsState::AdsStateRun,
            6 => AdsState::AdsStateStop,
            7 => AdsState::AdsStateSaveCFG,
            8 => AdsState::AdsStateLoadCFG,
            9 => AdsState::AdsStatePowerFailure,
            10 => AdsState::AdsStatePowerGood,
            11 => AdsState::AdsStateError,
            12 => AdsState::AdsStateShutDown,
            13 => AdsState::AdsStateSuspend,
            14 => AdsState::AdsStateResume,
            15 => AdsState::AdsStateConfig,
            16 => AdsState::AdsStateReconfig,
            _ => AdsState::AdsStateInvalid,
        }
    }
}

impl WriteTo for AdsState {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u16::<LittleEndian>(*self as u16)?;
        Ok(())
    }
}

impl ReadFrom for AdsState {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(AdsState::from(read.read_u16::<LittleEndian>()?))
    }
}

impl AdsState {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ads_state_write_to_test() {
        let compare: [u8; 2] = [5, 0];
        let mut buffer: Vec<u8> = Vec::new();
        AdsState::AdsStateRun.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, compare);
    }

    #[test]
    fn ads_state_read_from_test() {
        let data: Vec<u8> = vec![6, 0, 8, 4];
        let trans_mode = AdsState::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(trans_mode, AdsState::AdsStateStop);
    }

    #[test]
    fn ads_state_from_test() {
        assert_eq!(AdsState::from(0), AdsState::AdsStateInvalid);
        assert_eq!(AdsState::from(1), AdsState::AdsStateIdle);
        assert_eq!(AdsState::from(2), AdsState::AdsStateReset);
        assert_eq!(AdsState::from(3), AdsState::AdsStateInit);
        assert_eq!(AdsState::from(4), AdsState::AdsStateStart);
        assert_eq!(AdsState::from(5), AdsState::AdsStateRun);
        assert_eq!(AdsState::from(6), AdsState::AdsStateStop);
        assert_eq!(AdsState::from(7), AdsState::AdsStateSaveCFG);
        assert_eq!(AdsState::from(8), AdsState::AdsStateLoadCFG);
        assert_eq!(AdsState::from(9), AdsState::AdsStatePowerFailure);
        assert_eq!(AdsState::from(10), AdsState::AdsStatePowerGood);
        assert_eq!(AdsState::from(11), AdsState::AdsStateError);
        assert_eq!(AdsState::from(12), AdsState::AdsStateShutDown);
        assert_eq!(AdsState::from(13), AdsState::AdsStateSuspend);
        assert_eq!(AdsState::from(14), AdsState::AdsStateResume);
        assert_eq!(AdsState::from(15), AdsState::AdsStateConfig);
        assert_eq!(AdsState::from(16), AdsState::AdsStateReconfig);
        assert_eq!(AdsState::from(999), AdsState::AdsStateInvalid);
    }

    #[test]
    fn ads_state_as_u16_test() {
        assert_eq!(AdsState::AdsStateInvalid.as_u16(), 0);
        assert_eq!(AdsState::AdsStateIdle.as_u16(), 1);
        assert_eq!(AdsState::AdsStateReset.as_u16(), 2);
        assert_eq!(AdsState::AdsStateInit.as_u16(), 3);
        assert_eq!(AdsState::AdsStateStart.as_u16(), 4);
        assert_eq!(AdsState::AdsStateRun.as_u16(), 5);
        assert_eq!(AdsState::AdsStateStop.as_u16(), 6);
        assert_eq!(AdsState::AdsStateSaveCFG.as_u16(), 7);
        assert_eq!(AdsState::AdsStateLoadCFG.as_u16(), 8);
        assert_eq!(AdsState::AdsStatePowerFailure.as_u16(), 9);
        assert_eq!(AdsState::AdsStatePowerGood.as_u16(), 10);
        assert_eq!(AdsState::AdsStateError.as_u16(), 11);
        assert_eq!(AdsState::AdsStateShutDown.as_u16(), 12);
        assert_eq!(AdsState::AdsStateSuspend.as_u16(), 13);
        assert_eq!(AdsState::AdsStateResume.as_u16(), 14);
        assert_eq!(AdsState::AdsStateConfig.as_u16(), 15);
        assert_eq!(AdsState::AdsStateReconfig.as_u16(), 16);
    }
}
