extern crate ads_proto;

use ads_proto::error::AdsError;
use ads_proto::proto::ams_address::*;
use ads_proto::proto::ams_header::{AmsHeader, AmsTcpHeader};
use ads_proto::proto::proto_traits::*;
use ads_proto::proto::response::*;
use ads_proto::proto::state_flags::StateFlags;
use std::str::FromStr;

fn main() {
    //creating a response (server/router)

    //We need a targed ams address (from request header)
    let targed_ams_address = match AmsAddress::from_str("192.168.1.2.1.1:851") {
        Ok(a) => a,
        Err(e) => panic!("{}", e),
    };

    //and the source ams address (from server/router)
    let source_ams_address = AmsAddress::new(AmsNetId::from([192, 168, 1, 3, 1, 1]), 851);

    //we crate a default state flag for a response.
    //default for the response -> resonse=true, ads_command=true, netproto=TCP
    let state_flags = StateFlags::resp_default();

    //The invoke key has been sent with request and needs to be mirrored
    let invoke_id = 123321; //from request

    //create a response
    let response = Response::ReadDeviceInfo(ReadDeviceInfoResponse::new(
        AdsError::ErrNoError,
        1,
        2,
        131,
        ReadDeviceInfoResponse::create_device_name_buf("MyDeviceName"),
    ));
}
