use crate::error::AmsAddressError;
use crate::proto::proto_traits::{ReadFrom, WriteTo};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct AmsAddress {
    pub ams_net_id: AmsNetId,
    pub port: u16,
}

impl AmsAddress {
    pub fn new(ams_net_id: AmsNetId, port: u16) -> Self {
        AmsAddress { ams_net_id, port }
    }

    pub fn update_from_socket_addr(&mut self, socket_addr: &str) -> Result<(), AmsAddressError> {
        let ams_address = AmsAddress::from_str(socket_addr)?;
        self.ams_net_id = ams_address.ams_net_id;
        self.port = ams_address.port;
        Ok(())
    }
}

impl WriteTo for AmsAddress {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        self.ams_net_id.write_to(&mut wtr)?;
        wtr.write_u16::<LittleEndian>(self.port)?;
        Ok(())
    }
}

impl ReadFrom for AmsAddress {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(AmsAddress {
            ams_net_id: AmsNetId::read_from(read)?,
            port: read.read_u16::<LittleEndian>()?,
        })
    }
}

impl FromStr for AmsAddress {
    type Err = AmsAddressError;
    fn from_str(socket_addr: &str) -> Result<AmsAddress, AmsAddressError> {
        let split_socket: Vec<&str> = socket_addr.split(':').collect();
        if split_socket.len() != 2 {
            return Err(AmsAddressError::SplitError {
                length: split_socket.len(),
            });
        }

        let ams_net_id = AmsNetId::from_str(split_socket[0])?;

        let port = match split_socket[1].parse::<u16>() {
            Ok(p) => p,
            Err(e) => return Err(AmsAddressError::ParseError { source: e }),
        };
        Ok(AmsAddress::new(ams_net_id, port))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AmsNetId {
    net_id: [u8; 6],
}

impl WriteTo for AmsNetId {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_all(&self.net_id[..])?;
        Ok(())
    }
}

impl ReadFrom for AmsNetId {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let mut buffer: [u8; 6] = [0; 6];
        read.read_exact(&mut buffer)?;
        Ok(AmsNetId::from(buffer))
    }
}

impl From<[u8; 6]> for AmsNetId {
    fn from(net_id: [u8; 6]) -> AmsNetId {
        AmsNetId { net_id }
    }
}

impl FromStr for AmsNetId {
    type Err = AmsAddressError;
    fn from_str(net_id: &str) -> Result<AmsNetId, AmsAddressError> {
        let mut parts: Vec<&str> = net_id.split('.').collect();

        if parts.len() == 4 {
            parts.append(&mut vec!["1", "1"]);
        } else if parts.len() != 6 {
            return Err(AmsAddressError::InvalidAddressLength {
                length: parts.len(),
            });
        }

        let mut net_id = [0; 6];
        for (i, p) in parts.iter().enumerate() {
            match p.parse::<u8>() {
                Ok(v) => net_id[i] = v,
                Err(e) => return Err(AmsAddressError::ParseError { source: e }),
            }
        }
        Ok(AmsNetId { net_id })
    }
}

impl AmsNetId {
    #[allow(clippy::many_single_char_names)]
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> AmsNetId {
        AmsNetId {
            net_id: [a, b, c, d, e, f],
        }
    }

    pub fn net_id(&self) -> [u8; 6] {
        self.net_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ams_net_id_new_test() {
        let ams_net_id = AmsNetId::new(192, 168, 1, 1, 1, 1);
        assert_eq!(ams_net_id.net_id, [192, 168, 1, 1, 1, 1]);
    }

    #[test]
    fn ams_net_id_from_test() {
        let ams_net_id = AmsNetId::from([192, 168, 1, 1, 1, 1]);
        assert_eq!(ams_net_id.net_id, [192, 168, 1, 1, 1, 1]);
    }

    #[test]
    fn ams_net_id_parse_test() {
        let ams_net_id = AmsNetId::from_str("192.168.1.1.1.1").unwrap();
        assert_eq!(ams_net_id.net_id, [192, 168, 1, 1, 1, 1]);

        let ams_parse_error = AmsNetId::from_str("192.168.1.1.1.1.1").unwrap_err();
        assert_eq!(
            ams_parse_error,
            AmsAddressError::InvalidAddressLength { length: 7 }
        );
    }

    #[test]
    fn ams_net_id_write_to_test() {
        let ams_net_id = AmsNetId::from([192, 168, 1, 1, 1, 1]);
        let mut buffer: Vec<u8> = Vec::new();
        ams_net_id.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, [192, 168, 1, 1, 1, 1]);
    }

    #[test]
    fn ams_net_id_read_from_test() {
        let data: Vec<u8> = vec![192, 168, 1, 1, 1, 1, 99, 6, 33]; //Read only the first 6 bytes!
        let ams_net_id = AmsNetId::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(ams_net_id.net_id, [192, 168, 1, 1, 1, 1]);
    }

    #[test]
    fn ams_address_new_test() {
        let ams_net_id = AmsNetId::from_str("192.168.1.1.1.1").unwrap();
        let port = 30000;
        let ams_address = AmsAddress::new(ams_net_id.clone(), port);

        assert_eq!(ams_address.port, port);
        assert_eq!(ams_address.ams_net_id.net_id, ams_net_id.net_id);
    }

    #[test]
    fn ams_address_write_to_test() {
        let ams_net_id = AmsNetId::from_str("192.168.1.1.1.1").unwrap();
        let port = 30000;
        let ams_address = AmsAddress::new(ams_net_id.clone(), port);

        let mut buffer: Vec<u8> = Vec::new();
        ams_address.write_to(&mut buffer).unwrap();

        assert_eq!(buffer, [192, 168, 1, 1, 1, 1, 48, 117]);
    }

    #[test]
    fn ams_address_read_from_test() {
        let data: Vec<u8> = vec![192, 168, 1, 1, 1, 1, 48, 117];
        let ams_address = AmsAddress::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(ams_address.ams_net_id.net_id, [192, 168, 1, 1, 1, 1]);
        assert_eq!(ams_address.port, 30000);
    }

    #[test]
    fn ams_address_from_string_test() {
        let s = "169.0.0.1:45932";
        let ams_address = AmsAddress::from_str(s).unwrap();
        assert_eq!(ams_address.ams_net_id.net_id, [169, 0, 0, 1, 1, 1]);
        assert_eq!(ams_address.port, 45932);
    }
}
