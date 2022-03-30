use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum TryIntoError {
    #[error("try_into failed. self is wrong response!")]
    TryIntoResponseFailed,
    #[error("try_into failed. self is wrong request!")]
    TryIntoRequestFailed,
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum AmsAddressError {
    #[error("Failed parsing address from &str")]
    ParseError { source: std::num::ParseIntError },
    #[error("Split length wrong. Length is {}", length)]
    SplitError { length: usize },
    #[error("Supplied address length {}! Expected a length of 6", length)]
    InvalidAddressLength { length: usize },
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum AdsError {
    //Global error codes
    #[error("No error")]
    ErrNoError,
    #[error("Inernal error")]
    ErrInternal,
    #[error("No real-time")]
    ErrNoRTime,
    #[error("Allocation locked – memory error")]
    ErrAllocLockedMem,
    #[error("Mailbox full- the ADS message could not be sent. Reducing the number of ADS messages per cylce will help")]
    ErrInsertMailBox,
    #[error("Wrong HMSG")]
    ErrWrongRecieveHMSG,
    #[error("Target port not found - ADS server is not started or is not reachable")]
    ErrTargetPortNotFound,
    #[error("Error target computer not found - AMS route was not found")]
    ErrTargetMachineNotFound,
    #[error("Unknown command ID")]
    ErrUnknownCmdId,
    #[error("Invalid task ID")]
    ErrBadTaskId,
    #[error("No IO")]
    ErrNoIO,
    #[error("Unknown AMS command")]
    ErrUnknownAmsCmd,
    #[error("Wind32 error")]
    ErrWin32Error,
    #[error("Port not connected")]
    ErrPortNotConnected,
    #[error("Invalid AMS length")]
    ErrInvalidAmsLength,
    #[error("Invalid AMS net ID")]
    ErrInvalidAmsNetId,
    #[error("Installatin level is too low - TwinCat 2 licence error")]
    ErrLowInstLevel,
    #[error("No debuging available")]
    ErrNoDebugingAvailable,
    #[error("Port disabled - TwinCat system service not started")]
    ErrPortDisabled,
    #[error("Port already connected")]
    ErrPortAlreadyConnected,
    #[error("AMS sync wind32 error")]
    ErrAmsSyncW32Error,
    #[error("Ams sync timeout")]
    ErrAmsSyncTimeout,
    #[error("AMS sync error")]
    ErrAmsSyncAmsError,
    #[error("No index map for AMS sysnc available")]
    ErrAmsSyncNoIndexInMap,
    #[error("Invalid AMS port")]
    ErrInvalidAmsPort,
    #[error("No memory")]
    ErrNoMemory,
    #[error("TCP send error")]
    ErrTcpSend,
    #[error("Host unreachable")]
    ErrHostUnreachable,
    #[error("Invalid AMS fragment")]
    ErrInvalidAmsFragment,
    #[error("TLS send error - secure ADS connection failed")]
    ErrTlsSend,
    #[error("Access denied - secure ADS access denied")]
    ErrAccessDenied,
    //Router error codes
    #[error("Locked memory cannot be alocated")]
    RouterErrNoLockedMemory,
    #[error("The router memory size could not be changed")]
    RouterErrResizeMemory,
    #[error("The mailbox has reached the maximum number of possible messages")]
    RouterErrMailboxFull,
    #[error("The debug mailbox has reached the maximum number of possible messages")]
    RouterErrDebugBoxFull,
    #[error("The port type is unknown")]
    RouterErrUnknownPortType,
    #[error("The router is not initialized")]
    RouterErrNotInitialized,
    #[error("The port number is already assigned")]
    RouterErrPortAlreadyInUse,
    #[error("The port is not registered")]
    RouterErrNotRegistered,
    #[error("The maximum number of ports has been reached")]
    RouterErrNoMoreQueues,
    #[error("The port is invalid")]
    RouterErrInvalidPort,
    #[error("The router is not activated")]
    RouterErrNotActivated,
    #[error("The mailbox has reached the maximum number for fragmented messages")]
    RouterErrFragmentBoxFull,
    #[error("A fragment timeout has occured")]
    RouterErrFragmentTimeout,
    #[error("The port is removed")]
    RouterErrToBeRemoved,
    //General ADS error
    #[error("General device error")]
    AdsErrDeviceError,
    #[error("Service is not support by the server")]
    AdsErrDeviceSrvNotSupp,
    #[error("Invalid index group")]
    AdsErrDeviceInvalidGrp,
    #[error("Invalid index  offset")]
    AdsErrDeviceInvalidOffset,
    #[error("Reading or writing not permittet")]
    AdsErrDeviceInvalidAccess,
    #[error("Parameter size not correct")]
    AdsErrDeviceInvalidSize,
    #[error("Invalid data values")]
    AdsErrDeviceInvalidData,
    #[error("Device is not ready to operate")]
    AdsErrDeviceNotReady,
    #[error("Device is busy")]
    AdsErrDeviceBusy,
    #[error(
        "Invalid operating system context. This can result from use of ADS function blocks in different tasks. 
        It may be possible to resolve this through multitasking synchronization in the PLC"
    )]
    AdsErrDeviceInvalidContext,
    #[error("Inusfficient memory")]
    AdsErrDeviceNoMemory,
    #[error("Invalid parameter values")]
    AdsErrDeviceInvalidParm,
    #[error("Not found (files, ...)")]
    AdsErrDeviceNotFound,
    #[error("Syntax error in file or command")]
    AdsErrDeviceSyntax,
    #[error("Objects do not match")]
    AdsErrDeviceIncompatible,
    #[error("Object already exists")]
    AdsErrDeviceExists,
    #[error("Symbol not found")]
    AdsErrDeviceSymbolNotFound,
    #[error("Invalid symbol version. This can occure due to an online change. Crate a new handle")]
    AdsErrDeviceSymbolVersionInvalid,
    #[error("Device (server) is in invalid state")]
    AdsErrDeviceInvalidState,
    #[error("Ads transmode not supported")]
    AdsErrDeviceTransModeNotSupp,
    #[error("Notification handle is invalid")]
    AdsErrDeviceNotifyHndInvalid,
    #[error("Notification client not registered")]
    AdsErrDeviceClientUnknown,
    #[error("No further notification handle available")]
    AdsErrDeviceNoMoreHDLS,
    #[error("Notification size too large")]
    AdsErrDeviceInvalidWatchSize,
    #[error("Device not initialized")]
    AdsErrDeviceNotInit,
    #[error("Device has a timeout")]
    AdsErrDeviceTimeout,
    #[error("Interface query failed")]
    AdsErrDeviceNoInterface,
    #[error("Wrong interface request")]
    AdsErrDeviceInvalidInterface,
    #[error("Class ID is invalid")]
    AdsErrDeviceInvalidClsID,
    #[error("Object ID is invalid")]
    AdsErrDeviceInvalidOBJID,
    #[error("Request pending")]
    AdsErrDevicePending,
    #[error("Request is aborted")]
    AdsErrDeviceAborted,
    #[error("Signal warning")]
    AdsErrDeviceWarning,
    #[error("Invalid array index")]
    AdsErrDeviceInvalidArrayIDX,
    #[error("Symbol not active")]
    AdsErrDeviceSymbolNotActive,
    #[error("Access denied")]
    AdsErrDeviceAccessDenied,
    #[error("Missing license")]
    AdsErrDeviceLicenseNotFound,
    #[error("License expired")]
    AdsErrDeviceLicenseExpired,
    #[error("License exceeded")]
    AdsErrDeviceLicenseExceeded,
    #[error("Invalid license")]
    AdsErrDeviceLicenseInvalid,
    #[error("License problem: System ID is invalid")]
    AdsErrDeviceLicenseSystemID,
    #[error("License not limited in time")]
    AdsErrDeviceLicenseNoTimeLimit,
    #[error("License problem: Time in the future")]
    AdsErrDeviceLicenseFuturReissue,
    #[error("License period too long")]
    AdsErrDeivceLicenseTimeToLong,
    #[error("Exception at system startup")]
    AdsErrDeviceException,
    #[error("License file read twice")]
    AdsErrDeviceLicenseDublicated,
    #[error("Invalid signature")]
    AdsErrDeviceSignatureInvalid,
    #[error("Invalid certificate")]
    AdsErrDeviceCertificateInvalid,
    #[error("Public key not known from OEM")]
    AdsErrDeviceLicenseOemNotFound,
    #[error("License not valid for system ID")]
    AdsErrDeviceLicenseRestricted,
    #[error("Demo license prohibited")]
    AdsErrDeviceLicenseDemoDenied,
    #[error("Invalid function ID")]
    AdsErrDeviceInvalidFncID,
    #[error("Outside the valid range")]
    AdsErrDeviceOutOfRange,
    #[error("Invalid alignment")]
    AdsErrDeviceInvalidAlignment,
    #[error("Invalid platform level")]
    AdsErrDeviceLicensePlatform,
    #[error("Context - forward to passive level")]
    AdsErrDeviceForwardPL,
    #[error("Context -  forward to sispatch level")]
    AdsErrDeviceForwardDL,
    #[error("Contect - forward to real-time")]
    AdsErrDeviceForwardRT,
    #[error("Client error")]
    AdsErrClientError,
    #[error("Service contains an invalid parameter")]
    AdsErrClientInvalidParm,
    #[error("Possing list is empty")]
    AdsErrClientListEmpty,
    #[error("Var connection already in use")]
    AdsErrClientVarUsed,
    #[error("The called ID is already in use")]
    AdsErrClientDublIvokeID,
    #[error(
        "Timeout has occured - the remote terminal is not responding in the specific ADS timeout. 
        The router setting of the remote terminal may be configured incorrectly"
    )]
    AdsErrClientSyncTimeout,
    #[error("Error in win32 subsystem")]
    AdsErrClientW32Error,
    #[error("Invalid client timeout value")]
    AdsErrClientTimeoutInvalid,
    #[error("Port not open")]
    AdsErrClientPortNotOpen,
    #[error("No Ams Address")]
    AdsErrClientNoAmsAddress,
    #[error("Internal error in ADS sync")]
    AdsErrClientSyncInternal,
    #[error("Hash table overflow")]
    AdsErrClientAddHash,
    #[error("Key not found in the table")]
    AdsErrClientRemoveHash,
    #[error("No symbols in the cache")]
    AdsErrClientNoMoreSym,
    #[error("Invalid response received")]
    AdsErrClientSyncResInvalid,
    #[error("Sync port is locked")]
    AdsErrClientSyncPortLocked,
    //RTime error codes
    #[error("Internal error in the real-time system")]
    RtErrInternal,
    #[error("Time value is not valid")]
    RtErrBadTimerPeriods,
    #[error("Task pointer has the invalid value 0 (zero)")]
    RtErrInvalidTaskPtr,
    #[error("Stack pointer has the invalid value 0 (zero)")]
    RtErrInvalidStackPtr,
    #[error("The requested task priority is already assigned")]
    RtErrPrioExists,
    #[error("No free TCB (Task control block) available. The maximum number of TCB's is 64")]
    RtErrNoMoreTCB,
    #[error("No free semaphores available. The maximum number of semaphores is 64")]
    RtErrNoMoreSemas,
    #[error(
        "No free space available in the queue. The maximum number of positions in the queue is 64"
    )]
    RtErrNoMoreQueues,
    #[error("An external synchronization interrupt is already applied")]
    RtErrExtIrqAlreadyDef,
    #[error("No externel sync interrupt applied")]
    RtErrExtIrqNotDef,
    #[error("Application of the external synchronization interrupt has failed")]
    RtErrExtIrgInstallFaild,
    #[error("Call of a service function in the wrong context")]
    RtErrIrqlNotLessOrEqual,
    #[error("Intel VT-x extension not supported")]
    RtErrVmxNotSupported,
    #[error("Intel VT-x extension is not enabled in the BIOS")]
    RtErrVmxDisabled,
    #[error("Missing function in Intel VT-x extension")]
    RtErrVmxControlsMissing,
    #[error("Activation of intel VT-x fails")]
    RtErrVmxEnableFails,
    //TCP Winsock error codes
    #[error("Unknown Ads error code. Possibly a Win32 error code (Winsock)")]
    ErrUnknowAdsError { error_code: u32 },
}

impl From<u32> for AdsError {
    fn from(error_code: u32) -> Self {
        match error_code {
            //Global error codes
            0 => AdsError::ErrNoError,
            1 => AdsError::ErrInternal,
            2 => AdsError::ErrNoRTime,
            3 => AdsError::ErrAllocLockedMem,
            4 => AdsError::ErrInsertMailBox,
            5 => AdsError::ErrWrongRecieveHMSG,
            6 => AdsError::ErrTargetPortNotFound,
            7 => AdsError::ErrTargetMachineNotFound,
            8 => AdsError::ErrUnknownCmdId,
            9 => AdsError::ErrBadTaskId,
            10 => AdsError::ErrNoIO,
            11 => AdsError::ErrUnknownAmsCmd,
            12 => AdsError::ErrWin32Error,
            13 => AdsError::ErrPortNotConnected,
            14 => AdsError::ErrInvalidAmsLength,
            15 => AdsError::ErrInvalidAmsNetId,
            16 => AdsError::ErrLowInstLevel,
            17 => AdsError::ErrNoDebugingAvailable,
            18 => AdsError::ErrPortDisabled,
            19 => AdsError::ErrPortAlreadyConnected,
            20 => AdsError::ErrAmsSyncW32Error,
            21 => AdsError::ErrAmsSyncTimeout,
            22 => AdsError::ErrAmsSyncAmsError,
            23 => AdsError::ErrAmsSyncNoIndexInMap,
            24 => AdsError::ErrInvalidAmsPort,
            25 => AdsError::ErrNoMemory,
            26 => AdsError::ErrTcpSend,
            27 => AdsError::ErrHostUnreachable,
            28 => AdsError::ErrInvalidAmsFragment,
            29 => AdsError::ErrTlsSend,
            30 => AdsError::ErrAccessDenied,
            //Router error codes
            1280 => AdsError::RouterErrNoLockedMemory,
            1281 => AdsError::RouterErrResizeMemory,
            1282 => AdsError::RouterErrMailboxFull,
            1283 => AdsError::RouterErrDebugBoxFull,
            1284 => AdsError::RouterErrUnknownPortType,
            1285 => AdsError::RouterErrNotInitialized,
            1286 => AdsError::RouterErrPortAlreadyInUse,
            1287 => AdsError::RouterErrNotRegistered,
            1288 => AdsError::RouterErrNoMoreQueues,
            1289 => AdsError::RouterErrInvalidPort,
            1290 => AdsError::RouterErrNotActivated,
            1291 => AdsError::RouterErrFragmentBoxFull,
            1292 => AdsError::RouterErrFragmentTimeout,
            1293 => AdsError::RouterErrToBeRemoved,
            //General ADS error
            1792 => AdsError::AdsErrDeviceError,
            1793 => AdsError::AdsErrDeviceSrvNotSupp,
            1794 => AdsError::AdsErrDeviceInvalidGrp,
            1795 => AdsError::AdsErrDeviceInvalidOffset,
            1796 => AdsError::AdsErrDeviceInvalidAccess,
            1797 => AdsError::AdsErrDeviceInvalidSize,
            1798 => AdsError::AdsErrDeviceInvalidData,
            1799 => AdsError::AdsErrDeviceNotReady,
            1800 => AdsError::AdsErrDeviceBusy,
            1801 => AdsError::AdsErrDeviceInvalidContext,
            1802 => AdsError::AdsErrDeviceNoMemory,
            1803 => AdsError::AdsErrDeviceInvalidParm,
            1804 => AdsError::AdsErrDeviceNotFound,
            1805 => AdsError::AdsErrDeviceSyntax,
            1806 => AdsError::AdsErrDeviceIncompatible,
            1807 => AdsError::AdsErrDeviceExists,
            1808 => AdsError::AdsErrDeviceSymbolNotFound,
            1809 => AdsError::AdsErrDeviceSymbolVersionInvalid,
            1810 => AdsError::AdsErrDeviceInvalidState,
            1811 => AdsError::AdsErrDeviceTransModeNotSupp,
            1812 => AdsError::AdsErrDeviceNotifyHndInvalid,
            1813 => AdsError::AdsErrDeviceClientUnknown,
            1814 => AdsError::AdsErrDeviceNoMoreHDLS,
            1815 => AdsError::AdsErrDeviceInvalidWatchSize,
            1816 => AdsError::AdsErrDeviceNotInit,
            1817 => AdsError::AdsErrDeviceTimeout,
            1818 => AdsError::AdsErrDeviceNoInterface,
            1819 => AdsError::AdsErrDeviceInvalidInterface,
            1820 => AdsError::AdsErrDeviceInvalidClsID,
            1821 => AdsError::AdsErrDeviceInvalidOBJID,
            1822 => AdsError::AdsErrDevicePending,
            1823 => AdsError::AdsErrDeviceAborted,
            1824 => AdsError::AdsErrDeviceWarning,
            1825 => AdsError::AdsErrDeviceInvalidArrayIDX,
            1826 => AdsError::AdsErrDeviceSymbolNotActive,
            1827 => AdsError::AdsErrDeviceAccessDenied,
            1828 => AdsError::AdsErrDeviceLicenseNotFound,
            1829 => AdsError::AdsErrDeviceLicenseExpired,
            1830 => AdsError::AdsErrDeviceLicenseExceeded,
            1831 => AdsError::AdsErrDeviceLicenseInvalid,
            1832 => AdsError::AdsErrDeviceLicenseSystemID,
            1833 => AdsError::AdsErrDeviceLicenseNoTimeLimit,
            1834 => AdsError::AdsErrDeviceLicenseFuturReissue,
            1835 => AdsError::AdsErrDeivceLicenseTimeToLong,
            1836 => AdsError::AdsErrDeviceException,
            1837 => AdsError::AdsErrDeviceLicenseDublicated,
            1838 => AdsError::AdsErrDeviceSignatureInvalid,
            1839 => AdsError::AdsErrDeviceCertificateInvalid,
            1840 => AdsError::AdsErrDeviceLicenseOemNotFound,
            1841 => AdsError::AdsErrDeviceLicenseRestricted,
            1842 => AdsError::AdsErrDeviceLicenseDemoDenied,
            1843 => AdsError::AdsErrDeviceInvalidFncID,
            1844 => AdsError::AdsErrDeviceOutOfRange,
            1845 => AdsError::AdsErrDeviceInvalidAlignment,
            1846 => AdsError::AdsErrDeviceLicensePlatform,
            1847 => AdsError::AdsErrDeviceForwardPL,
            1848 => AdsError::AdsErrDeviceForwardDL,
            1849 => AdsError::AdsErrDeviceForwardRT,
            1856 => AdsError::AdsErrClientError,
            1857 => AdsError::AdsErrClientInvalidParm,
            1858 => AdsError::AdsErrClientListEmpty,
            1859 => AdsError::AdsErrClientVarUsed,
            1860 => AdsError::AdsErrClientDublIvokeID,
            1861 => AdsError::AdsErrClientSyncTimeout,
            1862 => AdsError::AdsErrClientW32Error,
            1863 => AdsError::AdsErrClientTimeoutInvalid,
            1864 => AdsError::AdsErrClientPortNotOpen,
            1865 => AdsError::AdsErrClientNoAmsAddress,
            1872 => AdsError::AdsErrClientSyncInternal,
            1873 => AdsError::AdsErrClientAddHash,
            1874 => AdsError::AdsErrClientRemoveHash,
            1875 => AdsError::AdsErrClientNoMoreSym,
            1876 => AdsError::AdsErrClientSyncResInvalid,
            1877 => AdsError::AdsErrClientSyncPortLocked,
            //RTime error codes
            4096 => AdsError::RtErrInternal,
            4097 => AdsError::RtErrBadTimerPeriods,
            4098 => AdsError::RtErrInvalidTaskPtr,
            4099 => AdsError::RtErrInvalidStackPtr,
            4100 => AdsError::RtErrPrioExists,
            4101 => AdsError::RtErrNoMoreTCB,
            4102 => AdsError::RtErrNoMoreSemas,
            4103 => AdsError::RtErrNoMoreQueues,
            4109 => AdsError::RtErrExtIrqAlreadyDef,
            4110 => AdsError::RtErrExtIrqNotDef,
            4111 => AdsError::RtErrExtIrgInstallFaild,
            4112 => AdsError::RtErrIrqlNotLessOrEqual,
            4119 => AdsError::RtErrVmxNotSupported,
            4120 => AdsError::RtErrVmxDisabled,
            4121 => AdsError::RtErrVmxControlsMissing,
            4122 => AdsError::RtErrVmxEnableFails,
            val => AdsError::ErrUnknowAdsError { error_code: val },
        }
    }
}

impl AdsError {
    pub fn as_u32(&self) -> u32 {
        match self {
            //Global error codes
            AdsError::ErrNoError => 0,
            AdsError::ErrInternal => 1,
            AdsError::ErrNoRTime => 2,
            AdsError::ErrAllocLockedMem => 3,
            AdsError::ErrInsertMailBox => 4,
            AdsError::ErrWrongRecieveHMSG => 5,
            AdsError::ErrTargetPortNotFound => 6,
            AdsError::ErrTargetMachineNotFound => 7,
            AdsError::ErrUnknownCmdId => 8,
            AdsError::ErrBadTaskId => 9,
            AdsError::ErrNoIO => 10,
            AdsError::ErrUnknownAmsCmd => 11,
            AdsError::ErrWin32Error => 12,
            AdsError::ErrPortNotConnected => 13,
            AdsError::ErrInvalidAmsLength => 14,
            AdsError::ErrInvalidAmsNetId => 15,
            AdsError::ErrLowInstLevel => 16,
            AdsError::ErrNoDebugingAvailable => 17,
            AdsError::ErrPortDisabled => 18,
            AdsError::ErrPortAlreadyConnected => 19,
            AdsError::ErrAmsSyncW32Error => 20,
            AdsError::ErrAmsSyncTimeout => 21,
            AdsError::ErrAmsSyncAmsError => 22,
            AdsError::ErrAmsSyncNoIndexInMap => 23,
            AdsError::ErrInvalidAmsPort => 24,
            AdsError::ErrNoMemory => 25,
            AdsError::ErrTcpSend => 26,
            AdsError::ErrHostUnreachable => 27,
            AdsError::ErrInvalidAmsFragment => 28,
            AdsError::ErrTlsSend => 29,
            AdsError::ErrAccessDenied => 30,
            //Router error codes
            AdsError::RouterErrNoLockedMemory => 1280,
            AdsError::RouterErrResizeMemory => 1281,
            AdsError::RouterErrMailboxFull => 1282,
            AdsError::RouterErrDebugBoxFull => 1283,
            AdsError::RouterErrUnknownPortType => 1284,
            AdsError::RouterErrNotInitialized => 1285,
            AdsError::RouterErrPortAlreadyInUse => 1286,
            AdsError::RouterErrNotRegistered => 1287,
            AdsError::RouterErrNoMoreQueues => 1288,
            AdsError::RouterErrInvalidPort => 1289,
            AdsError::RouterErrNotActivated => 1290,
            AdsError::RouterErrFragmentBoxFull => 1291,
            AdsError::RouterErrFragmentTimeout => 1292,
            AdsError::RouterErrToBeRemoved => 1293,
            //General ADS error
            AdsError::AdsErrDeviceError => 1792,
            AdsError::AdsErrDeviceSrvNotSupp => 1793,
            AdsError::AdsErrDeviceInvalidGrp => 1794,
            AdsError::AdsErrDeviceInvalidOffset => 1795,
            AdsError::AdsErrDeviceInvalidAccess => 1796,
            AdsError::AdsErrDeviceInvalidSize => 1797,
            AdsError::AdsErrDeviceInvalidData => 1798,
            AdsError::AdsErrDeviceNotReady => 1799,
            AdsError::AdsErrDeviceBusy => 1800,
            AdsError::AdsErrDeviceInvalidContext => 1801,
            AdsError::AdsErrDeviceNoMemory => 1802,
            AdsError::AdsErrDeviceInvalidParm => 1803,
            AdsError::AdsErrDeviceNotFound => 1804,
            AdsError::AdsErrDeviceSyntax => 1805,
            AdsError::AdsErrDeviceIncompatible => 1806,
            AdsError::AdsErrDeviceExists => 1807,
            AdsError::AdsErrDeviceSymbolNotFound => 1808,
            AdsError::AdsErrDeviceSymbolVersionInvalid => 1809,
            AdsError::AdsErrDeviceInvalidState => 1810,
            AdsError::AdsErrDeviceTransModeNotSupp => 1811,
            AdsError::AdsErrDeviceNotifyHndInvalid => 1812,
            AdsError::AdsErrDeviceClientUnknown => 1813,
            AdsError::AdsErrDeviceNoMoreHDLS => 1814,
            AdsError::AdsErrDeviceInvalidWatchSize => 1815,
            AdsError::AdsErrDeviceNotInit => 1816,
            AdsError::AdsErrDeviceTimeout => 1817,
            AdsError::AdsErrDeviceNoInterface => 1818,
            AdsError::AdsErrDeviceInvalidInterface => 1819,
            AdsError::AdsErrDeviceInvalidClsID => 1820,
            AdsError::AdsErrDeviceInvalidOBJID => 1821,
            AdsError::AdsErrDevicePending => 1822,
            AdsError::AdsErrDeviceAborted => 1823,
            AdsError::AdsErrDeviceWarning => 1824,
            AdsError::AdsErrDeviceInvalidArrayIDX => 1825,
            AdsError::AdsErrDeviceSymbolNotActive => 1826,
            AdsError::AdsErrDeviceAccessDenied => 1827,
            AdsError::AdsErrDeviceLicenseNotFound => 1828,
            AdsError::AdsErrDeviceLicenseExpired => 1829,
            AdsError::AdsErrDeviceLicenseExceeded => 1830,
            AdsError::AdsErrDeviceLicenseInvalid => 1831,
            AdsError::AdsErrDeviceLicenseSystemID => 1832,
            AdsError::AdsErrDeviceLicenseNoTimeLimit => 1833,
            AdsError::AdsErrDeviceLicenseFuturReissue => 1834,
            AdsError::AdsErrDeivceLicenseTimeToLong => 1835,
            AdsError::AdsErrDeviceException => 1836,
            AdsError::AdsErrDeviceLicenseDublicated => 1837,
            AdsError::AdsErrDeviceSignatureInvalid => 1838,
            AdsError::AdsErrDeviceCertificateInvalid => 1839,
            AdsError::AdsErrDeviceLicenseOemNotFound => 1840,
            AdsError::AdsErrDeviceLicenseRestricted => 1841,
            AdsError::AdsErrDeviceLicenseDemoDenied => 1842,
            AdsError::AdsErrDeviceInvalidFncID => 1843,
            AdsError::AdsErrDeviceOutOfRange => 1844,
            AdsError::AdsErrDeviceInvalidAlignment => 1845,
            AdsError::AdsErrDeviceLicensePlatform => 1846,
            AdsError::AdsErrDeviceForwardPL => 1847,
            AdsError::AdsErrDeviceForwardDL => 1848,
            AdsError::AdsErrDeviceForwardRT => 1849,
            AdsError::AdsErrClientError => 1856,
            AdsError::AdsErrClientInvalidParm => 1857,
            AdsError::AdsErrClientListEmpty => 1858,
            AdsError::AdsErrClientVarUsed => 1859,
            AdsError::AdsErrClientDublIvokeID => 1860,
            AdsError::AdsErrClientSyncTimeout => 1861,
            AdsError::AdsErrClientW32Error => 1862,
            AdsError::AdsErrClientTimeoutInvalid => 1863,
            AdsError::AdsErrClientPortNotOpen => 1864,
            AdsError::AdsErrClientNoAmsAddress => 1865,
            AdsError::AdsErrClientSyncInternal => 1872,
            AdsError::AdsErrClientAddHash => 1873,
            AdsError::AdsErrClientRemoveHash => 1874,
            AdsError::AdsErrClientNoMoreSym => 1875,
            AdsError::AdsErrClientSyncResInvalid => 1876,
            AdsError::AdsErrClientSyncPortLocked => 1877,
            //RTime error codes
            AdsError::RtErrInternal => 4096,
            AdsError::RtErrBadTimerPeriods => 4097,
            AdsError::RtErrInvalidTaskPtr => 4098,
            AdsError::RtErrInvalidStackPtr => 4099,
            AdsError::RtErrPrioExists => 4100,
            AdsError::RtErrNoMoreTCB => 4101,
            AdsError::RtErrNoMoreSemas => 4102,
            AdsError::RtErrNoMoreQueues => 4103,
            AdsError::RtErrExtIrqAlreadyDef => 4109,
            AdsError::RtErrExtIrqNotDef => 4110,
            AdsError::RtErrExtIrgInstallFaild => 4111,
            AdsError::RtErrIrqlNotLessOrEqual => 4112,
            AdsError::RtErrVmxNotSupported => 4119,
            AdsError::RtErrVmxDisabled => 4120,
            AdsError::RtErrVmxControlsMissing => 4121,
            AdsError::RtErrVmxEnableFails => 4122,
            AdsError::ErrUnknowAdsError { error_code } => *error_code,
        }
    }
}
