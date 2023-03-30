# ads-proto

Implementation of the [Beckhoff ADS protocol](https://download.beckhoff.com/download/document/automation/twincat3/TwinCAT_3_ADS_INTRO_EN.pdf). 

Implements the ADS types needed to create a complete AMS header including payload.

The following commands are implemented:
- Read device info
- read
- write
- read state
- write control
- add device notification
- delete device notification
- device notification
- read write

Additional implementations for commands:
- sum up request -> bundles multiple requests together
- sum up response -> bundles multiple responses together

## Docu
Build docu with cargo doc --open
## Examples
examples/request_example.rs 
examples/response_example.rs 
examples/request_to_response.rs 
...