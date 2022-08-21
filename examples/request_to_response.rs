extern crate ads_proto;
use ads_proto::ads_services::system_services::GET_SYMHANDLE_BY_NAME;
use ads_proto::error::AdsError;
use ads_proto::proto::ams_address::*;
use ads_proto::proto::ams_header::{AmsHeader, AmsTcpHeader};
use ads_proto::proto::proto_traits::*;
use ads_proto::proto::request::*;
use ads_proto::proto::response::*;
use ads_proto::proto::state_flags::StateFlags;
use anyhow;
use std::result::Result;
use std::str::FromStr;

fn main() -> Result<(), anyhow::Error> {
    //Creating a request (client)

    //AmsAddress source and targed
    let targed_ams_address = match AmsAddress::from_str("192.168.1.2.1.1:851") {
        Ok(a) => a,
        Err(e) => panic!("{}", e),
    };
    let source_ams_address = AmsAddress::new(AmsNetId::from([192, 168, 1, 3, 1, 1]), 851);

    //we crate a default state flag for a request.
    //default for the request -> resonse=false, ads_command=true, netproto=TCP
    let state_flags = StateFlags::req_default();

    //The invoke id can be any u32 number.
    //This invoke key will be returned in the response.
    let invoke_id = 123321;

    //Request a var handle
    let var_name = "Main.SomeVar";
    let request = Request::ReadWrite(ReadWriteRequest::new(
        GET_SYMHANDLE_BY_NAME.index_group,
        GET_SYMHANDLE_BY_NAME.index_offset_start,
        var_name.len() as u32,
        var_name.as_bytes().to_vec(),
    ));

    //Create the ams tcp header and create byte buffer to send over TCP/IP
    let ams_tcp_header = AmsTcpHeader::from(AmsHeader::new(
        targed_ams_address,
        source_ams_address,
        state_flags,
        invoke_id,
        request,
    ));

    let mut buffer: Vec<u8> = Vec::new();
    ams_tcp_header.write_to(&mut buffer)?;
    //=============================================================================

    //Receive the request and responde (server/router)
    let mut recv_ams_tcp_header = AmsTcpHeader::read_from(&mut buffer.as_slice())?;

    //Handle response
    let request = recv_ams_tcp_header.ams_header.request()?;

    let mut send_back_buffer: Vec<u8> = Vec::new();
    match request {
        Request::Invalid(_) => println!("Invalid request received!"),
        Request::Read(r) => println!("Read request received:\n{:?}", r),
        Request::ReadDeviceInfo(r) => println!("Read device info request received:\n{:?}", r),
        Request::ReadState(r) => println!("Read state request received:\n{:?}", r),
        Request::ReadWrite(r) => {
            println!("Read writerequest received:\n{:?}", r);
            //Return the handle for the requested var name. Reuse AmsTcpHeader
            let var_name = String::from_utf8(r.data)?;
            match var_name.as_str() {
                "Main.SomeVar" => {
                    let var_handle: u32 = 123;
                    let response = Response::ReadWrite(ReadWriteResponse::new(
                        AdsError::ErrNoError,
                        var_handle.to_le_bytes().to_vec(),
                    ));

                    //Update ams header with our response
                    recv_ams_tcp_header
                        .ams_header
                        .update_command(response, StateFlags::resp_default())?; //Update AmsHeader with response and state flag
                                                                                //Swap targed and source address for sendig back
                    recv_ams_tcp_header.ams_header.swap_address();
                    //Write to u8 buffer
                    recv_ams_tcp_header.write_to(&mut send_back_buffer)?;
                }
                _ => println!("Unknown var name"),
            }
        }
        Request::Write(r) => println!("Write request received:\n{:?}", r),
        Request::WriteControl(r) => println!("Write control request received:\n{:?}", r),
        Request::AddDeviceNotification(r) => {
            println!("Add device notification request received:\n{:?}", r)
        }
        Request::DeleteDeviceNotification(r) => {
            println!("Delete device notification request received:\n{:?}", r)
        }
        Request::DeviceNotification(r) => println!("Device notification received:\n{:?}", r),
    }
    //============================================================================================================

    //Read response from server (client)
    let mut resp_ams_tcp_header = AmsTcpHeader::read_from(&mut send_back_buffer.as_slice())?;

    //Get the requested handle
    let response: ReadWriteResponse = resp_ams_tcp_header.ams_header.response()?.try_into()?;

    let handle = u32::from_le_bytes(response.data.try_into().expect("Failed to convert to u32"));
    println!(
        "Received handle for var Main.SomeVar... Handle is: {:?}",
        handle
    );

    Ok(())
}
