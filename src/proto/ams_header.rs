use crate::error::AdsError;
use crate::proto::ams_address::AmsAddress;
use crate::proto::command_id::CommandID;
use crate::proto::proto_traits::{Command, ReadFrom, WriteTo};
use crate::proto::request::*;
use crate::proto::response::*;
use crate::proto::state_flags::StateFlags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};
use std::mem::swap;

///Length of the fix part of the AMS Header in bytes
const FIX_AMS_HEADER_LEN: u32 = 32;

#[derive(Debug, Clone)]
pub struct AmsTcpHeader {
    reserved: [u8; 2],
    pub length: u32,
    pub ams_header: AmsHeader,
}

impl WriteTo for AmsTcpHeader {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_all(&self.reserved)?;
        wtr.write_u32::<LittleEndian>(self.ams_header.header_len())?;
        self.ams_header.write_to(&mut wtr)?;
        Ok(())
    }
}

impl ReadFrom for AmsTcpHeader {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let reserved = read.read_u16::<LittleEndian>()?.to_le_bytes();
        Ok(AmsTcpHeader {
            reserved,
            length: read.read_u32::<LittleEndian>()?,
            ams_header: AmsHeader::read_from(read)?,
        })
    }
}

impl From<AmsHeader> for AmsTcpHeader {
    fn from(ams_header: AmsHeader) -> Self {
        AmsTcpHeader {
            reserved: [0, 0],
            length: ams_header.header_len(),
            ams_header,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AmsHeader {
    ams_address_targed: AmsAddress,
    ams_address_source: AmsAddress,
    command_id: CommandID,
    state_flags: StateFlags,
    length: u32,
    ams_ads_error: AdsError,
    invoke_id: u32,
    data: Vec<u8>,
}

impl WriteTo for AmsHeader {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        self.ams_address_targed.write_to(&mut wtr)?;
        self.ams_address_source.write_to(&mut wtr)?;
        self.command_id.write_to(&mut wtr)?;
        self.state_flags.write_to(&mut wtr)?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        wtr.write_u32::<LittleEndian>(self.ams_ads_error.as_u32())?;
        wtr.write_u32::<LittleEndian>(self.invoke_id)?;
        wtr.write_all(&self.data)?;
        Ok(())
    }
}

impl ReadFrom for AmsHeader {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let ams_address_targed = AmsAddress::read_from(read)?;
        let ams_address_source = AmsAddress::read_from(read)?;
        let command_id = CommandID::read_from(read)?;
        let state_flags = StateFlags::read_from(read)?;
        let length = read.read_u32::<LittleEndian>()?;
        let ams_ads_error = AdsError::from(read.read_u32::<LittleEndian>()?);
        let invoke_id = read.read_u32::<LittleEndian>()?;
        let mut data: Vec<u8> = vec![0; length as usize];
        read.read_exact(&mut data)?;

        Ok(AmsHeader {
            ams_address_targed,
            ams_address_source,
            command_id,
            state_flags,
            length,
            ams_ads_error,
            invoke_id,
            data,
        })
    }
}

impl AmsHeader {
    pub fn new(
        ams_address_targed: AmsAddress,
        ams_address_source: AmsAddress,
        state_flags: StateFlags,
        invoke_id: u32,
        command: impl Command + WriteTo,
    ) -> Self {
        let mut data: Vec<u8> = Vec::with_capacity(16);
        command
            .write_to(&mut data)
            .expect("failed to write request to buffer!");

        AmsHeader {
            ams_address_targed,
            ams_address_source,
            command_id: command.command_id(),
            state_flags,
            length: data.len() as u32,
            ams_ads_error: AdsError::ErrNoError,
            invoke_id,
            data,
        }
    }

    ///Returns the response from AMS header data
    pub fn response(&mut self) -> io::Result<Response> {
        if self.state_flags.is_response() || self.command_id == CommandID::DeviceNotification {
            match self.command_id {
                CommandID::Invalid => Err(io::Error::new(
                    io::ErrorKind::Other,
                    AdsError::AdsErrDeviceInvalidData,
                )),
                CommandID::ReadDeviceInfo => Ok(Response::ReadDeviceInfo(
                    ReadDeviceInfoResponse::read_from(&mut self.data.as_slice())?,
                )),
                CommandID::Read => Ok(Response::Read(ReadResponse::read_from(
                    &mut self.data.as_slice(),
                )?)),
                CommandID::Write => Ok(Response::Write(WriteResponse::read_from(
                    &mut self.data.as_slice(),
                )?)),
                CommandID::ReadState => Ok(Response::ReadState(ReadStateResponse::read_from(
                    &mut self.data.as_slice(),
                )?)),
                CommandID::WriteControl => Ok(Response::WriteControl(
                    WriteControlResponse::read_from(&mut self.data.as_slice())?,
                )),
                CommandID::AddDeviceNotification => Ok(Response::AddDeviceNotification(
                    AddDeviceNotificationResponse::read_from(&mut self.data.as_slice())?,
                )),
                CommandID::DeleteDeviceNotification => Ok(Response::DeleteDeviceNotification(
                    DeleteDeviceNotificationResponse::read_from(&mut self.data.as_slice())?,
                )),
                CommandID::DeviceNotification => Ok(Response::DeviceNotification(
                    AdsNotificationStream::read_from(&mut self.data.as_slice())?,
                )),
                CommandID::ReadWrite => Ok(Response::ReadWrite(ReadWriteResponse::read_from(
                    &mut self.data.as_slice(),
                )?)),
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Is not a response but a request!",
            ))
        }
    }

    ///Returns the request from AMS header data
    pub fn request(&mut self) -> io::Result<Request> {
        if !self.state_flags.is_response() {
            match self.command_id {
                CommandID::Invalid => Err(io::Error::new(
                    io::ErrorKind::Other,
                    AdsError::AdsErrDeviceInvalidData,
                )),
                CommandID::ReadDeviceInfo => {
                    Ok(Request::ReadDeviceInfo(ReadDeviceInfoRequest::new()))
                }
                CommandID::Read => Ok(Request::Read(ReadRequest::read_from(
                    &mut self.data.as_slice(),
                )?)),
                CommandID::Write => Ok(Request::Write(WriteRequest::read_from(
                    &mut self.data.as_slice(),
                )?)),
                CommandID::ReadState => Ok(Request::ReadState(ReadStateRequest::new())),
                CommandID::WriteControl => Ok(Request::WriteControl(
                    WriteControlRequest::read_from(&mut self.data.as_slice())?,
                )),
                CommandID::AddDeviceNotification => Ok(Request::AddDeviceNotification(
                    AddDeviceNotificationRequest::read_from(&mut self.data.as_slice())?,
                )),
                CommandID::DeleteDeviceNotification => Ok(Request::DeleteDeviceNotification(
                    DeleteDeviceNotificationRequest::read_from(&mut self.data.as_slice())?,
                )),
                CommandID::DeviceNotification => {
                    Ok(Request::DeviceNotification(DeviceNotificationRequest::new()))
                }
                CommandID::ReadWrite => Ok(Request::ReadWrite(ReadWriteRequest::read_from(
                    &mut self.data.as_slice(),
                )?)),
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Is not a request but a response!",
            ))
        }
    }

    ///Returns the command id from the ams header
    pub fn command_id(&self) -> CommandID {
        self.command_id
    }

    ///Returns the invoke id from the ams header. This is the invoke id set when requested the data
    pub fn invoke_id(&self) -> u32 {
        self.invoke_id
    }

    ///Returns the ads error code from the ams header. There is another ads error in the response data!
    pub fn ads_error(&self) -> &AdsError {
        &self.ams_ads_error
    }

    ///Return the raw data from the ams header data section. This data can be used to create the specific response object.
    pub fn raw_response_data(&self) -> &[u8] {
        &self.data[..]
    }

    ///Get the ads error code from the response data (ams header data section)
    pub fn response_result(&self) -> Option<AdsError> {
        if self.data.len() >= 4 {
            if let Ok(result) = self.data.as_slice().read_u32::<LittleEndian>() {
                return Some(AdsError::from(result));
            }
        }
        None
    }

    ///get the length in bytes of the whole ams_header.
    fn header_len(&self) -> u32 {
        self.data.len() as u32 + FIX_AMS_HEADER_LEN
    }

    ///get data length in bytes of the ams header data section
    pub fn data_len(&self) -> u32 {
        self.length
    }

    /// update command id, payload data and state flags.
    /// for example when changing the AmsHeader from request to response instead of cerating a new one
    /// see also swap_address()
    pub fn update_command(
        &mut self,
        command: impl Command + WriteTo,
        state_flag: StateFlags,
    ) -> io::Result<()> {
        self.command_id = command.command_id();
        self.data.clear();
        command.write_to(&mut self.data)?;
        self.length = self.data.len() as u32;
        self.state_flags = state_flag;
        Ok(())
    }

    /// update ams header data
    /// updates the payload data and the data length info in the ams header
    pub fn update_raw_data(&mut self, buffer: Vec<u8>) -> io::Result<()> {
        self.data = buffer;
        self.length = self.data.len() as u32;
        Ok(())
    }

    /// Swap targed and source AmsAddress
    pub fn swap_address(&mut self) {
        swap(&mut self.ams_address_source, &mut self.ams_address_targed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::ams_address::*;
    use std::str::FromStr;
    #[test]
    fn ams_header_write_to_test() {
        let mut buffer: Vec<u8> = Vec::new();

        let port = 30000;

        let ams_header = AmsHeader::new(
            AmsAddress::new(AmsNetId::from_str("192.168.1.1.1.1").unwrap(), port),
            AmsAddress::new(AmsNetId::new(192, 168, 1, 1, 1, 2), port),
            StateFlags::resp_default(),
            111,
            Request::Read(ReadRequest::new(259, 259, 4)),
        );

        ams_header.write_to(&mut buffer).unwrap();

        #[rustfmt::skip]
        let compare: Vec<u8> = vec![
            //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117,
            //CommandID -> Read 
            2, 0,                               
            //state flag -> Request, Ads command, TCP (4)
            5, 0,                               
            //Lennth of data for read request (12 byte)
            12, 0, 0, 0,                        
            //Error code -> No error 
            0, 0, 0, 0,                         
            //Invoke ID -> 111 
            111, 0, 0, 0,                       
            //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0  
        ];
        assert_eq!(compare, buffer);
    }

    #[test]
    fn ams_header_read_from_test() {
        #[rustfmt::skip]
        let data: Vec<u8> = vec![
            //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117,      
            //CommandID -> Read 
            2, 0,                               
            //state flag -> Request, Ads command, TCP (4)
            4, 0,                               
            //Lennth of data for read request (12 byte)
            12, 0, 0, 0,                        
            //Error code -> No error 
            0, 0, 0, 0,                         
            //Invoke ID -> 111 
            111, 0, 0, 0,                       
            //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0  
        ];

        let ams_header = AmsHeader::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(
            ams_header.ams_address_targed.ams_net_id.net_id(),
            [192, 168, 1, 1, 1, 1]
        );
        assert_eq!(
            ams_header.ams_address_source.ams_net_id.net_id(),
            [192, 168, 1, 1, 1, 2]
        );
        assert_eq!(ams_header.ams_address_targed.port, 30000);
        assert_eq!(ams_header.ams_address_source.port, 30000);
        assert_eq!(ams_header.command_id, CommandID::Read);
        assert_eq!(ams_header.state_flags.value(), 4);
        assert_eq!(ams_header.length, 12, "Wrong data length");
        assert_eq!(ams_header.ams_ads_error, AdsError::ErrNoError);
        assert_eq!(ams_header.invoke_id, 111);
        assert_eq!(ams_header.data, [3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0]);
    }

    #[test]
    fn ams_header_len_test() {
        let port = 30000;
        let ams_header = AmsHeader::new(
            AmsAddress::new(AmsNetId::from_str("192.168.1.1.1.1").unwrap(), port),
            AmsAddress::new(AmsNetId::new(192, 168, 1, 1, 1, 2), port),
            StateFlags::req_default(),
            111,
            Request::Read(ReadRequest::new(259, 259, 4)),
        );

        assert_eq!(ams_header.header_len(), 44);
    }

    #[test]
    fn ams_header_swap_address() {
        let port = 30000;
        let mut ams_header = AmsHeader::new(
            AmsAddress::new(AmsNetId::from_str("192.168.1.1.1.1").unwrap(), port),
            AmsAddress::new(AmsNetId::new(192, 168, 1, 1, 1, 2), port),
            StateFlags::req_default(),
            111,
            Request::Read(ReadRequest::new(259, 259, 4)),
        );

        //before swap
        assert_eq!(
            ams_header.ams_address_targed,
            AmsAddress::new(AmsNetId::from_str("192.168.1.1.1.1").unwrap(), port)
        );
        assert_eq!(
            ams_header.ams_address_source,
            AmsAddress::new(AmsNetId::new(192, 168, 1, 1, 1, 2), port)
        );

        ams_header.swap_address();

        //After swap
        assert_eq!(
            ams_header.ams_address_targed,
            AmsAddress::new(AmsNetId::new(192, 168, 1, 1, 1, 2), port)
        );
        assert_eq!(
            ams_header.ams_address_source,
            AmsAddress::new(AmsNetId::from_str("192.168.1.1.1.1").unwrap(), port)
        );
    }

    #[test]
    fn ams_tcp_header_write_to_test() {
        let mut buffer: Vec<u8> = Vec::new();

        let port = 30000;

        let ams_header = AmsHeader::new(
            AmsAddress::new(AmsNetId::from_str("192.168.1.1.1.1").unwrap(), port),
            AmsAddress::new(AmsNetId::new(192, 168, 1, 1, 1, 2), port),
            StateFlags::req_default(),
            111,
            Request::Read(ReadRequest::new(259, 259, 4)),
        );

        let ams_tcp_header = AmsTcpHeader::from(ams_header);
        ams_tcp_header.write_to(&mut buffer).unwrap();

        #[rustfmt::skip]
        let compare: Vec<u8> = vec![
            //Reserved has to be 0
            0,0,
            //Length in bytes of AmsHeader
            44, 0, 0, 0,
            //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,      
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117,      
            //CommandID -> Read 
            2, 0,                               
            //state flag -> Request, Ads command, TCP (4)
            4, 0,                               
            //Lennth of data for read request (12 byte)
            12, 0, 0, 0,                        
            //Error code -> No error 
            0, 0, 0, 0,                         
            //Invoke ID -> 111 
            111, 0, 0, 0,                       
            //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0  
        ];
        assert_eq!(compare, buffer);
    }

    #[test]
    fn ams_tcp_header_read_from_test() {
        #[rustfmt::skip]
        let data: Vec<u8> = vec![
          //Reserved has to be 0
          0,0,
          //Length in bytes of AmsHeader
          44, 0, 0, 0,
          //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
          192, 168, 1, 1, 1, 1, 48, 117,      
          //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
          192, 168, 1, 1, 1, 2, 48, 117,      
          //CommandID -> Read 
          2, 0,                               
          //state flag -> Request, Ads command, TCP (4)
          4, 0,                               
          //Lennth of data for read request (12 byte)
          12, 0, 0, 0,                        
          //Error code -> No error 
          0, 0, 0, 0,                         
          //Invoke ID -> 111 
          111, 0, 0, 0,                       
          //Data from read request -> see request.rs
          3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0  
        ];

        let ams_tcp_header = AmsTcpHeader::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(ams_tcp_header.reserved, [0, 0]);
        assert_eq!(ams_tcp_header.length, 44);
        assert_eq!(
            ams_tcp_header
                .ams_header
                .ams_address_targed
                .ams_net_id
                .net_id(),
            [192, 168, 1, 1, 1, 1]
        );
        assert_eq!(
            ams_tcp_header
                .ams_header
                .ams_address_source
                .ams_net_id
                .net_id(),
            [192, 168, 1, 1, 1, 2]
        );
        assert_eq!(ams_tcp_header.ams_header.ams_address_targed.port, 30000);
        assert_eq!(ams_tcp_header.ams_header.ams_address_source.port, 30000);
        assert_eq!(ams_tcp_header.ams_header.command_id, CommandID::Read);
        assert_eq!(ams_tcp_header.ams_header.state_flags.value(), 4);
        assert_eq!(ams_tcp_header.ams_header.length, 12);
        assert_eq!(
            ams_tcp_header.ams_header.ams_ads_error,
            AdsError::ErrNoError
        );
        assert_eq!(ams_tcp_header.ams_header.invoke_id, 111);
        assert_eq!(
            ams_tcp_header.ams_header.data,
            [3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0]
        );
    }

    #[test]
    fn ams_tcp_header_command_id() {
        let data: Vec<u8> = vec![
            //Reserved has to be 0
            0, 0, //Length in bytes of AmsHeader
            44, 0, 0, 0, //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117, //CommandID -> Read
            2, 0, //state flag -> Request, Ads command, TCP (4)
            4, 0, //Lennth of data for read request (12 byte)
            12, 0, 0, 0, //Error code -> No error
            0, 0, 0, 0, //Invoke ID -> 111
            111, 0, 0, 0, //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0,
        ];

        let ams_tcp_header = AmsTcpHeader::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(CommandID::Read, ams_tcp_header.ams_header.command_id());
    }

    #[test]
    fn ams_tcp_header_response_data_length() {
        let data: Vec<u8> = vec![
            //Reserved has to be 0
            0, 0, //Length in bytes of AmsHeader
            44, 0, 0, 0, //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117, //CommandID -> Read
            2, 0, //state flag -> Request, Ads command, TCP (4)
            4, 0, //Lennth of data for read request (12 byte)
            12, 0, 0, 0, //Error code -> No error
            0, 0, 0, 0, //Invoke ID -> 111
            111, 0, 0, 0, //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0,
        ];

        let ams_tcp_header = AmsTcpHeader::read_from(&mut data.as_slice()).unwrap();
        let len = ams_tcp_header.ams_header.data_len();
        assert_eq!(12, len);
    }

    #[test]
    fn ams_tcp_header_update_command() {
        let data: Vec<u8> = vec![
            //Reserved has to be 0
            0, 0, //Length in bytes of AmsHeader
            44, 0, 0, 0, //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117, //CommandID -> Read
            2, 0, //state flag -> Request, Ads command, TCP (4)
            4, 0, //Lennth of data for read request (12 byte)
            12, 0, 0, 0, //Error code -> No error
            0, 0, 0, 0, //Invoke ID -> 111
            111, 0, 0, 0, //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0,
        ];

        let mut ams_tcp_header = AmsTcpHeader::read_from(&mut data.as_slice()).unwrap();
        let request = Request::ReadDeviceInfo(ReadDeviceInfoRequest::new());
        let mut new_data: Vec<u8> = Vec::new();
        request.write_to(&mut new_data).unwrap();
        ams_tcp_header
            .ams_header
            .update_command(request, StateFlags::req_default())
            .unwrap();
        assert_eq!(new_data, ams_tcp_header.ams_header.raw_response_data());
        assert_eq!(
            ams_tcp_header.ams_header.command_id(),
            CommandID::ReadDeviceInfo
        );
    }

    #[test]
    fn ams_tcp_header_update_data() {
        let data: Vec<u8> = vec![
            //Reserved has to be 0
            0, 0, //Length in bytes of AmsHeader
            44, 0, 0, 0, //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117, //CommandID -> Read
            2, 0, //state flag -> Request, Ads command, TCP (4)
            4, 0, //Lennth of data for read request (12 byte)
            12, 0, 0, 0, //Error code -> No error
            0, 0, 0, 0, //Invoke ID -> 111
            111, 0, 0, 0, //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0,
        ];

        let mut ams_tcp_header = AmsTcpHeader::read_from(&mut data.as_slice()).unwrap();
        let new_data: Vec<u8> = vec![111, 0, 0, 0, 3, 1, 0, 0, 1, 3, 0, 0, 8, 0]; //2 byte shorter
        ams_tcp_header
            .ams_header
            .update_raw_data(new_data.clone())
            .unwrap();
        assert_eq!(new_data, ams_tcp_header.ams_header.raw_response_data());
        assert_eq!(
            new_data.len(),
            ams_tcp_header.ams_header.raw_response_data().len()
        );
        assert_eq!(new_data.len(), 14);
    }

    #[test]
    fn ams_tcp_header_invoke_id() {
        let data: Vec<u8> = vec![
            //Reserved has to be 0
            0, 0, //Length in bytes of AmsHeader
            44, 0, 0, 0, //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117, //CommandID -> Read
            2, 0, //state flag -> Request, Ads command, TCP (4)
            4, 0, //Lennth of data for read request (12 byte)
            12, 0, 0, 0, //Error code -> No error
            0, 0, 0, 0, //Invoke ID -> 111
            111, 0, 0, 0, //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0,
        ];

        let ams_tcp_header = AmsTcpHeader::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(111, ams_tcp_header.ams_header.invoke_id());
    }

    #[test]
    fn ams_tcp_header_ads_error() {
        let data: Vec<u8> = vec![
            //Reserved has to be 0
            0, 0, //Length in bytes of AmsHeader
            44, 0, 0, 0, //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117, //CommandID -> Read
            2, 0, //state flag -> Request, Ads command, TCP (4)
            4, 0, //Lennth of data for read request (12 byte)
            12, 0, 0, 0, //Error code -> No error
            0, 0, 0, 0, //Invoke ID -> 111
            111, 0, 0, 0, //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0,
        ];

        let ams_tcp_header = AmsTcpHeader::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(&AdsError::ErrNoError, ams_tcp_header.ams_header.ads_error());
    }

    #[test]
    fn ams_tcp_header_raw_response_data() {
        let data: Vec<u8> = vec![
            //Reserved has to be 0
            0, 0, //Length in bytes of AmsHeader
            44, 0, 0, 0, //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117, //CommandID -> Read
            2, 0, //state flag -> Request, Ads command, TCP (4)
            4, 0, //Lennth of data for read request (12 byte)
            12, 0, 0, 0, //Error code -> No error
            0, 0, 0, 0, //Invoke ID -> 111
            111, 0, 0, 0, //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0,
        ];

        let ams_tcp_header = AmsTcpHeader::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(
            &[3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0],
            ams_tcp_header.ams_header.raw_response_data()
        );
    }

    #[test]
    fn ams_tcp_header_responser_result() {
        let data: Vec<u8> = vec![
            //Reserved has to be 0
            0, 0, //Length in bytes of AmsHeader
            44, 0, 0, 0, //target AmsAddress -> NetId/port (192.168.1.1.1.1, 30000)
            192, 168, 1, 1, 1, 1, 48, 117,
            //Source AmsAddress -> NetId/port (192.168.1.1.1.2, 30000)
            192, 168, 1, 1, 1, 2, 48, 117, //CommandID -> Read
            2, 0, //state flag -> Request, Ads command, TCP (4)
            4, 0, //Lennth of data for read request (12 byte)
            12, 0, 0, 0, //Error code -> No error
            1, 0, 0, 0, //Invoke ID -> 111
            111, 0, 0, 0, //Data from read request -> see request.rs
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0,
        ];

        let ams_tcp_header = AmsTcpHeader::read_from(&mut data.as_slice()).unwrap();
        assert_eq!(
            Some(AdsError::ErrUnknowAdsError { error_code: 259 }),
            ams_tcp_header.ams_header.response_result()
        );
    }
}
