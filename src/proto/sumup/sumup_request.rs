use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

use crate::proto::command_id::CommandID;
use crate::proto::proto_traits::{ReadFrom, WriteTo};
use crate::proto::request::{ReadRequest, ReadWriteRequest, WriteRequest};

///Ads Sumup Read Write Request data
///Bundle multiple requestst toghether. Add this data to the read write request or parse from.
#[derive(Debug, Clone, PartialEq)]
pub struct SumupReadWriteRequest {
    read_write_requests: Vec<ReadWriteRequest>,
    command_id: CommandID,
}

#[derive(Debug, Clone, PartialEq)]
struct ReadWriteAccessData {
    index_group: u32,
    index_offset: u32,
    read_length: u32,
    write_length: u32,
}

impl WriteTo for ReadWriteAccessData {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.index_group)?;
        wtr.write_u32::<LittleEndian>(self.index_offset)?;
        wtr.write_u32::<LittleEndian>(self.read_length)?;
        wtr.write_u32::<LittleEndian>(self.write_length)?;
        Ok(())
    }
}

impl ReadFrom for ReadWriteAccessData {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let index_group = read.read_u32::<LittleEndian>()?;
        let index_offset = read.read_u32::<LittleEndian>()?;
        let read_length = read.read_u32::<LittleEndian>()?;
        let write_length = read.read_u32::<LittleEndian>()?;

        Ok(ReadWriteAccessData {
            index_group,
            index_offset,
            read_length,
            write_length,
        })
    }
}

impl SumupReadWriteRequest {
    pub fn new(read_write_requests: Vec<ReadWriteRequest>) -> Self {
        SumupReadWriteRequest {
            read_write_requests,
            command_id: CommandID::ReadWrite,
        }
    }
}

impl WriteTo for SumupReadWriteRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        let mut access_data: Vec<u8> = Vec::new();
        let mut data: Vec<u8> = Vec::new();
        for request in &self.read_write_requests {
            access_data.write_u32::<LittleEndian>(request.index_group)?;
            access_data.write_u32::<LittleEndian>(request.index_offset)?;
            access_data.write_u32::<LittleEndian>(request.read_length)?;
            access_data.write_u32::<LittleEndian>(request.write_length)?;
            data.write_all(request.data.as_slice())?;
        }
        access_data.append(&mut data);
        wtr.write_all(&access_data)?;
        Ok(())
    }
}

impl ReadFrom for SumupReadWriteRequest {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let mut data_buf: Vec<u8> = Vec::new();
        let mut read_write_access: Vec<ReadWriteAccessData> = Vec::new();

        //Read all bytes and get the total length
        read.read_to_end(&mut data_buf)?;
        let total_data_len = data_buf.len() as u32;
        let mut access_data_length: u32 = 0;
        let mut data_length: u32 = 0;
        let mut data_buf = data_buf.as_slice();

        //Get the access data bytes
        for _ in 0..total_data_len / 16 {
            let access_data = ReadWriteAccessData::read_from(&mut data_buf)?;
            data_length += access_data.write_length;
            read_write_access.push(access_data);
            access_data_length += 16;
            if (total_data_len - data_length - access_data_length) == 0 {
                break;
            }
        }

        //Get the actual data/value bytes and create ReadWriteRequests
        let mut read_write_requests: Vec<ReadWriteRequest> = Vec::new();
        for access in read_write_access {
            let mut buf = vec![0; access.write_length as usize];
            data_buf.read_exact(&mut buf)?;
            read_write_requests.push(ReadWriteRequest::new(
                access.index_group,
                access.index_offset,
                access.read_length,
                buf,
            ));
        }
        Ok(SumupReadWriteRequest::new(read_write_requests))
    }
}

///Ads Sumup Read Request data
///Bundle multiple requestst toghether. Add this data to a read write request or parse from.
#[derive(Debug, Clone, PartialEq)]
pub struct SumupReadRequest {
    read_requests: Vec<ReadRequest>,
    command_id: CommandID,
}

impl SumupReadRequest {
    pub fn new(read_requests: Vec<ReadRequest>) -> Self {
        SumupReadRequest {
            read_requests,
            command_id: CommandID::Read,
        }
    }

    pub fn expected_response_len(&self) -> u32 {
        let mut result = 0;
        for request in &self.read_requests {
            result += request.length + 8; //8 byte -> 4 byte result + 4 byte length in response data
        }
        result
    }

    pub fn request_count(&self) -> u32 {
        self.read_requests.len() as u32
    }
}

impl WriteTo for SumupReadRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        let mut access_data: Vec<u8> = Vec::new();
        for request in &self.read_requests {
            request.write_to(&mut access_data)?;
        }
        wtr.write_all(&access_data)?;
        Ok(())
    }
}

impl ReadFrom for SumupReadRequest {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let access_data_size: usize = 12; //Index group(4 byte) + index offset(4 byte) + read length(4 byte)
        let mut read_requests: Vec<ReadRequest> = Vec::new();
        loop {
            let mut buf = vec![0; access_data_size];
            match read.read_exact(&mut buf) {
                Ok(_) => (),
                Err(_) => {
                    break;
                }
            }
            read_requests.push(ReadRequest::read_from(&mut buf.as_slice())?);
        }
        Ok(SumupReadRequest::new(read_requests))
    }
}

///Ads Sumup Write Request data
///Bundle multiple requestst toghether. Add this data to the read write request or parse from.
#[derive(Debug, Clone, PartialEq)]
pub struct SumupWriteRequest {
    write_requests: Vec<WriteRequest>,
    command_id: CommandID,
}

#[derive(Debug, Clone, PartialEq)]
struct WriteAccessData {
    index_group: u32,
    index_offset: u32,
    write_length: u32,
}

impl WriteTo for WriteAccessData {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.index_group)?;
        wtr.write_u32::<LittleEndian>(self.index_offset)?;
        wtr.write_u32::<LittleEndian>(self.write_length)?;
        Ok(())
    }
}

impl ReadFrom for WriteAccessData {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let index_group = read.read_u32::<LittleEndian>()?;
        let index_offset = read.read_u32::<LittleEndian>()?;
        let write_length = read.read_u32::<LittleEndian>()?;

        Ok(WriteAccessData {
            index_group,
            index_offset,
            write_length,
        })
    }
}

impl SumupWriteRequest {
    pub fn new(write_requests: Vec<WriteRequest>) -> Self {
        SumupWriteRequest {
            write_requests,
            command_id: CommandID::Write,
        }
    }

    pub fn request_count(&self) -> u32 {
        self.write_requests.len() as u32
    }

    pub fn expected_response_len(&self) -> u32 {
        self.request_count() * 4
    }
}

impl WriteTo for SumupWriteRequest {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        let mut access_data: Vec<u8> = Vec::new();
        let mut data: Vec<u8> = Vec::new();
        for request in &self.write_requests {
            access_data.write_u32::<LittleEndian>(request.index_group)?;
            access_data.write_u32::<LittleEndian>(request.index_offset)?;
            access_data.write_u32::<LittleEndian>(request.length)?;
            data.write_all(request.data.as_slice())?;
        }
        access_data.append(&mut data);
        wtr.write_all(&access_data)?;
        Ok(())
    }
}

impl ReadFrom for SumupWriteRequest {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let mut data_buf: Vec<u8> = Vec::new();
        let mut write_access: Vec<WriteAccessData> = Vec::new();

        //Read all bytes and get the total length
        read.read_to_end(&mut data_buf)?;
        let total_data_len = data_buf.len() as u32;
        let mut access_data_length: u32 = 0;
        let mut data_length: u32 = 0;
        let mut data_buf = data_buf.as_slice();

        //Get the access data bytes
        for _ in 0..total_data_len / 12 {
            let access_data = WriteAccessData::read_from(&mut data_buf)?;
            data_length += access_data.write_length;
            write_access.push(access_data);
            access_data_length += 12;
            if (total_data_len - data_length - access_data_length) == 0 {
                break;
            }
        }

        //Get the actual data/value bytes and create ReadWriteRequests
        let mut write_requests: Vec<WriteRequest> = Vec::new();
        for access in write_access {
            let mut buf = vec![0; access.write_length as usize];
            data_buf.read_exact(&mut buf)?;
            write_requests.push(WriteRequest::new(
                access.index_group,
                access.index_offset,
                buf,
            ));
        }
        Ok(SumupWriteRequest::new(write_requests))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sumup_read_write_request_write_to_test() {
        let mut rw_vec: Vec<ReadWriteRequest> = Vec::new();
        let data: u32 = 111111;
        let data: Vec<u8> = data.to_le_bytes().to_vec();
        let rw_1 = ReadWriteRequest::new(259, 33, 4, data);
        rw_vec.push(rw_1);

        let data: u64 = 222222;
        let data: Vec<u8> = data.to_le_bytes().to_vec();
        let rw_2 = ReadWriteRequest::new(260, 22, 4, data);
        rw_vec.push(rw_2);

        let mut buffer: Vec<u8> = Vec::new();
        SumupReadWriteRequest::new(rw_vec)
            .write_to(&mut buffer)
            .unwrap();

        let compare = vec![
            3, 1, 0, 0, 33, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 1, 0, 0, 22, 0, 0, 0, 4, 0, 0, 0,
            8, 0, 0, 0, 7, 178, 1, 0, 14, 100, 3, 0, 0, 0, 0, 0,
        ];
        assert_eq!(buffer, compare);
    }

    #[test]
    fn sumup_read_write_request_read_from_test() {
        let read_data = vec![
            3, 1, 0, 0, 33, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 4, 1, 0, 0, 22, 0, 0, 0, 4, 0, 0, 0,
            8, 0, 0, 0, 7, 178, 1, 0, 14, 100, 3, 0, 0, 0, 0, 0,
        ];

        let sum_read_write_request =
            SumupReadWriteRequest::read_from(&mut read_data.as_slice()).unwrap();

        let mut rw_vec: Vec<ReadWriteRequest> = Vec::new();
        let data: u32 = 111111;
        let data: Vec<u8> = data.to_le_bytes().to_vec();
        let rw_1 = ReadWriteRequest::new(259, 33, 4, data);
        rw_vec.push(rw_1);

        let data: u64 = 222222;
        let data: Vec<u8> = data.to_le_bytes().to_vec();
        let rw_2 = ReadWriteRequest::new(260, 22, 4, data);
        rw_vec.push(rw_2);

        let compare = SumupReadWriteRequest::new(rw_vec);

        assert_eq!(
            sum_read_write_request, compare,
            "comparing sum_read_write_request failed"
        );
    }

    #[test]
    fn sumup_read_request_write_to_test() {
        let mut r_vec: Vec<ReadRequest> = Vec::new();
        let r_1 = ReadRequest::new(259, 33, 4);
        r_vec.push(r_1);

        let r_2 = ReadRequest::new(260, 22, 4);
        r_vec.push(r_2);

        let mut buffer: Vec<u8> = Vec::new();
        SumupReadRequest::new(r_vec).write_to(&mut buffer).unwrap();

        let compare = vec![
            3, 1, 0, 0, 33, 0, 0, 0, 4, 0, 0, 0, 4, 1, 0, 0, 22, 0, 0, 0, 4, 0, 0, 0,
        ];
        assert_eq!(buffer, compare);
    }

    #[test]
    fn sumup_read_request_read_from_test() {
        let read_data = vec![
            3, 1, 0, 0, 33, 0, 0, 0, 4, 0, 0, 0, 4, 1, 0, 0, 22, 0, 0, 0, 4, 0, 0, 0,
        ];

        let sum_read_request = SumupReadRequest::read_from(&mut read_data.as_slice()).unwrap();

        let mut r_vec: Vec<ReadRequest> = Vec::new();
        let rw_1 = ReadRequest::new(259, 33, 4);
        r_vec.push(rw_1);

        let rw_2 = ReadRequest::new(260, 22, 4);
        r_vec.push(rw_2);

        let compare = SumupReadRequest::new(r_vec);

        assert_eq!(
            sum_read_request, compare,
            "comparing sum_read_write_request failed"
        );

        assert_eq!(
            sum_read_request.command_id,
            CommandID::Read,
            "Wrong command ID"
        );

        for request in &sum_read_request.read_requests {
            assert_eq!(request.command_id, CommandID::Read, "wrong command id");
            assert_eq!(request.length, 4, "Wrong length");
        }
        assert_eq!(
            sum_read_request.read_requests[0].index_group, 259,
            "Wrong index group"
        );
        assert_eq!(
            sum_read_request.read_requests[0].index_offset, 33,
            "Wrong index offset"
        );
        assert_eq!(
            sum_read_request.read_requests[1].index_group, 260,
            "Wrong index group"
        );
        assert_eq!(
            sum_read_request.read_requests[1].index_offset, 22,
            "Wrong index offset"
        );
    }

    #[test]
    fn sumup_write_request_write_to_test() {
        let mut rw_vec: Vec<WriteRequest> = Vec::new();
        let data: u32 = 111111;
        let data: Vec<u8> = data.to_le_bytes().to_vec();
        let rw_1 = WriteRequest::new(259, 33, data);
        rw_vec.push(rw_1);

        let data: u64 = 222222;
        let data: Vec<u8> = data.to_le_bytes().to_vec();
        let rw_2 = WriteRequest::new(260, 22, data);
        rw_vec.push(rw_2);

        let mut buffer: Vec<u8> = Vec::new();
        SumupWriteRequest::new(rw_vec)
            .write_to(&mut buffer)
            .unwrap();

        let compare = vec![
            3, 1, 0, 0, 33, 0, 0, 0, 4, 0, 0, 0, 4, 1, 0, 0, 22, 0, 0, 0, 8, 0, 0, 0, 7, 178, 1, 0,
            14, 100, 3, 0, 0, 0, 0, 0,
        ];
        assert_eq!(buffer, compare);
    }

    #[test]
    fn sumup_write_request_read_from_test() {
        let read_data = vec![
            3, 1, 0, 0, 33, 0, 0, 0, 4, 0, 0, 0, 4, 1, 0, 0, 22, 0, 0, 0, 8, 0, 0, 0, 7, 178, 1, 0,
            14, 100, 3, 0, 0, 0, 0, 0,
        ];

        let sum_write_request = SumupWriteRequest::read_from(&mut read_data.as_slice()).unwrap();

        let mut rw_vec: Vec<WriteRequest> = Vec::new();
        let data: u32 = 111111;
        let data: Vec<u8> = data.to_le_bytes().to_vec();
        let rw_1 = WriteRequest::new(259, 33, data);
        rw_vec.push(rw_1);

        let data: u64 = 222222;
        let data: Vec<u8> = data.to_le_bytes().to_vec();
        let rw_2 = WriteRequest::new(260, 22, data);
        rw_vec.push(rw_2);

        let compare = SumupWriteRequest::new(rw_vec);

        assert_eq!(
            sum_write_request, compare,
            "comparing sum_read_write_request failed"
        );
    }
}
