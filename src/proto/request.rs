use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

use crate::error::TryIntoError;
use crate::proto::ads_state::AdsState;
use crate::proto::ads_transition_mode::AdsTransMode;
use crate::proto::command_id::CommandID;
use crate::proto::proto_traits::{Command, ReadFrom, WriteTo};
use std::convert::TryInto;

/// Each Request enum variant holds the struct with the data needed for a certain command.
/// The created Request can then be supplied to an [AMS header](super::ams_header).
/// ```
/// use crate::ads_proto::proto::request::*;
/// use crate::ads_proto::proto::proto_traits::{ReadFrom, WriteTo};
///
/// let request = Request::ReadDeviceInfo(ReadDeviceInfoRequest::new());
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum Request {
    Invalid(InvalidRequest),
    ReadDeviceInfo(ReadDeviceInfoRequest),
    ReadState(ReadStateRequest),
    Read(ReadRequest),
    Write(WriteRequest),
    WriteControl(WriteControlRequest),
    AddDeviceNotification(AddDeviceNotificationRequest),
    DeleteDeviceNotification(DeleteDeviceNotificationRequest),
    DeviceNotification(DeviceNotificationRequest),
    ReadWrite(ReadWriteRequest),
}

impl WriteTo for Request {
    fn write_to<W: Write>(&self, wtr: W) -> io::Result<()> {
        match self {
            Request::Invalid(_) => Ok(()),
            Request::ReadDeviceInfo(_) => Ok(()),
            Request::ReadState(_) => Ok(()),
            Request::Read(r) => r.write_to(wtr),
            Request::Write(r) => r.write_to(wtr),
            Request::ReadWrite(r) => r.write_to(wtr),
            Request::AddDeviceNotification(r) => r.write_to(wtr),
            Request::WriteControl(r) => r.write_to(wtr),
            Request::DeviceNotification(_) => Ok(()),
            Request::DeleteDeviceNotification(r) => r.write_to(wtr),
        }
    }
}

impl Command for Request {
    fn command_id(&self) -> CommandID {
        match self {
            Request::Invalid(r) => r.command_id,
            Request::ReadDeviceInfo(r) => r.command_id,
            Request::ReadState(r) => r.command_id,
            Request::Read(r) => r.command_id,
            Request::Write(r) => r.command_id,
            Request::ReadWrite(r) => r.command_id,
            Request::AddDeviceNotification(r) => r.command_id,
            Request::WriteControl(r) => r.command_id,
            Request::DeviceNotification(r) => r.command_id,
            Request::DeleteDeviceNotification(r) => r.command_id,
        }
    }
}

impl From<InvalidRequest> for Request {
    fn from(request: InvalidRequest) -> Self {
        Request::Invalid(request)
    }
}

impl TryInto<InvalidRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<InvalidRequest, Self::Error> {
        match self {
            Request::Invalid(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

impl From<ReadDeviceInfoRequest> for Request {
    fn from(request: ReadDeviceInfoRequest) -> Self {
        Request::ReadDeviceInfo(request)
    }
}

impl TryInto<ReadDeviceInfoRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<ReadDeviceInfoRequest, Self::Error> {
        match self {
            Request::ReadDeviceInfo(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

impl From<ReadStateRequest> for Request {
    fn from(request: ReadStateRequest) -> Self {
        Request::ReadState(request)
    }
}

impl TryInto<ReadStateRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<ReadStateRequest, Self::Error> {
        match self {
            Request::ReadState(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

impl From<ReadRequest> for Request {
    fn from(request: ReadRequest) -> Self {
        Request::Read(request)
    }
}

impl TryInto<ReadRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<ReadRequest, Self::Error> {
        match self {
            Request::Read(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

impl From<WriteRequest> for Request {
    fn from(request: WriteRequest) -> Self {
        Request::Write(request)
    }
}

impl TryInto<WriteRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<WriteRequest, Self::Error> {
        match self {
            Request::Write(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

impl From<WriteControlRequest> for Request {
    fn from(request: WriteControlRequest) -> Self {
        Request::WriteControl(request)
    }
}

impl TryInto<WriteControlRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<WriteControlRequest, Self::Error> {
        match self {
            Request::WriteControl(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

impl From<AddDeviceNotificationRequest> for Request {
    fn from(request: AddDeviceNotificationRequest) -> Self {
        Request::AddDeviceNotification(request)
    }
}

impl TryInto<AddDeviceNotificationRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<AddDeviceNotificationRequest, Self::Error> {
        match self {
            Request::AddDeviceNotification(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

impl From<DeleteDeviceNotificationRequest> for Request {
    fn from(request: DeleteDeviceNotificationRequest) -> Self {
        Request::DeleteDeviceNotification(request)
    }
}

impl TryInto<DeleteDeviceNotificationRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<DeleteDeviceNotificationRequest, Self::Error> {
        match self {
            Request::DeleteDeviceNotification(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

impl From<DeviceNotificationRequest> for Request {
    fn from(request: DeviceNotificationRequest) -> Self {
        Request::DeviceNotification(request)
    }
}

impl TryInto<DeviceNotificationRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<DeviceNotificationRequest, Self::Error> {
        match self {
            Request::DeviceNotification(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

impl From<ReadWriteRequest> for Request {
    fn from(request: ReadWriteRequest) -> Self {
        Request::ReadWrite(request)
    }
}

impl TryInto<ReadWriteRequest> for Request {
    type Error = TryIntoError;

    fn try_into(self) -> Result<ReadWriteRequest, Self::Error> {
        match self {
            Request::ReadWrite(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoRequestFailed),
        }
    }
}

/// ADS Invalid request
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidRequest {
    command_id: CommandID,
}

impl InvalidRequest {
    pub fn new() -> Self {
        InvalidRequest {
            command_id: CommandID::Invalid,
        }
    }
}

impl Default for InvalidRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// ADS read device info request
#[derive(Debug, Clone, PartialEq)]
pub struct ReadDeviceInfoRequest {
    command_id: CommandID,
}

impl ReadDeviceInfoRequest {
    pub fn new() -> Self {
        ReadDeviceInfoRequest {
            command_id: CommandID::ReadDeviceInfo,
        }
    }
}

impl Default for ReadDeviceInfoRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// ADS read device info request
#[derive(Debug, Clone, PartialEq)]
pub struct ReadStateRequest {
    command_id: CommandID,
}

impl ReadStateRequest {
    pub fn new() -> Self {
        ReadStateRequest {
            command_id: CommandID::ReadState,
        }
    }
}

impl Default for ReadStateRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// ADS read device info request
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceNotificationRequest {
    command_id: CommandID,
}

impl DeviceNotificationRequest {
    pub fn new() -> Self {
        DeviceNotificationRequest {
            command_id: CommandID::DeviceNotification,
        }
    }
}

impl Default for DeviceNotificationRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// ADS Read
#[derive(Debug, Clone, PartialEq)]
pub struct ReadRequest {
    pub index_group: u32,
    pub index_offset: u32,
    pub length: u32,
    pub command_id: CommandID,
}

impl ReadRequest {
    pub fn new(index_group: u32, index_offset: u32, length: u32) -> Self {
        ReadRequest {
            index_group,
            index_offset,
            length,
            command_id: CommandID::Read,
        }
    }
}

impl WriteTo for ReadRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.index_group)?;
        wtr.write_u32::<LittleEndian>(self.index_offset)?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        Ok(())
    }
}

impl ReadFrom for ReadRequest {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(ReadRequest {
            index_group: read.read_u32::<LittleEndian>()?,
            index_offset: read.read_u32::<LittleEndian>()?,
            length: read.read_u32::<LittleEndian>()?,
            command_id: CommandID::Read,
        })
    }
}

///ADS Write
#[derive(Debug, Clone, PartialEq)]
pub struct WriteRequest {
    pub index_group: u32,
    pub index_offset: u32,
    pub length: u32,
    pub data: Vec<u8>,
    pub command_id: CommandID,
}

impl WriteRequest {
    pub fn new(index_group: u32, index_offset: u32, data: Vec<u8>) -> Self {
        WriteRequest {
            index_group,
            index_offset,
            length: data.len() as u32,
            data,
            command_id: CommandID::Write,
        }
    }
}

impl WriteTo for WriteRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.index_group)?;
        wtr.write_u32::<LittleEndian>(self.index_offset)?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        wtr.write_all(self.data.as_slice())?;
        Ok(())
    }
}

impl ReadFrom for WriteRequest {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let index_group = read.read_u32::<LittleEndian>()?;
        let index_offset = read.read_u32::<LittleEndian>()?;
        let length = read.read_u32::<LittleEndian>()?;
        let mut data: Vec<u8> = Vec::with_capacity(length as usize);
        read.read_to_end(&mut data)?;

        Ok(WriteRequest {
            index_group,
            index_offset,
            length,
            data,
            command_id: CommandID::Write,
        })
    }
}

/// ADS Write Control
#[derive(Debug, Clone, PartialEq)]
pub struct WriteControlRequest {
    pub ads_state: AdsState,
    pub device_state: u16,
    pub length: u32,
    pub data: Vec<u8>,
    pub command_id: CommandID,
}

impl WriteControlRequest {
    pub fn new(ads_state: AdsState, device_state: u16, length: u32, data: Vec<u8>) -> Self {
        WriteControlRequest {
            ads_state,
            device_state,
            length,
            data,
            command_id: CommandID::WriteControl,
        }
    }
}

impl WriteTo for WriteControlRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        self.ads_state.write_to(&mut wtr)?;
        wtr.write_u16::<LittleEndian>(self.device_state)?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        wtr.write_all(self.data.as_slice())?;
        Ok(())
    }
}

impl ReadFrom for WriteControlRequest {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let ads_state = AdsState::read_from(read)?;
        let device_state = read.read_u16::<LittleEndian>()?;
        let length = read.read_u32::<LittleEndian>()?;
        let mut data: Vec<u8> = Vec::with_capacity(length as usize);
        read.read_to_end(&mut data)?;
        Ok(WriteControlRequest {
            ads_state,
            device_state,
            length,
            data,
            command_id: CommandID::WriteControl,
        })
    }
}

/// ADS Add Device Notification
#[derive(Debug, PartialEq, Clone)]
pub struct AddDeviceNotificationRequest {
    pub index_group: u32,
    pub index_offset: u32,
    pub length: u32,
    pub transmission_mode: AdsTransMode,
    pub max_delay: u32,
    pub cycle_time: u32,
    pub reserved: [u8; 16],
    pub command_id: CommandID,
}

impl AddDeviceNotificationRequest {
    pub fn new(
        index_group: u32,
        index_offset: u32,
        length: u32,
        transmission_mode: AdsTransMode,
        max_delay: u32,
        cycle_time: u32,
    ) -> Self {
        AddDeviceNotificationRequest {
            index_group,
            index_offset,
            length,
            transmission_mode,
            max_delay,
            cycle_time,
            reserved: [0; 16],
            command_id: CommandID::AddDeviceNotification,
        }
    }
}

impl WriteTo for AddDeviceNotificationRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.index_group)?;
        wtr.write_u32::<LittleEndian>(self.index_offset)?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        self.transmission_mode.write_to(&mut wtr)?;
        wtr.write_u32::<LittleEndian>(self.max_delay)?;
        wtr.write_u32::<LittleEndian>(self.cycle_time)?;
        wtr.write_all(&self.reserved)?;
        Ok(())
    }
}

impl ReadFrom for AddDeviceNotificationRequest {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(AddDeviceNotificationRequest {
            index_group: read.read_u32::<LittleEndian>()?,
            index_offset: read.read_u32::<LittleEndian>()?,
            length: read.read_u32::<LittleEndian>()?,
            transmission_mode: AdsTransMode::read_from(read)?,
            max_delay: read.read_u32::<LittleEndian>()?,
            cycle_time: read.read_u32::<LittleEndian>()?,
            reserved: [0; 16],
            command_id: CommandID::AddDeviceNotification,
        })
    }
}

/// ADS read device info request
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteDeviceNotificationRequest {
    pub handle: u32,
    pub command_id: CommandID,
}

impl WriteTo for DeleteDeviceNotificationRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.handle)?;
        Ok(())
    }
}

impl ReadFrom for DeleteDeviceNotificationRequest {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(DeleteDeviceNotificationRequest {
            handle: read.read_u32::<LittleEndian>()?,
            command_id: CommandID::DeleteDeviceNotification,
        })
    }
}

impl DeleteDeviceNotificationRequest {
    pub fn new(handle: u32) -> Self {
        DeleteDeviceNotificationRequest {
            handle,
            command_id: CommandID::DeleteDeviceNotification,
        }
    }
}

/// ADS Read Write
#[derive(Debug, Clone, PartialEq)]
pub struct ReadWriteRequest {
    pub index_group: u32,
    pub index_offset: u32,
    pub read_length: u32,
    pub write_length: u32,
    pub data: Vec<u8>,
    pub command_id: CommandID,
}

impl ReadWriteRequest {
    pub fn new(index_group: u32, index_offset: u32, read_length: u32, data: Vec<u8>) -> Self {
        ReadWriteRequest {
            index_group,
            index_offset,
            read_length,
            write_length: data.len() as u32,
            data,
            command_id: CommandID::ReadWrite,
        }
    }
}

impl WriteTo for ReadWriteRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.index_group)?;
        wtr.write_u32::<LittleEndian>(self.index_offset)?;
        wtr.write_u32::<LittleEndian>(self.read_length)?;
        wtr.write_u32::<LittleEndian>(self.write_length)?;
        wtr.write_all(self.data.as_slice())?;
        Ok(())
    }
}

impl ReadFrom for ReadWriteRequest {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let index_group = read.read_u32::<LittleEndian>()?;
        let index_offset = read.read_u32::<LittleEndian>()?;
        let read_length = read.read_u32::<LittleEndian>()?;
        let write_length = read.read_u32::<LittleEndian>()?;
        let mut data: Vec<u8> = vec![0; write_length as usize];
        read.read_exact(&mut data)?;

        Ok(ReadWriteRequest {
            index_group,
            index_offset,
            read_length,
            write_length,
            data,
            command_id: CommandID::ReadWrite,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_from_invalid() {
        let invalid_request = InvalidRequest::new();

        assert_eq!(
            Request::Invalid(invalid_request.clone()),
            Request::from(invalid_request)
        );
    }

    #[test]
    fn request_try_into_invalid() {
        let invalid_request = InvalidRequest::new();

        let request = Request::Invalid(invalid_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(invalid_request, test);
    }

    #[test]
    fn request_from_read_device_info() {
        let read_device_info_request = ReadDeviceInfoRequest::new();

        assert_eq!(
            Request::ReadDeviceInfo(read_device_info_request.clone()),
            Request::from(read_device_info_request)
        );
    }

    #[test]
    fn request_try_into_read_device_info() {
        let read_device_info_request = ReadDeviceInfoRequest::new();

        let request = Request::ReadDeviceInfo(read_device_info_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(read_device_info_request, test);
    }

    #[test]
    fn request_from_read_state() {
        let read_state_request = ReadStateRequest::new();

        assert_eq!(
            Request::ReadState(read_state_request.clone()),
            Request::from(read_state_request)
        );
    }

    #[test]
    fn request_try_into_read_state() {
        let read_state_request = ReadStateRequest::new();

        let request = Request::ReadState(read_state_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(read_state_request, test);
    }

    #[test]
    fn request_from_read() {
        let read_request = ReadRequest::new(1, 1, 1);

        assert_eq!(
            Request::Read(read_request.clone()),
            Request::from(read_request)
        );
    }

    #[test]
    fn request_try_into_read() {
        let read_request = ReadRequest::new(1, 1, 1);

        let request = Request::Read(read_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(read_request, test);
    }

    #[test]
    fn request_from_write() {
        let write_request = WriteRequest::new(1, 1, vec![88]);

        assert_eq!(
            Request::Write(write_request.clone()),
            Request::from(write_request)
        );
    }

    #[test]
    fn request_try_into_write() {
        let write_request = WriteRequest::new(1, 1, vec![88]);
        let request = Request::Write(write_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(write_request, test);
    }

    #[test]
    fn request_from_write_control() {
        let write_control_request =
            WriteControlRequest::new(AdsState::AdsStateConfig, 1, 1, vec![88]);

        assert_eq!(
            Request::WriteControl(write_control_request.clone()),
            Request::from(write_control_request)
        );
    }

    #[test]
    fn request_try_into_write_control() {
        let write_control_request =
            WriteControlRequest::new(AdsState::AdsStateConfig, 1, 1, vec![88]);
        let request = Request::WriteControl(write_control_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(write_control_request, test);
    }

    #[test]
    fn request_from_add_device_notification() {
        let add_device_notification_request =
            AddDeviceNotificationRequest::new(1, 1, 1, AdsTransMode::Cyclic, 1, 1);

        assert_eq!(
            Request::AddDeviceNotification(add_device_notification_request.clone()),
            Request::from(add_device_notification_request)
        );
    }

    #[test]
    fn request_try_into_add_device_notification() {
        let add_device_notification_request =
            AddDeviceNotificationRequest::new(1, 1, 1, AdsTransMode::Cyclic, 1, 1);

        let request = Request::AddDeviceNotification(add_device_notification_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(add_device_notification_request, test);
    }

    #[test]
    fn request_from_delete_device_notification() {
        let delete_device_notification_request = DeleteDeviceNotificationRequest::new(1);

        assert_eq!(
            Request::DeleteDeviceNotification(delete_device_notification_request.clone()),
            Request::from(delete_device_notification_request)
        );
    }

    #[test]
    fn request_try_into_delete_device_notification() {
        let delete_device_notification_request = DeleteDeviceNotificationRequest::new(1);

        let request = Request::DeleteDeviceNotification(delete_device_notification_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(delete_device_notification_request, test);
    }

    #[test]
    fn request_from_device_notification() {
        let device_notification_request = DeviceNotificationRequest::new();

        assert_eq!(
            Request::DeviceNotification(device_notification_request.clone()),
            Request::from(device_notification_request)
        );
    }

    #[test]
    fn request_try_into_device_notification() {
        let device_notification_request = DeviceNotificationRequest::new();

        let request = Request::DeviceNotification(device_notification_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(device_notification_request, test);
    }

    #[test]
    fn request_from_read_write() {
        let read_write_request = ReadWriteRequest::new(1, 1, 1, vec![85]);

        assert_eq!(
            Request::ReadWrite(read_write_request.clone()),
            Request::from(read_write_request)
        );
    }

    #[test]
    fn request_try_into_read_write() {
        let read_write_request = ReadWriteRequest::new(1, 1, 1, vec![85]);

        let request = Request::ReadWrite(read_write_request.clone());
        let test = request.try_into().unwrap();

        assert_eq!(read_write_request, test);
    }

    #[test]
    fn read_request_test() {
        let mut buffer: Vec<u8> = Vec::new();
        Request::Read(ReadRequest::new(259, 259, 4))
            .write_to(&mut buffer)
            .unwrap();

        let compare: Vec<u8> = vec![3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0];
        assert_eq!(compare, buffer);
    }

    #[test]
    fn read_request_read_from_test() {
        let reader: Vec<u8> = vec![3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0];
        let request = ReadRequest::read_from(&mut reader.as_slice()).unwrap();

        let compare = ReadRequest::new(259, 259, 4);
        assert_eq!(request.index_group, compare.index_group);
        assert_eq!(request.index_offset, compare.index_offset);
        assert_eq!(request.length, compare.length);
    }

    #[test]
    fn write_uint_request_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let data: u32 = 12000;
        Request::Write(WriteRequest::new(259, 259, data.to_le_bytes().to_vec()))
            .write_to(&mut buffer)
            .unwrap();

        let compare: Vec<u8> = vec![3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0, 224, 46, 0, 0];
        assert_eq!(compare, buffer);
    }

    #[test]
    fn write_float_request_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let data: f32 = 12000.33;
        Request::Write(WriteRequest::new(259, 259, data.to_le_bytes().to_vec()))
            .write_to(&mut buffer)
            .unwrap();

        let compare: Vec<u8> = vec![3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0, 82, 129, 59, 70];
        assert_eq!(compare, buffer);
    }

    #[test]
    fn write_request_read_from_test() {
        let reader: Vec<u8> = vec![4, 1, 0, 0, 4, 1, 0, 0, 4, 0, 0, 0, 225, 46, 0, 0];
        let request = WriteRequest::read_from(&mut reader.as_slice()).unwrap();
        let data_value: u32 = 12001;
        let data = data_value.to_le_bytes();
        let compare = WriteRequest::new(260, 260, data.to_vec());

        assert_eq!(
            request.index_group, compare.index_group,
            "Wrong index group"
        );
        assert_eq!(
            request.index_offset, compare.index_offset,
            "Wrong index offset"
        );
        assert_eq!(request.length, compare.length, "Wrong length");
        assert_eq!(request.data, data, "Data not as expected");
    }

    #[test]
    fn write_control_request_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let data: u8 = 0;
        Request::WriteControl(WriteControlRequest::new(
            AdsState::AdsStateIdle,
            296,
            1,
            data.to_le_bytes().to_vec(),
        ))
        .write_to(&mut buffer)
        .unwrap();

        let compare: Vec<u8> = vec![1, 0, 40, 1, 1, 0, 0, 0, 0];
        assert_eq!(compare, buffer);
    }

    #[test]
    fn write_contro_request_read_from_test() {
        let reader: Vec<u8> = vec![1, 0, 40, 1, 1, 0, 0, 0, 0, 0, 0, 0];
        let request = WriteControlRequest::read_from(&mut reader.as_slice()).unwrap();
        let data_value: u32 = 0;
        let data = data_value.to_le_bytes();
        let compare = WriteControlRequest::new(AdsState::AdsStateIdle, 296, 1, data.to_vec());

        assert_eq!(request.ads_state, compare.ads_state, "Wrong Ads state");
        assert_eq!(
            request.device_state, compare.device_state,
            "Wrong device state"
        );
        assert_eq!(request.length, compare.length, "Wrong length");
        assert_eq!(request.data, data, "Data not as expected"); //4 byte -> data_value is u32
    }

    #[test]
    fn read_write_request_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let data: u32 = 40000;
        let data: Vec<u8> = data.to_le_bytes().to_vec();
        Request::ReadWrite(ReadWriteRequest::new(259, 259, 4, data))
            .write_to(&mut buffer)
            .unwrap();

        let compare: Vec<u8> = vec![
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 64, 156, 0, 0,
        ];
        assert_eq!(compare, buffer);
    }

    #[test]
    fn read_write_request_read_from_test() {
        let reader: Vec<u8> = vec![3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 0, 0];
        let request = ReadWriteRequest::read_from(&mut reader.as_slice()).unwrap();
        let data_value: u16 = 0;
        let data = data_value.to_le_bytes();
        let compare = ReadWriteRequest::new(259, 259, 4, data.to_vec());

        assert_eq!(
            request.index_group, compare.index_group,
            "Wrong index group"
        );
        assert_eq!(
            request.index_offset, compare.index_offset,
            "Wrong index offset"
        );
        assert_eq!(
            request.read_length, compare.read_length,
            "Wrong read length"
        );
        assert_eq!(
            request.write_length, compare.write_length,
            "Wrong write length"
        );
        assert_eq!(request.command_id, compare.command_id, "Wrong command id");
        assert_eq!(request.data, data, "Data not as expected"); //2 byte -> data_value is u16
    }

    #[test]
    fn add_device_notification_request_test() {
        let mut buffer: Vec<u8> = Vec::new();
        Request::AddDeviceNotification(AddDeviceNotificationRequest::new(
            259,
            259,
            4,
            AdsTransMode::Cyclic,
            1,
            1,
        ))
        .write_to(&mut buffer)
        .unwrap();

        let compare: Vec<u8> = vec![
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(compare, buffer);
    }

    #[test]
    fn add_device_notification_request_read_from_test() {
        let reader: Vec<u8> = vec![
            3, 1, 0, 0, 3, 1, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let request = AddDeviceNotificationRequest::read_from(&mut reader.as_slice()).unwrap();
        let compare = AddDeviceNotificationRequest::new(259, 259, 4, AdsTransMode::Cyclic, 5, 1);

        assert_eq!(
            request.index_group, compare.index_group,
            "Wrong index group"
        );
        assert_eq!(
            request.index_offset, compare.index_offset,
            "Wrong index offset"
        );
        assert_eq!(request.length, compare.length, "Wrong length");
        assert_eq!(
            request.transmission_mode, compare.transmission_mode,
            "Wrong transmission mode"
        );
        assert_eq!(
            request.max_delay, compare.max_delay,
            "Wrong max delay wrong"
        );
        assert_eq!(request.cycle_time, compare.cycle_time, "Wrong cycle time");
        assert_eq!(
            request.reserved, compare.reserved,
            "Reserved not as expected"
        );
    }

    #[test]
    fn delete_device_notification_request_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let notification_handle = DeleteDeviceNotificationRequest::new(1234);
        Request::DeleteDeviceNotification(notification_handle)
            .write_to(&mut buffer)
            .unwrap();

        let compare: Vec<u8> = vec![210, 4, 0, 0];
        assert_eq!(compare, buffer);
    }

    #[test]
    fn delete_device_notification_request_read_from_test() {
        let reader: Vec<u8> = vec![210, 4, 0, 0];
        let request = DeleteDeviceNotificationRequest::read_from(&mut reader.as_slice()).unwrap();
        let compare = DeleteDeviceNotificationRequest::new(1234);

        assert_eq!(request.handle, compare.handle, "Wrong handle");
        assert_eq!(request.command_id, compare.command_id, "Wrong command id");
    }
}
