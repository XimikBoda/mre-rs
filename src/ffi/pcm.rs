use core::ffi::c_void;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcmEvent {
    None = 0,
    DataRequest,
    DataNotification,
    End,
    Error,
    DecoderUnsupport,
    Repeated,
    Terminated,
    LedOn,
    LedOff,
    VibratorOn,
    VibratorOff,
    BacklightOn,
    BacklightOff,
    ExtendedEvent,
    ReadError,
    UpdateDur,
    StopTimeUp,
    DemoTimeUp,
    BufferUnderflow,
    ReadyToPlay,
    DataRefill,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcmStatus {
    Success = 200,
    Fail,
    Reentry,
    NotInitialized,
    BadFormat,
    BadParameter,
    BadCommand,
    NoHandler,
    UnsupportedChannel,
    UnsupportedFreq,
    UnsupportedType,
    UnsupportedOperation,
    SeekFail,
    SeekEof,
    ReadFail,
    WriteFail,
    DiskFull,
    MergeTypeMismatch,
    FileIncomplete,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcmChannel {
    None = 0,
    Main,
    Sub,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcmBufferMode {
    FreeSpaceMode = 0,
    DataConsumeMode,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PcmParam {
    pub is_stereo: u8,
    pub bit_per_sample: u8,
    pub sample_freq: u16,
    pub force_voice: u8,
}


pub type PcmHandleCb = extern "C" fn(handle: *mut Pcm, event: PcmEvent);

#[allow(non_snake_case)]
#[repr(C)]
pub struct Pcm {
    pub unknown: [u8; 0x78],

    pub SetBuffer: extern "C" fn(hdl: *mut Pcm, buffer: *mut u8, buf_len: u32),
    pub GetWriteBuffer: extern "C" fn(hdl: *mut Pcm, buffer: *mut *mut u8, buf_len: *mut u32),
    pub GetReadBuffer: extern "C" fn(hdl: *mut Pcm, buffer: *mut *mut u8, buf_len: *mut u32),
    pub WriteDataDone: extern "C" fn(hdl: *mut Pcm, len: u32),
    pub FinishWriteData: extern "C" fn(hdl: *mut Pcm),
    pub ResetMediaBuf: extern "C" fn(hdl: *mut Pcm),
    pub ReadDataDone: extern "C" fn(hdl: *mut Pcm, len: u32),
    pub DataFinished: extern "C" fn(hdl: *mut Pcm),
    
    pub SetStoreFlag: extern "C" fn(hdl: *mut Pcm, f_store_last_file_offset: u8),
    pub StoreFileOffset: extern "C" fn(hdl: *mut Pcm),
    pub SetFileOffset: extern "C" fn(hdl: *mut Pcm, cur_offset: u32),
    pub GetFileOffset: extern "C" fn(hdl: *mut Pcm) -> u32,
    
    pub GetFreeSpace: extern "C" fn(hdl: *mut Pcm) -> i32,
    pub GetDataCount: extern "C" fn(hdl: *mut Pcm) -> i32,
    
    pub SetLevel: extern "C" fn(hdl: *mut Pcm, level: u8) -> PcmStatus,
    pub GetLevel: extern "C" fn(hdl: *mut Pcm) -> u8,
    pub SetStartTime: extern "C" fn(hdl: *mut Pcm, ms_start_time: u32) -> PcmStatus,
    pub SetStopTime: extern "C" fn(hdl: *mut Pcm, ms_stop_time: u32) -> PcmStatus,
    pub GetCurrentTime: extern "C" fn(hdl: *mut Pcm) -> u32,
    pub GetTotalDuration: extern "C" fn(hdl: *mut Pcm) -> u32,
    
    pub BuildCache: extern "C" fn(hdl: *mut Pcm, get_dur_last_ret: *mut PcmStatus, progress: *mut u32, limit_frame_number: u8),
    pub SetCacheTbl: extern "C" fn(hdl: *mut Pcm, ptr: *mut u8, cache_size: u32), // Змінено usize на cache_size
    pub GetCacheDuration: extern "C" fn(hdl: *mut Pcm) -> u32,
    
    pub SelectChannel: extern "C" fn(hdl: *mut Pcm, channel: PcmChannel),
    pub ReachValidRegion: extern "C" fn(hdl: *mut Pcm) -> PcmStatus,
    pub SetUserData: extern "C" fn(hdl: *mut Pcm, app_data: *mut c_void),
    pub GetUserData: extern "C" fn(hdl: *mut Pcm, app_data: *mut *mut c_void),
    
    pub Trim: extern "C" fn(hdl: *mut Pcm) -> PcmStatus,
    pub SetBufferInternal: extern "C" fn(hdl: *mut Pcm, size: u32),
    pub FreeBufferInternal: extern "C" fn(hdl: *mut Pcm),

    pub Play: extern "C" fn(hdl: *mut Pcm) -> PcmStatus,
    pub Record: extern "C" fn(hdl: *mut Pcm) -> PcmStatus,
    pub Stop: extern "C" fn(hdl: *mut Pcm) -> PcmStatus,
    pub Pause: extern "C" fn(hdl: *mut Pcm) -> PcmStatus,
    pub Resume: extern "C" fn(hdl: *mut Pcm) -> PcmStatus,
    pub Process: extern "C" fn(hdl: *mut Pcm, event: PcmEvent) -> PcmEvent,
    pub Close: extern "C" fn(hdl: *mut Pcm) -> PcmStatus,

    pub unknown2: [u8; 16],
    pub SetDataRequestThreshold: extern "C" fn(hdl: *mut Pcm, mode: PcmBufferMode, threshold: u32, param: *mut c_void),
}

pub type PcmOpenFn = extern "C" fn(handle: PcmHandleCb, param: *mut PcmParam) -> *mut Pcm;

static MAGIC_BYTES: &[u8; 12] = &[
    0x70, 0xB5, 0x06, 0x00, 0x0D, 0x00, 0x00, 0x22, 0x11, 0x00, 0x1C, 0x20
];

pub unsafe fn find_pcm_open() -> Option<PcmOpenFn> {
    unsafe {
        let get_sym_fn = crate::entry::SYSTEM_GET_SYM_ENTRY?;

        let base_addr = (get_sym_fn as usize) & 0xFF00_0000;

        for i in (0..0x100_0000).step_by(4) {
            let current_ptr = (base_addr + i) as *const u8;

            let memory_slice = core::slice::from_raw_parts(current_ptr, 12);

            if memory_slice == MAGIC_BYTES {
                let func_addr = (current_ptr as usize) | 1;
                
                let func: PcmOpenFn = core::mem::transmute(func_addr);
                return Some(func);
            }
        }

        None
    }
}