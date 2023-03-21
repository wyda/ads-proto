use crate::error::AdsError;
use crate::proto::proto_traits::{ReadFrom, WriteTo};
use crate::proto::response::{ReadResponse, WriteResponse};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

//Helper struct
#[derive(Debug, Clone, PartialEq, Eq)]
struct AccessData {
    result: u32,
    length: u32,
}

impl WriteTo for AccessData {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        wtr.write_u32::<LittleEndian>(self.result)?;
        wtr.write_u32::<LittleEndian>(self.length)?;
        Ok(())
    }
}

impl ReadFrom for AccessData {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let result = read.read_u32::<LittleEndian>()?;
        let length = read.read_u32::<LittleEndian>()?;

        Ok(AccessData { result, length })
    }
}

///Ads Sumup Read response
///Bundle multiple responses toghether. Add this data to the read write response or parse from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumupReadResponse {
    pub read_responses: Vec<ReadResponse>,
}

impl SumupReadResponse {
    pub fn new(read_responses: Vec<ReadResponse>) -> Self {
        SumupReadResponse { read_responses }
    }
}

impl ReadFrom for SumupReadResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let mut data_buf: Vec<u8> = Vec::new();
        let mut read_access: Vec<AccessData> = Vec::new();

        //Read all bytes to get the total length
        read.read_to_end(&mut data_buf)?;
        let total_data_len = data_buf.len() as u32;
        let mut access_data_length: u32 = 0;
        let mut data_length: u32 = 0;
        let mut data_buf = data_buf.as_slice();

        //Get the access data bytes
        for _ in 0..total_data_len / 8 {
            let access_data = AccessData::read_from(&mut data_buf)?;
            access_data_length += 8;
            data_length += access_data.length;
            read_access.push(access_data);
            if (total_data_len - data_length - access_data_length) == 0 {
                break;
            }
        }

        //Get the actual data/value bytes and create ReadWriteResponses
        let mut read_response: Vec<ReadResponse> = Vec::new();
        for access in read_access {
            let mut buf = vec![0; access.length as usize];
            data_buf.read_exact(&mut buf)?;
            read_response.push(ReadResponse::new(AdsError::from(access.result), buf));
        }
        Ok(SumupReadResponse::new(read_response))
    }
}

impl WriteTo for SumupReadResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        let mut access_data: Vec<u8> = Vec::new();
        let mut data: Vec<u8> = Vec::new();
        for response in &self.read_responses {
            access_data.write_u32::<LittleEndian>(response.result.as_u32())?;
            access_data.write_u32::<LittleEndian>(response.length)?;
            data.write_all(response.data.as_slice())?;
        }
        access_data.append(&mut data);
        wtr.write_all(&access_data)?;
        Ok(())
    }
}

///Ads Sumup Write response
///Bundle multiple responses toghether. Add this data to the read write response or parse from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumupWriteResponse {
    pub write_responses: Vec<WriteResponse>,
}

impl SumupWriteResponse {
    pub fn new(write_responses: Vec<WriteResponse>) -> Self {
        SumupWriteResponse { write_responses }
    }
}

impl ReadFrom for SumupWriteResponse {
    fn read_from<R: Read>(read: &mut R) -> io::Result<Self> {
        let mut data_buf: Vec<u8> = Vec::new();
        read.read_to_end(&mut data_buf)?;
        let mut write_resp: Vec<WriteResponse> = Vec::new();
        let mut buf = data_buf.as_slice();
        for _ in 0..(data_buf.len() / 4) {
            write_resp.push(WriteResponse::read_from(&mut buf)?)
        }
        Ok(SumupWriteResponse::new(write_resp))
    }
}

impl WriteTo for SumupWriteResponse {
    fn write_to<W: Write>(&self, mut wtr: W) -> io::Result<()> {
        for result in &self.write_responses {
            result.write_to(&mut wtr)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sumup_read_write_to_test() {
        let mut response_group: Vec<ReadResponse> = Vec::new();
        let data_1 = vec![1, 0];
        let response_1 = ReadResponse::new(AdsError::ErrNoError, data_1);
        response_group.push(response_1);
        let data_2 = vec![2, 0, 0, 0];
        let response_2 = ReadResponse::new(AdsError::ErrNoError, data_2);
        response_group.push(response_2);
        let data_3 = vec![3, 0, 0, 0, 0, 0, 0, 0];
        let response_3 = ReadResponse::new(AdsError::ErrNoError, data_3);
        response_group.push(response_3);

        let sum_read_write_response = SumupReadResponse::new(response_group);
        let mut buf: Vec<u8> = Vec::new();
        sum_read_write_response.write_to(&mut buf).unwrap();

        #[rustfmt::skip]
        let compare_data = vec![
            0,0,0,0,        //result response 1
            2,0,0,0,        //data length response 1
            0,0,0,0,        //result response 2
            4,0,0,0,        //data length response 1
            0,0,0,0,        //result response 3
            8,0,0,0,        //data length response 1
            1,0,            //data response 1
            2,0,0,0,        //data response 2
            3,0,0,0,0,0,0,0 //data response 3
        ];

        assert_eq!(buf, compare_data);
    }

    #[test]
    fn sumup_read_read_from_test() {
        #[rustfmt::skip]
        let data = vec![
            0,0,0,0,        //result response 1
            2,0,0,0,        //data length response 1
            0,0,0,0,        //result response 2
            4,0,0,0,        //data length response 1
            0,0,0,0,        //result response 3
            8,0,0,0,        //data length response 1
            1,0,            //data response 1
            2,0,0,0,        //data response 2
            3,0,0,0,0,0,0,0 //data response 3
        ];

        let sum_read_response = SumupReadResponse::read_from(&mut data.as_slice()).unwrap();

        let data_1 = vec![1, 0];
        let response_1 = ReadResponse::new(AdsError::ErrNoError, data_1);
        let data_2 = vec![2, 0, 0, 0];
        let response_2 = ReadResponse::new(AdsError::ErrNoError, data_2);
        let data_3 = vec![3, 0, 0, 0, 0, 0, 0, 0];
        let response_3 = ReadResponse::new(AdsError::ErrNoError, data_3);

        assert_eq!(sum_read_response.read_responses[0], response_1);
        assert_eq!(sum_read_response.read_responses[1], response_2);
        assert_eq!(sum_read_response.read_responses[2], response_3);

        let value_1: u16 = sum_read_response.read_responses[0]
            .data
            .as_slice()
            .read_u16::<LittleEndian>()
            .unwrap();
        assert_eq!(value_1, 1);
        assert_eq!(
            sum_read_response.read_responses[0].result,
            AdsError::ErrNoError
        );
        assert_eq!(sum_read_response.read_responses[0].length, 2);
        let value_2: u32 = sum_read_response.read_responses[1]
            .data
            .as_slice()
            .read_u32::<LittleEndian>()
            .unwrap();
        assert_eq!(value_2, 2);
        assert_eq!(
            sum_read_response.read_responses[1].result,
            AdsError::ErrNoError
        );
        assert_eq!(sum_read_response.read_responses[1].length, 4);
        let value_3: u64 = sum_read_response.read_responses[2]
            .data
            .as_slice()
            .read_u64::<LittleEndian>()
            .unwrap();
        assert_eq!(value_3, 3);
        assert_eq!(
            sum_read_response.read_responses[2].result,
            AdsError::ErrNoError
        );
        assert_eq!(sum_read_response.read_responses[2].length, 8);
    }

    #[test]
    fn sumup_write_write_to_test() {
        let mut response_group: Vec<WriteResponse> = Vec::new();
        let response_1 = WriteResponse::new(AdsError::ErrNoError);
        response_group.push(response_1);
        let response_2 = WriteResponse::new(AdsError::AdsErrClientPortNotOpen);
        response_group.push(response_2);
        let response_3 = WriteResponse::new(AdsError::ErrNoError);
        response_group.push(response_3);

        let sum_read_write_response = SumupWriteResponse::new(response_group);
        let mut buf: Vec<u8> = Vec::new();
        sum_read_write_response.write_to(&mut buf).unwrap();

        #[rustfmt::skip]
        let compare_data = vec![
            0,0,0,0,        //AdsError::ErrNoError
            72,7,0,0,       //AdsError::AdsErrClientPortNotOpen
            0,0,0,0,        //AdsError::ErrNoError        
        ];

        assert_eq!(buf, compare_data);
    }

    #[test]
    fn sumup_write_read_from_test() {
        #[rustfmt::skip]
        let data = vec![
            0,0,0,0,        //AdsError::ErrNoError
            72,7,0,0,       //AdsError::AdsErrClientPortNotOpen
            0,0,0,0,        //AdsError::ErrNoError        
        ];

        let sum_write_response = SumupWriteResponse::read_from(&mut data.as_slice()).unwrap();

        let mut response_group: Vec<WriteResponse> = Vec::new();
        let response_1 = WriteResponse::new(AdsError::ErrNoError);
        response_group.push(response_1);
        let response_2 = WriteResponse::new(AdsError::AdsErrClientPortNotOpen);
        response_group.push(response_2);
        let response_3 = WriteResponse::new(AdsError::ErrNoError);
        response_group.push(response_3);
        let compare = SumupWriteResponse::new(response_group);

        assert_eq!(sum_write_response, compare);
    }
}
