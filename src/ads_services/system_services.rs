pub struct AdsService {
    pub index_group: u32,
    pub index_offset_start: u32,
    pub index_offset_end: u32,
}

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
