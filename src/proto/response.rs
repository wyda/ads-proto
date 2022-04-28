use crate::error::{AdsError, TryIntoError};
use crate::proto::ads_state::AdsState;
use crate::proto::command_id::CommandID;
use crate::proto::proto_traits::{Command, ReadFrom, WriteTo};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::string::FromUtf8Error;

/// Each Response enum variant holds the struct with the data needed for a certain command.
/// The created Response can then be supplied to an [AMS header](super::ams_header).
/// ```
/// use crate::ads_proto::proto::response::*;
/// use crate::ads_proto::proto::proto_traits::{ReadFrom, WriteTo};
/// use crate::ads_proto::error::AdsError;
///
/// let response = Response::ReadDeviceInfo(ReadDeviceInfoResponse::new(
///     AdsError::ErrNoError,
///     1,
///     2,
///     33,
///     [1; 16],
/// ));
/// ```
#[derive(Debug, PartialEq)]
pub enum Response {
    Invalid(InvalidResponse),
    ReadDeviceInfo(ReadDeviceInfoResponse),
    Read(ReadResponse),
    Write(WriteResponse),
    ReadState(ReadStateResponse),
    WriteControl(WriteControlResponse),
    AddDeviceNotification(AddDeviceNotificationResponse),
    DeleteDeviceNotification(DeleteDeviceNotificationResponse),
    DeviceNotification(AdsNotificationStream),
    ReadWrite(ReadWriteResponse),
}

impl WriteTo for Response {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        match self {
            Response::Invalid(_) => Ok(()),
            Response::ReadDeviceInfo(w) => w.write_to(&mut wtr),
            Response::Read(w) => w.write_to(&mut wtr),
            Response::Write(w) => w.write_to(&mut wtr),
            Response::ReadState(w) => w.write_to(&mut wtr),
            Response::WriteControl(w) => w.write_to(&mut wtr),
            Response::AddDeviceNotification(w) => w.write_to(&mut wtr),
            Response::DeleteDeviceNotification(w) => w.write_to(&mut wtr),
            Response::DeviceNotification(w) => w.write_to(&mut wtr),
            Response::ReadWrite(w) => w.write_to(&mut wtr),
        }
    }
}

impl Command for Response {
    fn command_id(&self) -> CommandID {
        match self {
            Response::Invalid(r) => r.command_id,
            Response::ReadDeviceInfo(r) => r.command_id,
            Response::ReadState(r) => r.command_id,
            Response::Read(r) => r.command_id,
            Response::Write(r) => r.command_id,
            Response::ReadWrite(r) => r.command_id,
            Response::AddDeviceNotification(r) => r.command_id,
            Response::WriteControl(r) => r.command_id,
            Response::DeviceNotification(r) => r.command_id,
            Response::DeleteDeviceNotification(r) => r.command_id,
        }
    }
}

impl From<InvalidResponse> for Response {
    fn from(request: InvalidResponse) -> Self {
        Response::Invalid(request)
    }
}

impl TryInto<InvalidResponse> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<InvalidResponse, Self::Error> {
        match self {
            Response::Invalid(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

impl From<ReadDeviceInfoResponse> for Response {
    fn from(response: ReadDeviceInfoResponse) -> Self {
        Response::ReadDeviceInfo(response)
    }
}

impl TryInto<ReadDeviceInfoResponse> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<ReadDeviceInfoResponse, Self::Error> {
        match self {
            Response::ReadDeviceInfo(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

impl From<WriteResponse> for Response {
    fn from(response: WriteResponse) -> Self {
        Response::Write(response)
    }
}

impl TryInto<WriteResponse> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<WriteResponse, Self::Error> {
        match self {
            Response::Write(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

impl From<WriteControlResponse> for Response {
    fn from(response: WriteControlResponse) -> Self {
        Response::WriteControl(response)
    }
}

impl TryInto<WriteControlResponse> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<WriteControlResponse, Self::Error> {
        match self {
            Response::WriteControl(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

impl From<ReadStateResponse> for Response {
    fn from(response: ReadStateResponse) -> Self {
        Response::ReadState(response)
    }
}

impl TryInto<ReadStateResponse> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<ReadStateResponse, Self::Error> {
        match self {
            Response::ReadState(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

impl From<AddDeviceNotificationResponse> for Response {
    fn from(response: AddDeviceNotificationResponse) -> Self {
        Response::AddDeviceNotification(response)
    }
}

impl TryInto<AddDeviceNotificationResponse> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<AddDeviceNotificationResponse, Self::Error> {
        match self {
            Response::AddDeviceNotification(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

impl From<DeleteDeviceNotificationResponse> for Response {
    fn from(response: DeleteDeviceNotificationResponse) -> Self {
        Response::DeleteDeviceNotification(response)
    }
}

impl TryInto<DeleteDeviceNotificationResponse> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<DeleteDeviceNotificationResponse, Self::Error> {
        match self {
            Response::DeleteDeviceNotification(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

impl From<AdsNotificationStream> for Response {
    fn from(response: AdsNotificationStream) -> Self {
        Response::DeviceNotification(response)
    }
}

impl TryInto<AdsNotificationStream> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<AdsNotificationStream, Self::Error> {
        match self {
            Response::DeviceNotification(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

impl From<ReadResponse> for Response {
    fn from(response: ReadResponse) -> Self {
        Response::Read(response)
    }
}

impl TryInto<ReadResponse> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<ReadResponse, Self::Error> {
        match self {
            Response::Read(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

impl From<ReadWriteResponse> for Response {
    fn from(response: ReadWriteResponse) -> Self {
        Response::ReadWrite(response)
    }
}

impl TryInto<ReadWriteResponse> for Response {
    type Error = TryIntoError;

    fn try_into(self) -> Result<ReadWriteResponse, Self::Error> {
        match self {
            Response::ReadWrite(r) => Ok(r),
            _ => Err(TryIntoError::TryIntoResponseFailed),
        }
    }
}

/// ADS Invalid response
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidResponse {
    command_id: CommandID,
}

impl InvalidResponse {
    pub fn new() -> Self {
        InvalidResponse {
            command_id: CommandID::Invalid,
        }
    }
}

impl Default for InvalidResponse {
    fn default() -> Self {
        Self::new()
    }
}

/// ADS Read Device Info
#[derive(Debug, PartialEq, Clone)]
pub struct ReadDeviceInfoResponse {
    pub result: AdsError,
    pub major_version: u8,
    pub minor_version: u8,
    pub version_build: u16,
    pub device_name: [u8; 16],
    pub command_id: CommandID,
}

impl ReadFrom for ReadDeviceInfoResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let result = AdsError::from(read.read_u32::<LittleEndian>()?);
        let major_version = read.read_u8()?;
        let minor_version = read.read_u8()?;
        let version_build = read.read_u16::<LittleEndian>()?;
        let mut device_name = [0; 16];
        read.read_exact(&mut device_name)?;
        Ok(Self {
            result,
            major_version,
            minor_version,
            version_build,
            device_name,
            command_id: CommandID::ReadDeviceInfo,
        })
    }
}

impl WriteTo for ReadDeviceInfoResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result.as_u32())?;
        wtr.write_u8(self.major_version)?;
        wtr.write_u8(self.minor_version)?;
        wtr.write_u16::<LittleEndian>(self.version_build)?;
        wtr.write_all(&self.device_name)?;
        Ok(())
    }
}

impl ReadDeviceInfoResponse {
    pub fn new(
        result: AdsError,
        major_version: u8,
        minor_version: u8,
        version_build: u16,
        device_name: [u8; 16],
    ) -> Self {
        ReadDeviceInfoResponse {
            result,
            major_version,
            minor_version,
            version_build,
            device_name,
            command_id: CommandID::ReadDeviceInfo,
        }
    }

    pub fn get_device_name(&self) -> Result<String, FromUtf8Error> {
        let name_bytes = self
            .device_name
            .to_vec()
            .iter()
            .filter(|value| **value > 0)
            .copied()
            .collect();

        String::from_utf8(name_bytes)
    }

    pub fn create_device_name_buf(device_name: &str) -> [u8; 16] {
        let mut device_name_buffer: [u8; 16] = [0; 16];
        for (n, b) in device_name.as_bytes().iter().enumerate() {
            if n == device_name_buffer.len() {
                break;
            }
            device_name_buffer[n] = *b;
        }
        device_name_buffer
    }
}

///Ads Write
#[derive(Debug, PartialEq, Clone)]
pub struct WriteResponse {
    pub result: AdsError,
    pub command_id: CommandID,
}

impl ReadFrom for WriteResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let result = AdsError::from(read.read_u32::<LittleEndian>()?);
        Ok(Self {
            result,
            command_id: CommandID::Write,
        })
    }
}

impl WriteTo for WriteResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result.as_u32())?;
        Ok(())
    }
}

impl WriteResponse {
    pub fn new(result: AdsError) -> Self {
        WriteResponse {
            result,
            command_id: CommandID::Write,
        }
    }
}

/// ADS Read State
#[derive(Debug, PartialEq, Clone)]
pub struct ReadStateResponse {
    pub result: AdsError,
    pub ads_state: AdsState,
    pub device_state: u16,
    pub command_id: CommandID,
}

impl ReadFrom for ReadStateResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(Self {
            result: AdsError::from(read.read_u32::<LittleEndian>()?),
            ads_state: AdsState::read_from(read)?,
            device_state: read.read_u16::<LittleEndian>()?,
            command_id: CommandID::ReadState,
        })
    }
}

impl WriteTo for ReadStateResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result.as_u32())?;
        self.ads_state.write_to(&mut wtr)?;
        wtr.write_u16::<LittleEndian>(self.device_state)?;
        Ok(())
    }
}

impl ReadStateResponse {
    pub fn new(result: AdsError, ads_state: AdsState, device_state: u16) -> Self {
        ReadStateResponse {
            result,
            ads_state,
            device_state,
            command_id: CommandID::ReadState,
        }
    }
}

///Write control
#[derive(Debug, PartialEq, Clone)]
pub struct WriteControlResponse {
    pub result: AdsError,
    pub command_id: CommandID,
}

impl ReadFrom for WriteControlResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(Self {
            result: AdsError::from(read.read_u32::<LittleEndian>()?),
            command_id: CommandID::WriteControl,
        })
    }
}

impl WriteTo for WriteControlResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result.as_u32())?;
        Ok(())
    }
}

impl WriteControlResponse {
    pub fn new(result: AdsError) -> Self {
        WriteControlResponse {
            result,
            command_id: CommandID::WriteControl,
        }
    }
}

/// ADS Add Device Notification
#[derive(Debug, PartialEq, Clone)]
pub struct AddDeviceNotificationResponse {
    pub result: AdsError,
    pub notification_handle: u32,
    pub command_id: CommandID,
}

impl ReadFrom for AddDeviceNotificationResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(Self {
            result: AdsError::from(read.read_u32::<LittleEndian>()?),
            notification_handle: read.read_u32::<LittleEndian>()?,
            command_id: CommandID::AddDeviceNotification,
        })
    }
}

impl WriteTo for AddDeviceNotificationResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result.as_u32())?;
        wtr.write_u32::<LittleEndian>(self.notification_handle)?;
        Ok(())
    }
}

impl AddDeviceNotificationResponse {
    pub fn new(result: AdsError, notification_handle: u32) -> Self {
        AddDeviceNotificationResponse {
            result,
            notification_handle,
            command_id: CommandID::AddDeviceNotification,
        }
    }
}

/// ADS Delete Device Notification
#[derive(Debug, PartialEq, Clone)]
pub struct DeleteDeviceNotificationResponse {
    pub result: AdsError,
    pub command_id: CommandID,
}

impl ReadFrom for DeleteDeviceNotificationResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        Ok(Self {
            result: AdsError::from(read.read_u32::<LittleEndian>()?),
            command_id: CommandID::DeleteDeviceNotification,
        })
    }
}

impl WriteTo for DeleteDeviceNotificationResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result.as_u32())?;
        Ok(())
    }
}

impl DeleteDeviceNotificationResponse {
    pub fn new(result: AdsError) -> Self {
        DeleteDeviceNotificationResponse {
            result,
            command_id: CommandID::DeleteDeviceNotification,
        }
    }
}

//ADS Device Notification Response
#[derive(Debug, PartialEq, Clone)]
pub struct AdsNotificationSample {
    pub notification_handle: u32,
    pub sample_size: u32,
    pub data: Vec<u8>,
}

impl AdsNotificationSample {
    pub fn new(notification_handle: u32, data: Vec<u8>) -> Self {
        AdsNotificationSample {
            notification_handle,
            sample_size: data.len() as u32,
            data,
        }
    }
    pub fn sample_len(&self) -> usize {
        //plus fixed byte length (notification_handle, sample_size)
        self.data.len() + 8
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AdsStampHeader {
    pub time_stamp: u64,
    pub samples: u32,
    pub notification_samples: Vec<AdsNotificationSample>,
}

impl ReadFrom for AdsStampHeader {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let time_stamp = read.read_u64::<LittleEndian>()?;
        let samples = read.read_u32::<LittleEndian>()?;
        let mut notification_samples: Vec<AdsNotificationSample> =
            Vec::with_capacity(samples as usize);

        for _ in 0..samples {
            let notification_handle = read.read_u32::<LittleEndian>()?;
            let sample_size = read.read_u32::<LittleEndian>()?;
            let mut data = vec![0; sample_size as usize];
            read.read_exact(&mut data)?;
            notification_samples.push(AdsNotificationSample {
                notification_handle,
                sample_size,
                data,
            });
        }

        Ok(Self {
            time_stamp,
            samples,
            notification_samples,
        })
    }
}

impl WriteTo for AdsStampHeader {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u64::<LittleEndian>(self.time_stamp)?;
        wtr.write_u32::<LittleEndian>(self.samples)?;

        for sample in &self.notification_samples {
            wtr.write_u32::<LittleEndian>(sample.notification_handle)?;
            wtr.write_u32::<LittleEndian>(sample.sample_size)?;
            wtr.write_all(sample.data.as_slice())?;
        }
        Ok(())
    }
}

impl AdsStampHeader {
    pub fn new(
        time_stamp: u64,
        samples: u32,
        notification_samples: Vec<AdsNotificationSample>,
    ) -> Self {
        AdsStampHeader {
            time_stamp,
            samples,
            notification_samples,
        }
    }

    pub fn stamp_len(&self) -> usize {
        let mut len: usize = 0;
        for sample in &self.notification_samples {
            len += sample.sample_len();
        }
        //plus fixed byte size (time_stamp, samples)
        len + 12
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AdsNotificationStream {
    pub length: u32,
    pub stamps: u32,
    pub ads_stamp_headers: Vec<AdsStampHeader>,
    pub command_id: CommandID,
}

impl ReadFrom for AdsNotificationStream {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let length = read.read_u32::<LittleEndian>()?;
        let stamps = read.read_u32::<LittleEndian>()?;
        let stamp_data_size = ((length - 4) / stamps) as u32; //-4 -> stamps is in length incl. but already read in previous line!
        let mut ads_stamp_headers: Vec<AdsStampHeader> = Vec::with_capacity(stamps as usize);
        let mut buffer: Vec<u8> = vec![0; (stamp_data_size) as usize];
        for _ in 0..stamps {
            read.read_exact(&mut buffer)?;
            let stamp = AdsStampHeader::read_from(&mut buffer.as_slice())?;
            ads_stamp_headers.push(stamp);
        }

        Ok(Self {
            length,
            stamps,
            ads_stamp_headers,
            command_id: CommandID::DeviceNotification,
        })
    }
}

impl WriteTo for AdsNotificationStream {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.length)?;
        wtr.write_u32::<LittleEndian>(self.stamps)?;

        for stamp_header in &self.ads_stamp_headers {
            stamp_header.write_to(&mut wtr)?;
        }
        Ok(())
    }
}

impl AdsNotificationStream {
    pub fn new(length: u32, stamps: u32, ads_stamp_headers: Vec<AdsStampHeader>) -> Self {
        AdsNotificationStream {
            length,
            stamps,
            ads_stamp_headers,
            command_id: CommandID::DeviceNotification,
        }
    }

    pub fn stream_len(&self) -> usize {
        let mut len: usize = 0;
        for stamp in &self.ads_stamp_headers {
            len += stamp.stamp_len();
        }
        //plus fixed byte size (length, stamps)
        len + 8
    }
}

//Ads Read response
#[derive(Debug, Clone, PartialEq)]
pub struct ReadResponse {
    pub result: AdsError,
    pub length: u32,
    pub data: Vec<u8>,
    pub command_id: CommandID,
}

impl ReadFrom for ReadResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let result = AdsError::from(read.read_u32::<LittleEndian>()?);
        let length = read.read_u32::<LittleEndian>()?;
        let mut data = Vec::with_capacity(length as usize);
        read.read_to_end(&mut data)?;
        Ok(Self {
            result,
            length,
            data,
            command_id: CommandID::Read,
        })
    }
}

impl WriteTo for ReadResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result.as_u32())?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        wtr.write_all(self.data.as_slice())?;
        Ok(())
    }
}

impl ReadResponse {
    pub fn new(result: AdsError, data: Vec<u8>) -> Self {
        ReadResponse {
            result,
            length: data.len() as u32,
            data,
            command_id: CommandID::Read,
        }
    }
}

//Ads ReadWrite response
#[derive(Debug, Clone, PartialEq)]
pub struct ReadWriteResponse {
    pub result: AdsError,
    pub length: u32,
    pub data: Vec<u8>,
    pub command_id: CommandID,
}

impl ReadFrom for ReadWriteResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let result = AdsError::from(read.read_u32::<LittleEndian>()?);
        let length = read.read_u32::<LittleEndian>()?;
        let mut data = Vec::with_capacity(length as usize);
        read.read_to_end(&mut data)?;
        Ok(Self {
            result,
            length,
            data,
            command_id: CommandID::ReadWrite,
        })
    }
}

impl WriteTo for ReadWriteResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result.as_u32())?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        wtr.write_all(self.data.as_slice())?;
        Ok(())
    }
}

impl ReadWriteResponse {
    pub fn new(result: AdsError, data: Vec<u8>) -> Self {
        ReadWriteResponse {
            result,
            length: data.len() as u32,
            data,
            command_id: CommandID::ReadWrite,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_from_read_device_info() {
        let read_device_info_response =
            ReadDeviceInfoResponse::new(AdsError::ErrNoError, 1, 2, 33, [1; 16]);

        assert_eq!(
            Response::ReadDeviceInfo(read_device_info_response.clone()),
            Response::from(read_device_info_response)
        );
    }

    #[test]
    fn response_try_into_read_device_info() {
        let read_device_info_response =
            ReadDeviceInfoResponse::new(AdsError::ErrNoError, 1, 2, 33, [1; 16]);

        let response = Response::ReadDeviceInfo(read_device_info_response.clone());
        assert_eq!(CommandID::ReadDeviceInfo, response.command_id());

        let test = response.try_into().unwrap();
        assert_eq!(read_device_info_response, test);
    }

    #[test]
    fn response_from_read() {
        let read_response = ReadResponse::new(AdsError::ErrNoError, vec![66]);

        assert_eq!(
            Response::Read(read_response.clone()),
            Response::from(read_response)
        );
    }

    #[test]
    fn response_try_into_read() {
        let read_response = ReadResponse::new(AdsError::ErrNoError, vec![66]);

        let response = Response::Read(read_response.clone());
        assert_eq!(CommandID::Read, response.command_id());

        let test = response.try_into().unwrap();
        assert_eq!(read_response, test);
    }

    #[test]
    fn response_from_write() {
        let write_response = WriteResponse::new(AdsError::ErrNoError);

        assert_eq!(
            Response::Write(write_response.clone()),
            Response::from(write_response)
        );
    }

    #[test]
    fn response_try_into_write() {
        let write_response = WriteResponse::new(AdsError::ErrNoError);

        let response = Response::Write(write_response.clone());
        assert_eq!(CommandID::Write, response.command_id());

        let test = response.try_into().unwrap();
        assert_eq!(write_response, test);
    }

    #[test]
    fn response_from_read_state() {
        let read_state_response =
            ReadStateResponse::new(AdsError::ErrNoError, AdsState::AdsStateConfig, 123);

        assert_eq!(
            Response::ReadState(read_state_response.clone()),
            Response::from(read_state_response)
        );
    }

    #[test]
    fn response_try_into_read_state() {
        let read_state_response =
            ReadStateResponse::new(AdsError::ErrNoError, AdsState::AdsStateConfig, 123);

        let response = Response::ReadState(read_state_response.clone());
        assert_eq!(CommandID::ReadState, response.command_id());

        let test = response.try_into().unwrap();
        assert_eq!(read_state_response, test);
    }

    #[test]
    fn response_from_write_control() {
        let write_control_response = WriteControlResponse::new(AdsError::ErrNoError);

        assert_eq!(
            Response::WriteControl(write_control_response.clone()),
            Response::from(write_control_response)
        );
    }

    #[test]
    fn response_try_into_write_control() {
        let write_control_response = WriteControlResponse::new(AdsError::ErrNoError);

        let response = Response::WriteControl(write_control_response.clone());
        assert_eq!(CommandID::WriteControl, response.command_id());

        let test = response.try_into().unwrap();
        assert_eq!(write_control_response, test);
    }

    #[test]
    fn response_from_add_device_notification() {
        let add_device_notification_response =
            AddDeviceNotificationResponse::new(AdsError::ErrNoError, 1);

        assert_eq!(
            CommandID::AddDeviceNotification,
            Response::AddDeviceNotification(add_device_notification_response.clone()).command_id()
        );

        assert_eq!(
            Response::AddDeviceNotification(add_device_notification_response.clone()),
            Response::from(add_device_notification_response)
        );
    }

    #[test]
    fn response_try_into_add_device_notification() {
        let add_device_notification_response =
            AddDeviceNotificationResponse::new(AdsError::ErrNoError, 1);

        let response = Response::AddDeviceNotification(add_device_notification_response.clone());
        assert_eq!(CommandID::AddDeviceNotification, response.command_id());

        let test = response.try_into().unwrap();
        assert_eq!(add_device_notification_response, test);
    }

    #[test]
    fn response_from_delete_device_notification() {
        let delete_device_notification_response =
            DeleteDeviceNotificationResponse::new(AdsError::ErrNoError);

        assert_eq!(
            Response::DeleteDeviceNotification(delete_device_notification_response.clone()),
            Response::from(delete_device_notification_response)
        );
    }

    #[test]
    fn response_try_into_delete_device_notification() {
        let delete_device_notification_response =
            DeleteDeviceNotificationResponse::new(AdsError::ErrNoError);

        let response =
            Response::DeleteDeviceNotification(delete_device_notification_response.clone());
        assert_eq!(CommandID::DeleteDeviceNotification, response.command_id());

        let test = response.try_into().unwrap();
        assert_eq!(delete_device_notification_response, test);
    }

    #[test]
    fn response_from_device_notification() {
        let notification_sample1: Vec<u8> = vec![4, 0, 0, 0, 2, 0, 0, 0, 6, 0];
        let notification_sample2: Vec<u8> = vec![4, 0, 0, 0, 4, 0, 0, 0, 9, 0, 0, 0];

        let mut stamp_header: Vec<u8> = vec![255, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0];
        stamp_header.extend(notification_sample1);
        stamp_header.extend(notification_sample2);

        let mut notification_stream: Vec<u8> = vec![72, 0, 0, 0, 2, 0, 0, 0];
        notification_stream.extend(stamp_header.clone());
        notification_stream.extend(stamp_header);

        let device_notification_response =
            AdsNotificationStream::read_from(&mut notification_stream.as_slice()).unwrap();

        assert_eq!(
            Response::DeviceNotification(device_notification_response.clone()),
            Response::from(device_notification_response)
        );
    }

    #[test]
    fn response_try_into_device_notification() {
        let notification_sample1: Vec<u8> = vec![4, 0, 0, 0, 2, 0, 0, 0, 6, 0];
        let notification_sample2: Vec<u8> = vec![4, 0, 0, 0, 4, 0, 0, 0, 9, 0, 0, 0];

        let mut stamp_header: Vec<u8> = vec![255, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0];
        stamp_header.extend(notification_sample1);
        stamp_header.extend(notification_sample2);

        let mut notification_stream: Vec<u8> = vec![72, 0, 0, 0, 2, 0, 0, 0];
        notification_stream.extend(stamp_header.clone());
        notification_stream.extend(stamp_header);

        let device_notification_response =
            AdsNotificationStream::read_from(&mut notification_stream.as_slice()).unwrap();

        let response = Response::DeviceNotification(device_notification_response.clone());
        assert_eq!(CommandID::DeviceNotification, response.command_id());

        let test = response.try_into().unwrap();
        assert_eq!(device_notification_response, test);
    }

    #[test]
    fn response_from_read_write() {
        let read_write_response = ReadWriteResponse::new(AdsError::ErrNoError, vec![66]);

        assert_eq!(
            CommandID::ReadWrite,
            Response::ReadWrite(read_write_response.clone()).command_id()
        );

        assert_eq!(
            Response::ReadWrite(read_write_response.clone()),
            Response::from(read_write_response)
        );
    }

    #[test]
    fn response_try_into_read_write() {
        let read_write_response = ReadWriteResponse::new(AdsError::ErrNoError, vec![66]);

        let response = Response::ReadWrite(read_write_response.clone());
        assert_eq!(CommandID::ReadWrite, response.command_id());

        let test = response.try_into().unwrap();
        assert_eq!(read_write_response, test);
    }

    #[test]
    fn read_device_info_response_write_to_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut device_name: [u8; 16] = [0; 16];

        for (n, b) in "Device".as_bytes().iter().enumerate() {
            device_name[n] = *b;
        }

        let device_info_response =
            ReadDeviceInfoResponse::new(AdsError::ErrAccessDenied, 1, 2, 10, device_name);

        let response_data: Vec<u8> = vec![
            30, 0, 0, 0, 1, 2, 10, 0, 68, 101, 118, 105, 99, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        device_info_response.write_to(&mut buffer).unwrap();

        assert_eq!(buffer, response_data);
    }

    #[test]
    fn read_device_info_response_test() {
        let response_data: Vec<u8> = vec![
            30, 0, 0, 0, 2, 14, 1, 1, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 0, 0, 0,
            0, 0,
        ];

        let read_device_info_response =
            ReadDeviceInfoResponse::read_from(&mut response_data.as_slice()).unwrap();

        assert_eq!(read_device_info_response.result, AdsError::ErrAccessDenied);
        assert_eq!(read_device_info_response.major_version, 2);
        assert_eq!(read_device_info_response.minor_version, 14);
        assert_eq!(read_device_info_response.version_build, 257);

        let expected_device_name: [u8; 16] = [
            72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 0, 0, 0, 0, 0,
        ]; //Hello World
        assert_eq!(read_device_info_response.device_name, expected_device_name);
        let expected_device_name = "Hello World".to_string();
        assert_eq!(
            read_device_info_response.get_device_name().unwrap(),
            expected_device_name,
            "Parsing device name failed"
        );
    }

    #[test]
    fn read_device_info_device_name_buf_test() {
        let device_name: [u8; 16] = ReadDeviceInfoResponse::create_device_name_buf("Device");

        let device_info_response =
            ReadDeviceInfoResponse::new(AdsError::ErrAccessDenied, 1, 2, 10, device_name);

        assert_eq!(device_info_response.get_device_name().unwrap(), "Device");
    }

    #[test]
    fn read_device_info_device_name_buf_test2() {
        let device_name: [u8; 16] =
            ReadDeviceInfoResponse::create_device_name_buf("OverflowtestOverflowtest");

        let device_info_response =
            ReadDeviceInfoResponse::new(AdsError::ErrAccessDenied, 1, 2, 10, device_name);

        assert_eq!(
            device_info_response.get_device_name().unwrap(),
            "OverflowtestOver"
        );
    }

    #[test]
    fn read_response_test() {
        let response_data: Vec<u8> = vec![4, 0, 0, 0, 2, 0, 0, 0, 255, 2];

        let read_response = ReadResponse::read_from(&mut response_data.as_slice()).unwrap();

        assert_eq!(read_response.result, AdsError::ErrInsertMailBox);
        assert_eq!(read_response.length, 2);
        assert_eq!(read_response.data, vec![255, 2]);
    }

    #[test]
    fn read_response_write_to_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let data: u32 = 90000;
        let read_response =
            ReadResponse::new(AdsError::ErrAccessDenied, data.to_le_bytes().to_vec());
        read_response.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, [30, 0, 0, 0, 4, 0, 0, 0, 144, 95, 1, 0]);
    }

    #[test]
    fn write_response_test() {
        let response_data: Vec<u8> = vec![4, 0, 0, 0];

        let write_response = WriteResponse::read_from(&mut response_data.as_slice()).unwrap();

        assert_eq!(write_response.result, AdsError::from(4));
    }

    #[test]
    fn write_response_write_to_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let write_response = WriteResponse::new(AdsError::ErrAccessDenied);
        write_response.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, [30, 0, 0, 0]);
    }

    #[test]
    fn read_state_response_test() {
        let response_data: Vec<u8> = vec![4, 0, 0, 0, 9, 0, 1, 1];

        let read_state_response =
            ReadStateResponse::read_from(&mut response_data.as_slice()).unwrap();

        assert_eq!(read_state_response.result, AdsError::ErrInsertMailBox);
        assert_eq!(
            read_state_response.ads_state,
            AdsState::AdsStatePowerFailure
        );
        assert_eq!(read_state_response.device_state, 257);
    }

    #[test]
    fn read_state_response_write_to_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let read_state_response =
            ReadStateResponse::new(AdsError::ErrAccessDenied, AdsState::AdsStateConfig, 4);
        read_state_response.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, [30, 0, 0, 0, 15, 0, 4, 0]);
    }

    #[test]
    fn write_control_response_test() {
        let response_data: Vec<u8> = vec![30, 0, 0, 0];

        let write_control_response =
            WriteControlResponse::read_from(&mut response_data.as_slice()).unwrap();

        assert_eq!(write_control_response.result, AdsError::ErrAccessDenied);
    }

    #[test]
    fn write_control_response_write_to_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let write_control_response = WriteControlResponse::new(AdsError::ErrAccessDenied);
        write_control_response.write_to(&mut buffer).unwrap();
        assert_eq!(buffer, [30, 0, 0, 0]);
    }

    #[test]
    fn add_device_notification_response_test() {
        let response_data: Vec<u8> = vec![4, 0, 0, 0, 10, 0, 0, 0];

        let add_device_notification_response =
            AddDeviceNotificationResponse::read_from(&mut response_data.as_slice()).unwrap();

        assert_eq!(
            add_device_notification_response.result,
            AdsError::ErrInsertMailBox
        );
        assert_eq!(add_device_notification_response.notification_handle, 10);
    }

    #[test]
    fn add_device_notification_response_write_to_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let add_device_notification_response =
            AddDeviceNotificationResponse::new(AdsError::ErrInsertMailBox, 10);
        add_device_notification_response
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer, [4, 0, 0, 0, 10, 0, 0, 0]);
    }

    #[test]
    fn delete_device_notification_response_test() {
        let response_data: Vec<u8> = vec![4, 0, 0, 0];

        let delete_device_notification_response =
            DeleteDeviceNotificationResponse::read_from(&mut response_data.as_slice()).unwrap();

        assert_eq!(
            delete_device_notification_response.result,
            AdsError::ErrInsertMailBox
        );
    }

    #[test]
    fn delete_device_notification_response_write_to_test() {
        let mut buffer: Vec<u8> = Vec::new();
        let delete_device_notification_response =
            DeleteDeviceNotificationResponse::new(AdsError::ErrAccessDenied);
        delete_device_notification_response
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer, [30, 0, 0, 0]);
    }

    #[test]
    fn ads_notification_stream_test() {
        let notification_sample1: Vec<u8> = vec![4, 0, 0, 0, 2, 0, 0, 0, 6, 0];
        let notification_sample2: Vec<u8> = vec![4, 0, 0, 0, 4, 0, 0, 0, 9, 0, 0, 0];

        let mut stamp_header: Vec<u8> = vec![255, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0];
        stamp_header.extend(notification_sample1);
        stamp_header.extend(notification_sample2);

        let mut notification_stream: Vec<u8> = vec![72, 0, 0, 0, 2, 0, 0, 0];
        notification_stream.extend(stamp_header.clone());
        notification_stream.extend(stamp_header);

        let notification_data =
            AdsNotificationStream::read_from(&mut notification_stream.as_slice()).unwrap();

        assert_eq!(notification_data.length, 72, "Wrong data stream length");
        assert_eq!(notification_data.stamps, 2, "Wrong data stream stamp count");
        assert_eq!(
            notification_data.ads_stamp_headers.len(),
            2,
            "Wrong stamp header vec length"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[0]
                .notification_samples
                .len(),
            2,
            "Wrong notification sample vec len [0]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[0].samples, 2,
            "Wrong notification samples count [0]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[0].time_stamp, 255,
            "Wrong time stamp [0]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[0].notification_samples[0].notification_handle, 4,
            "Wrong notification handle [0][0]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[0].notification_samples[0].sample_size, 2,
            "Wrong sample size [0][0]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[0].notification_samples[0].data,
            vec![6, 0],
            "Wrong data [0][0]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[0].notification_samples[1].notification_handle, 4,
            "Wrong notification handle [0][1]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[0].notification_samples[1].sample_size, 4,
            "Wrong sample size [0][1]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[0].notification_samples[1].data,
            vec![9, 0, 0, 0],
            "Wrong data [0][1]"
        );

        assert_eq!(
            notification_data.ads_stamp_headers[1]
                .notification_samples
                .len(),
            2,
            "Wrong notification sample vec len [1]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[1].samples, 2,
            "Wrong notification samples count [1]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[1].time_stamp, 255,
            "Wrong time stamp [1]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[1].notification_samples[0].notification_handle, 4,
            "Wrong notification handle [1][0]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[1].notification_samples[0].sample_size, 2,
            "Wrong sample size [1][0]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[1].notification_samples[0].data,
            vec![6, 0],
            "Wrong data [1][0]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[1].notification_samples[1].notification_handle, 4,
            "Wrong notification handle [1][1]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[1].notification_samples[1].sample_size, 4,
            "Wrong sample size [1][1]"
        );
        assert_eq!(
            notification_data.ads_stamp_headers[1].notification_samples[1].data,
            vec![9, 0, 0, 0],
            "Wrong data [1][1]"
        );
    }

    #[test]
    fn ads_notification_stream_write_to_test() {
        //4+4+4=12byte
        let sample_data1: u32 = 1000;
        let notification_sample1 = AdsNotificationSample {
            notification_handle: 10,
            sample_size: 4,
            data: sample_data1.to_le_bytes().to_vec(),
        };

        //4+4+2=10byte
        let sample_data2: u16 = 2000;
        let notification_sample2 = AdsNotificationSample {
            notification_handle: 20,
            sample_size: 2,
            data: sample_data2.to_le_bytes().to_vec(),
        };

        //4+4+8=16byte
        let sample_data3: u64 = 3000;
        let notification_sample3 = AdsNotificationSample {
            notification_handle: 30,
            sample_size: 8,
            data: sample_data3.to_le_bytes().to_vec(),
        };

        //8+4+12+10=34byte
        let mut notification_samples = Vec::new();
        notification_samples.push(notification_sample1);
        notification_samples.push(notification_sample2);
        let stamp_header1 = AdsStampHeader::new(1234567890, 2, notification_samples);

        //8+4+16=28byte
        let mut notification_samples = Vec::new();
        notification_samples.push(notification_sample3);
        let stamp_header2 = AdsStampHeader::new(1234567890, 1, notification_samples);

        let mut stamp_headers = Vec::new();
        stamp_headers.push(stamp_header1);
        stamp_headers.push(stamp_header2);

        let mut len: usize = 0;
        for header in &stamp_headers {
            len += header.stamp_len();
        }
        len += 4; //4 byte for the u32 stamps var after length

        let expected_len: usize = 66;
        assert_eq!(&len, &expected_len, "Wrong number of bytes");

        //4+4+34+28=70byte
        let ads_notification_stream =
            AdsNotificationStream::new(len as u32, stamp_headers.len() as u32, stamp_headers);

        let expected_len: usize = 70;
        assert_eq!(
            &ads_notification_stream.stream_len(),
            &expected_len,
            "Wrong number of bytes"
        );

        let mut buffer: Vec<u8> = Vec::new();

        ads_notification_stream.write_to(&mut buffer).unwrap();

        #[rustfmt::skip]
        let expected_data = [
            //Notification stream Length
            66, 0, 0, 0,
            ////Notification stream number of stamps
            2, 0, 0, 0,
            //Stamp header1 time_stamp
            210, 2, 150, 73, 0, 0, 0, 0,
            //Stamp header1 number of samples
            2, 0, 0, 0,
            //Notification sample 1 notification handle
            10, 0, 0, 0,
            //Notification sample 1 sample size
            4, 0, 0, 0,
            //Notification sample 1 data
            232, 3, 0, 0,
            //Notification sample 2 notification handle
            20, 0, 0, 0,
            //Notification sample 2 sample size
            2, 0, 0, 0,
            //Notification sample 2 data
            208, 7,
            //Stamp header2 time_stamp
            210, 2, 150, 73, 0, 0, 0, 0,
            //Stamp header2 number of samples
            1, 0, 0, 0,
            //Notification sample 3 notification handle
            30, 0, 0, 0,
            //Notification sample 3 sample size
            8, 0, 0, 0,
            //Notification sample 3 data
            184, 11, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(buffer, expected_data, "Data in buffer is not as expected");
    }
}
