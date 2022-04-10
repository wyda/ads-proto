/*!
 * Implementation of the [Beckhoff ADS](https://download.beckhoff.com/download/Document/automation/twincat3/TwinCAT_3_ADS_INTRO_EN.pdf) protocol
*/

///A collection of System Services with index group and index offset
pub mod ads_services;
///contains the ADS error codes and additional error types used in the module proto.
pub mod error;
///contains everything you need to create an [AMS header](proto::ams_header) and it's payload.
pub mod proto;
