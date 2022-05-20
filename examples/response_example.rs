extern crate ads_proto;
use ads_proto::error::AdsError;
use ads_proto::proto::ams_address::*;
use ads_proto::proto::ams_header::{AmsHeader, AmsTcpHeader};
use ads_proto::proto::proto_traits::*;
use ads_proto::proto::response::*;
use ads_proto::proto::state_flags::StateFlags;
use anyhow;
use std::result::Result;
use std::str::FromStr;

fn main() -> Result<(), anyhow::Error> {
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

    //Now we need to create the ams header
    let ams_header = AmsHeader::new(
        targed_ams_address,
        source_ams_address,
        state_flags,
        invoke_id,
        response,
    );

    //Create AmsTcpHeader for sending
    let ams_tcp_header = AmsTcpHeader::from(ams_header);

    //Create byte buffer to send over TCP/IP
    let mut buffer: Vec<u8> = Vec::new();
    ams_tcp_header.write_to(&mut buffer)?;

    //===========================================================================
    //Read received response data (client)
    let mut recv_ams_tcp_header = AmsTcpHeader::read_from(&mut buffer.as_slice())?;

    //get the response
    let recv_response = recv_ams_tcp_header.ams_header.response()?;

    //handle response
    match recv_response {
        Response::Invalid(r) => panic!("{:?}\n", r),
        Response::Read(r) => println!("Got a read response {:?}\n", r),
        Response::ReadDeviceInfo(r) => {
            println!("Ads Error: {:?}", r.result);
            println!("major_version: {:?}", r.major_version);
            println!("minor_version: {:?}", r.minor_version);
            println!("version_build: {:?}", r.version_build);
            println!("device_name,: {:?}", r.get_device_name(),);
        }
        Response::ReadState(r) => println!("Got a read state response {:?}\n", r),
        Response::ReadWrite(r) => println!("Got a read write response {:?}\n", r),
        Response::Write(r) => println!("Got a write response {:?}\n", r),
        Response::WriteControl(r) => println!("Got a write control response {:?}\n", r),
        Response::AddDeviceNotification(r) => {
            println!("Got a add device notification response {:?}\n", r)
        }
        Response::DeleteDeviceNotification(r) => {
            println!("Got a delete device notification response {:?}\n", r)
        }
        Response::DeviceNotification(r) => println!("Got a device notification response {:?}\n", r),
    }

    Ok(())
}
