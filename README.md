# ads-proto
### WIP

Implementation of the [Beckhoff ADS protocol](https://download.beckhoff.com/download/document/automation/twincat3/TwinCAT_3_ADS_INTRO_EN.pdf). 

Originally forked from [mattsse/rust-ads](https://github.com/mattsse/rust-ads) 
and now moved from [wyda/rust-ads](https://github.com/wyda/rust-ads) to this separate repository.

This library implements the basic ADS types and you could use it as example to implement your own Ads client or server.

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

...
