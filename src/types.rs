
use libc::size_t;
use std::ptr::RawPtr;

pub type JackNframesT = u32;
pub type JackTimeT = u64;
pub type JackUuidT = u64;
pub type JackNativeThreadT = ::libc::pthread_t;

#[allow(non_uppercase_statics)]
bitflags!(
    #[repr(C)]
    flags JackOptions: u32 {
        const JackNullOption    = 0x000000,
        const JackNoStartServer = 0x000001,
        const JackUseExactName  = 0x000010,
        const JackServerName    = 0x000100,
        const JackLoadName      = 0x001000,
        const JackLoadInit      = 0x010000,
        const JackSessionID     = 0x100000
    }
)

bitflags!(
    #[repr(C)]
    flags JackStatus: u32 {
        const JackNullStatus = 0x00,
        const JackFailure = 0x01,
        const JackInvalidOption = 0x02,
        const JackNameNotUnique = 0x04,
        const JackServerStarted = 0x08,
        const JackServerFailed = 0x10,
        const JackServerError = 0x20,
        const JackNoSuchClient = 0x40,
        const JackLoadFailure = 0x80,
        const JackInitFailure = 0x100,
        const JackShmFailure = 0x200,
        const JackVersionError = 0x400,
        const JackBackendError = 0x800,
        const JackClientZombie = 0x1000
    }
)

bitflags!(
    #[repr(C)]
    flags JackPortFlags: u32 {
        const JackPortIsInput = 0x1,
        const JackPortIsOutput = 0x2,
        const JackPortIsPhysical = 0x4,
        const JackPortCanMonitor = 0x8,
        const JackPortIsTerminal = 0x10,
    }
)

#[repr(C)]
pub enum JackTransportState {
	  /* the order matters for binary compatibility */
	  JackTransportStopped = 0,	/**< Transport halted */
	  JackTransportRolling = 1,	/**< Transport playing */
	  JackTransportLooping = 2,	/**< For OLD_TRANSPORT, now ignored */
	  /**< Waiting for sync ready */
	  JackTransportStarting = 3,
}

pub type JackUniqueT = u64;
pub type JackPositionBitsT = ::libc::c_uint;

#[repr(C,packed)]
pub struct JackPositionT {
    unique_1: JackUniqueT,
    pub usecs: JackTimeT,
    pub frame_rate: JackNframesT,
    pub frame: JackNframesT,
    pub valid: JackPositionBitsT,
    pub bar: i32,
    pub beat: i32,
    pub tick: i32,
    pub bar_start_tick: ::libc::c_double,
    pub beats_per_bar: ::libc::c_float,
    pub beat_type: ::libc::c_float,
    pub ticks_per_beat: ::libc::c_double,
    pub beats_per_minute: ::libc::c_double,
    pub frame_time: ::libc::c_double,
    pub next_time: ::libc::c_double,
    pub bbt_offset: JackNframesT,
    pub audio_frames_per_video_frame: ::libc::c_float,
    pub video_offset: JackNframesT,
    pub padding: [i32, ..7u],
    unique_2: JackUniqueT,
}

// midi types

pub type JackMidiDataT = ::libc::c_uchar;

#[repr(C)]
pub struct JackMidiEvent {
    pub time: JackNframesT,
    pub size: size_t,
    buffer: *mut JackMidiDataT,
}

impl JackMidiEvent {
    pub fn read_data(&self, index: u32) -> JackMidiDataT {
        if index as size_t >= self.size {
            panic!("Out of bounds trying to read midi event");
        }
        unsafe {
            *(RawPtr::offset(self.buffer,index as int))
        }
    }
}
