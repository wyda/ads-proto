extern crate ads_proto;

use ads_proto::ads_services::system_services::GET_SYMHANDLE_BY_NAME;
use ads_proto::proto::ams_address::*;
use ads_proto::proto::ams_header::{AmsHeader, AmsTcpHeader};
use ads_proto::proto::proto_traits::*;
use ads_proto::proto::request::*;
use ads_proto::proto::state_flags::StateFlags;
use std::str::FromStr;

fn main() {
    //Creating a request (client)

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

    //Now we need to create the ams header
    let ams_header = AmsHeader::new(
        targed_ams_address,
        source_ams_address,
        state_flags,
        invoke_id,
        request,
    );

    //If we want to send over TCP we need to put this AMS header into a AmsTcpHeader
    let mut tcp_ams_header = AmsTcpHeader::from(ams_header);
    print!("Our AmsTcpHeader struct:\n{:?}\n", tcp_ams_header);

    //Finally create the byte buffer.
    let mut buffer: Vec<u8> = Vec::new();
    tcp_ams_header
        .write_to(&mut buffer)
        .expect("Failed to write byte buffer!");

    print!(
        "Byte buffer (AmsTcpHeader) with read write request\n{:?}\n",
        buffer
    );

    //We may want to reuse the header and supply a new request
    let mut request_buffer: Vec<u8> = Vec::new();
    Request::ReadState(ReadStateRequest::new())
        .write_to(&mut request_buffer)
        .expect("Failed to write buffer!");
    tcp_ams_header.ams_header.update_data(request_buffer);

    let mut new_buffer: Vec<u8> = Vec::new();
    tcp_ams_header
        .write_to(&mut new_buffer)
        .expect("Failed to write byte buffer!");
    print!(
        "Byte buffer (AmsTcpHeader) with read state request\n{:?}\n",
        new_buffer
    );
    print!("==========================================================================\n");
    //=========================================================

    //Reading a request (server/router)
    let mut tcp_ams_header = AmsTcpHeader::read_from(&mut buffer.as_slice())
        .expect("Failed to read AmsTcpHeader from byte buffer!");

    //Get the requst data
    let request = tcp_ams_header
        .ams_header
        .request()
        .expect("Failed to get request data!");

    //Handle the request
    match request {
        Request::Invalid(_) => panic!(),
        Request::Read(r) => {
            let r: ReadRequest = r.try_into().unwrap();
            print!("var handle is {:?}", r.index_offset);
        }
        Request::ReadDeviceInfo(r) => {
            let _r: ReadDeviceInfoRequest = r.try_into().unwrap();
            print!("recieved a read device info request");
        }
        Request::ReadState(r) => {
            let _r: ReadStateRequest = r.try_into().unwrap();
            print!("recieved a read state request");
        }
        Request::ReadWrite(r) => {
            let r: ReadWriteRequest = r.try_into().unwrap();
            print!("received a read write request...\n");
            print!("Command id  :{:?} \n", r.command_id);
            print!("Data        :{:?} \n", r.data);
            print!("Index group :{:?} \n", r.index_group);
            print!("Index offset:{:?} \n", r.index_offset);
            print!("Read length :{:?} \n", r.read_length);
            print!("Write length:{:?} \n", r.write_length);
        }
        Request::Write(r) => {
            let r: WriteRequest = r.try_into().unwrap();
            print!("recieved a write request");
            print!("data to write -> {:?}", r.data);
        }
        Request::WriteControl(r) => {
            let r: WriteControlRequest = r.try_into().unwrap();
            print!("recieved a write control request:");
            print!("requested ads state is: {:?}", r.ads_state);
        }
        Request::AddDeviceNotification(r) => {
            let r: AddDeviceNotificationRequest = r.try_into().unwrap();
            print!("recieved an add device notification request");
            print!("transition mode is -> {:?}", r.transmission_mode);
        }
        Request::DeleteDeviceNotification(r) => {
            let r: DeleteDeviceNotificationRequest = r.try_into().unwrap();
            print!("recieved a delete device notification request");
            print!("delete notification for handle -> {:?}", r.handle);
        }
        Request::DeviceNotification(r) => {
            let _r: DeviceNotificationRequest = r.try_into().unwrap();
            print!("recieved a device notification (?)");
        }
    }
}
