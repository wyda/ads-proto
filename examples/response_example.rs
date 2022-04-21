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

    let response = Response::ReadDeviceInfo(ReadDeviceInfoResponse::new(
        AdsError::ErrNoError,
        1,
        2,
        131,
        ReadDeviceInfoResponse::create_device_name_buf("MyDeviceName"),
    ));

    //Create response header
}
