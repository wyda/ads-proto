pub struct AdsService {
    pub index_group: u32,
    pub index_offset_start: u32,
    pub index_offset_end: u32,
}

///Commands for EtherCat Master ADS Port 0xFFFF

///Returns current state of master (UINT16). Following values are returned by this service:
///0x0000: Init State
///0x0002: Pre-Operational State
///0x0003: Bootstrap State
///0x0004: Safe-Operational State
///0x0008: Operational State
pub const GET_ETHERCAT_MASTER_STATE: AdsService = AdsService {
    index_group: 0x00000003,
    index_offset_start: 0x00000100,
    index_offset_end: 0x00000000,
};

///Request State from master.
///Index_Offset is the state to request. Following values are accepted:
///0x0000: Init State
///0x0002: Pre-Operational State
///0x0003: Bootstrap State  
///0x0004: Safe-Operational State
///0x0008: Operational State
pub const REQUEST_ETHERCAT_MASTER_STATE: AdsService = AdsService {
    index_group: 0x00000003,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

///Returns the number of projected slaves (UINT16).
pub const GET_PROJECTED_SLAVES: AdsService = AdsService {
    index_group: 0x00000006,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

/// Returns the fixed addresses of all slaves (UINT16[nSlaves]).
pub const GET_FIXED_ADDRESSES: AdsService = AdsService {
    index_group: 0x00000007,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

///Returns the EtherCAT status and the Link status of all Slaves:
/// {
///BYTE
///EtherCAT state of a slave. The state can adopt one of the following values:
///0x0000: Init State
///0x0002: Pre-Operational State
///0x0003: Bootstrap State
///0x0004: Safe-Operational State
///0x0008: Operational State

///Additionally following bits can be set:
///0x0010: Error State
///0x0020: Invalid VPRS( VendorId, Product Code, RevisionsNo or SerialNo)
///0x0040: Initialization command error

///BYTE
///Link status of an EtherCAT slave. The Link status can consist of an ORing of the following bits:
///0x0000: Link ok.
///0x0001: Link not present
///0x0002: No communication
///0x0004: Link missing
///0x0008: Additional link
///0x0010: Port A
///0x0020: Port B
///0x0040: Port C
///0x0080: Port D

///example: 0x0024 = Missing Link at port B.
///}[nSlaves]
pub const GET_SLAVE_STATUS: AdsService = AdsService {
    index_group: 0x00000009,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

///Returns the EtherCAT status of the EtherCAT slave specified in the index offset. See GET_SLAVE_STATUS for details.
/// Index offset is the fixed address of the slave. Max 65535 slaves.
pub const GET_SLAVE_STATUS_BY_INDEX: AdsService = AdsService {
    index_group: 0x00000009,
    index_offset_start: 0x00000001,
    index_offset_end: 0x0000FFFF,
};

///Request a new state from the selected EtherCAT slave (UINT16).
/// Index offset is the fixed address of the slave. Max 65535 slaves.
/// See GET_ETHERCAT_MASTER_STATE for possible states.
pub const REQUEST_SLAVE_STATE: AdsService = AdsService {
    index_group: 0x00000009,
    index_offset_start: 0x00000001,
    index_offset_end: 0x0000FFFF,
};

/// Returns the CANopen identity object of an EtherCAT slave device.
/// Index offset is the fixed address of the slave. Max 65535 slaves.
/// {
///UINT32 Vendor Id
///UINT32 Product Code
///UINT32 Revision Number
///UINT32 Serial Number
///}
pub const GET_CANOPEN_IDENTITY: AdsService = AdsService {
    index_group: 0x00000011,
    index_offset_start: 0x00000001,
    index_offset_end: 0x0000FFFF,
};

/// Returns the Crc error counters of all slaves.
///{
/// UINT32 
/// Crc error counter of port A +
/// Crc error counter of port B +
/// Crc error counter of port C +
/// Crc error counter of port D +
///}[nSlaves]
pub const GET_SLAVE_CRC_ERROR_COUNTER: AdsService = AdsService {
    index_group: 0x00000012,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

/// Returns the Crc error counters of the EtherCAT slave specified in the index offset.
/// {
/// UINT32 Crc error counter of port A.
/// UINT32 Crc error counter of port B.
/// UINT32 Crc error counter of port C.
/// UINT32 Crc error counter of port D.
/// }
pub const GET_CRC_ERROR_COUNTERS: AdsService = AdsService {
    index_group: 0x00000012,
    index_offset_start: 0x00000001,
    index_offset_end: 0x0000FFFF,
};

/// Returns frame counters and and lost frame counters
/// {
///UINT32 system time
///UINT32 number of cyclic frames sent by master
///UINT32 number of lost cyclic frames
///UINT32 number of acyclic frames sent by master
///UINT32 number of lost acyclic frames
///}
pub const GET_FRAME_LOSTFRAMES_COUNTERS: AdsService = AdsService {
    index_group: 0x0000000C,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

/// Returns frame counters and and lost frame counters
pub const RESET_FRAME_LOSTFRAMES_COUNTERS: AdsService = AdsService {
    index_group: 0x0000000C,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

/// Commands for EtherCat Slaves. Slave Ads Port(0x1 - 0xFFFE)
/// CANopen over EtherCAT
/// 
/// SDO Upload/Download Request. The object is selected with the index offset (UINT8[n]).
/// Index and Subindex of an SDO.
///HIWORD(0xyyyy0000)= index
///LOBYTE(0x000000yy)= subindex
///Example:
///0x1c120001:
///index = 0x1c12
///subindex = 1
/// HIBYTE of LOWORD (0x0000yy00):
///------
///0x01
///Complete Access 
pub const UP_DOWNLOAD_SDO: AdsService = AdsService {
    index_group: 0x0000F302,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

/// Returns the indexes of the list type specified in the index offset. If 0 is passed as index offset the length of each list type is returned.
/// {
/// UINT16
/// list type = 0 :number of list types
/// list type > 0 :list type
///
/// UINT16[n]
/// list type = 0: length of the list type n+1
/// list type > 0: length of the selected list
///}
/// 
/// List type = HIWORD(0xyyyy0000)
/// Example
/// 0x00000000: return length of the indiviual list types
/// 0x00010000: return indexes of all objects
pub const GET_INDEXES_OF_LIST_TYPE: AdsService = AdsService {
    index_group: 0x0000F3FC,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

/// Get SDO info description
/// index = HIWORD(0xyyyy0000)
pub const GET_INFO_DESCRIPTION: AdsService = AdsService {
    index_group: 0x0000F3FD,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

/// Get SDO info entry description
/// index = HIWORD(0xyyyy0000)
/// subindex = LOBYTE of LOWORD
/// (0x000000yy)
/// valueInfo = HIBYTE of LOWORD
/// (0x0000yy00)
pub const GET_SDO_INFO_ENTRY_DESCRIPTION :AdsService = AdsService {
    index_group: 0x0000F3FE,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

/// Servo Drive over EtherCAT
///
/// Upload/Download IDN (UINT8[n])
/// IDN = LOWORD(0x0000yyyy)
///element = LOBYTE of HIWORD
///(0x00yy0000) :
///
///R/W UINT8[n]
///
//Upload/Download IDN 0x01
///Data Status 0x02
///Name (read only) 0x04
///Attribute 0x08
///Unit 0x10
///Minimum 0x20
///Maximum 0x40
//Value 0x80
///	
///Default
///Drive Number = Bits 1-3 of HIBYTE of HIWORD
///(0xy0000000)
///Command Flag = Bit 8 of HIBYTE of HIWORD
///(0xy0000000)
pub const UP_DOWNLOAD_IDN: AdsService = AdsService {
    index_group: 0x0000F420,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

/// commands for PLC Symbol Handling
/// 
///Reqeust var-handle by name.
///Index offset is allways 0
pub const GET_SYMHANDLE_BY_NAME: AdsService = AdsService {
    index_group: 0x0000F003,
    index_offset_start: 0x00000000,
    index_offset_end: 0x00000000,
};

///Read or write to the the var behind the handle requested with GET_SYMHANDLE_BY_NAME
///Index offset is symhandle
pub const READ_WRITE_SYMVAL_BY_HANDLE: AdsService = AdsService {
    index_group: 0x0000F005,
    index_offset_start: 0x00000000,
    index_offset_end: 0xFFFFFFFF,
};

///Releases a symhandle.
///Send Symhandle with the write data
pub const RELEASE_SYMHANDLE: AdsService = AdsService {
    index_group: 0x0000F006,
    index_offset_start: 0x00000000,
    index_offset_end: 0xFFFFFFFF,
};

///UploadSymbols
///Read all symbols from the PLC.
pub const ADSIGRP_SYM_UPLOAD: AdsService = AdsService {
    index_group: 0x0000F00B,
    index_offset_start: 0x00000000,
    index_offset_end: 0xFFFFFFFF,
};

///Symbol Upload Info
///Read the symbol upload info from the PLC. Length of the upload data.
pub const ADSIGRP_SYM_UPLOADINFO: AdsService = AdsService {
    index_group: 0x0000F00C,
    index_offset_start: 0x00000000,
    index_offset_end: 0xFFFFFFFF,
};

/// Index offset = Number of internal sub-commands.
/// Max commands = 500
pub const ADSIGRP_SUMUP_WRITE: AdsService = AdsService {
    index_group: 0x0000F081,
    index_offset_start: 0x00000000,
    index_offset_end: 0xFFFFFFFF,
};

/// Index offset = Number of internal sub-commands.
/// Max commands = 500
pub const ADSIGRP_SUMUP_READEX: AdsService = AdsService {
    index_group: 0x0000F083,
    index_offset_start: 0x00000000,
    index_offset_end: 0xFFFFFFFF,
};

/// Index offset = Number of internal sub-commands.
/// Max commands = 500
pub const ADSIGRP_SUMUP_READWRITE: AdsService = AdsService {
    index_group: 0x0000F082,
    index_offset_start: 0x00000000,
    index_offset_end: 0xFFFFFFFF,
};
