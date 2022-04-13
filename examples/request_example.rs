extern crate ads_proto;

use ads_proto::ads_services::system_services::GET_SYMHANDLE_BY_NAME;
use ads_proto::proto::ams_address::*;
use ads_proto::proto::ams_header::{AmsHeader, AmsTcpHeader};
use ads_proto::proto::proto_traits::*;
use ads_proto::proto::request::*;
use ads_proto::proto::state_flags::StateFlags;
use std::str::FromStr;

fn main() {
    //Example for creating a ams request

    //We need the targed ams address
    let targed_ams_address = match AmsAddress::from_str("192.168.1.2.1.1:851") {
        Ok(a) => a,
        Err(e) => panic!("{}", e),
    };

    //and the source ams address
    let source_ams_address = AmsAddress::new(AmsNetId::from([192, 168, 1, 3, 1, 1]), 851);

    //we crate a default state flag for a request.
    //default for the request -> resonse=false, ads_command=true, netproto=TCP
    let state_flags = StateFlags::req_default();

    //The invoke id can be any u32 number.
    //This invoke key will be returned in the response.
    let invoke_id = 123321;

    //Last but no least we need the request we want to send
    //We want to request a handle for a variable in this example
    let var_name = "Main.SomeVar";
    let request = Request::ReadWrite(ReadWriteRequest::new(
        GET_SYMHANDLE_BY_NAME.index_group,
        GET_SYMHANDLE_BY_NAME.index_offset_start,
        var_name.len() as u32,
        var_name.as_bytes().to_vec(),
    ));

    //Now we have all we need to create the ams header
    let ams_header = AmsHeader::new(
        targed_ams_address,
        source_ams_address,
        state_flags,
        invoke_id,
        request,
    );

    //If we want to send over TCP we need to put this AMS header into a AmsTcpHeader
    let tcp_ams_header = AmsTcpHeader::from(ams_header);

    //Finally create the byte buffer.
    let mut buffer: Vec<u8> = Vec::new();
    tcp_ams_header.write_to(&mut buffer);
}
