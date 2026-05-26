#[allow(non_snake_case)]
use crate::mre_api;

pub const VM_BITSTREAM_SUCCEED: i32 = 0;
pub const VM_BITSTREAM_BUFFER_OVERFLOW: i32 = 8001;
pub const VM_BITSTREAM_BUFFER_UNDERFLOW: i32 = 8002;
pub const VM_BITSTREAM_EVENT_NONE: i32 = 8003;
pub const VM_BITSTREAM_EVENT_DATA_REQUEST: i32 = 8004;
pub const VM_BITSTREAM_EVENT_ERROR: i32 = 8005;
pub const VM_BITSTREAM_RECOVER: i32 = 8006;
pub const VM_BITSTREAM_INTERRUPT: i32 = 8100;
pub const VM_BITSTREAM_INTERRUPT_RESUME: i32 = 8101;

pub const VM_BITSTREAM_ERR_FAILED: i32 = -8001;
pub const VM_BITSTREAM_ERR_INVALID_RESOULTION: i32 = -8002;
pub const VM_BITSTREAM_ERR_UNSUPPORTED_FORMAT: i32 = -8003;
pub const VM_BITSTREAM_ERR_INVALID_BITSTREAM: i32 = -8004;
pub const VM_BITSTREAM_ERR_MEMORY_INSUFFICIENT: i32 = -8005;
pub const VM_BITSTREAM_ERR_INSUFFICIENT_MEMORY: i32 = -8006;
pub const VM_BITSTREAM_ERR_INVALID_FORMAT: i32 = -8007;
pub const VM_BITSTREAM_NOT_SUPPORTED: i32 = -8008;
pub const VM_BITSTREAM_INVALID_PARAMETER: i32 = -8009;

pub const VM_BITSTREAM_FAILED: i32 = -1;


#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum vm_bitstream_codec_type_enum {
    VM_BITSTREAM_CODEC_TYPE_NONE = 0,
    VM_BITSTREAM_CODEC_TYPE_SILENT,
    VM_BITSTREAM_CODEC_TYPE_AMR,
    VM_BITSTREAM_CODEC_TYPE_AMRWB,
    VM_BITSTREAM_CODEC_TYPE_AAC,
    VM_BITSTREAM_CODEC_TYPE_DAF,
    VM_BITSTREAM_CODEC_TYPE_MP4A,
    VM_BITSTREAM_CODEC_TYPE_MP4AG,
    VM_BITSTREAM_CODEC_TYPE_WAV,
    VM_BITSTREAM_CODEC_TYPE_ADPCM,
    VM_BITSTREAM_CODEC_TYPE_PCM,
    VM_BITSTREAM_CODEC_TYPE_TOTAL,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum vm_bitstream_sample_freq_enum {
    VM_BITSTREAM_SAMPLE_FREQ_8000 = 0,
    VM_BITSTREAM_SAMPLE_FREQ_11025,
    VM_BITSTREAM_SAMPLE_FREQ_16000,
    VM_BITSTREAM_SAMPLE_FREQ_22050,
    VM_BITSTREAM_SAMPLE_FREQ_24000,
    VM_BITSTREAM_SAMPLE_FREQ_32000,
    VM_BITSTREAM_SAMPLE_FREQ_44100,
    VM_BITSTREAM_SAMPLE_FREQ_48000,
    VM_BITSTREAM_SAMPLE_FREQ_TOTAL,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_bitstream_audio_cfg_struct {
    pub vm_codec_type: vm_bitstream_codec_type_enum,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_bitstream_pcm_audio_cfg_struct {
    pub vm_codec_type: vm_bitstream_codec_type_enum,
    pub is_stereo: i32,
    pub bit_per_sample: u8,
    pub sample_freq: vm_bitstream_sample_freq_enum,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_bitstream_audio_buffer_status {
    pub total_buf_size: u32,
    pub free_buf_size: u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct vm_bitstream_audio_start_param {
    pub start_time: u32,
    pub volume: u8,
    pub audio_path: u8,
}

#[allow(non_camel_case_types)]
pub type vm_bitstream_audio_result_callback = extern "C" fn(handle: i32, result: i32);

mre_api!(vm_bitstream_audio_open(handle: *mut i32, audio_type: *mut vm_bitstream_audio_cfg_struct, callback: vm_bitstream_audio_result_callback) -> i32 = -1);

mre_api!(vm_bitstream_audio_open_pcm(handle: *mut i32, audio_type: *mut vm_bitstream_pcm_audio_cfg_struct, callback: vm_bitstream_audio_result_callback) -> i32 = -1);

mre_api!(vm_bitstream_audio_finished(handle: i32) -> i32 = -1);
mre_api!(vm_bitstream_audio_close(handle: i32) -> i32 = -1);

mre_api!(vm_bitstream_audio_get_buffer_status(handle: i32, status: *mut vm_bitstream_audio_buffer_status) -> i32 = -1);
mre_api!(vm_bitstream_audio_get_play_time(handle: i32, current_time: *mut u32) -> i32 = -1);

mre_api!(vm_bitstream_audio_put_data(handle: i32, buffer: *const u8, buffer_size: u32, written: *mut u32) -> i32 = -1);

mre_api!(vm_bitstream_audio_start(handle: i32, para: *mut vm_bitstream_audio_start_param) -> i32 = -1);
mre_api!(vm_bitstream_audio_stop(handle: i32) -> i32 = -1);


mre_api!(vm_bitstream_audio_register_interrupt_callback(callback: vm_bitstream_audio_result_callback) -> i32 = -1);
mre_api!(vm_bitstream_audio_clear_interrupt_callback(handle: i32));