pub type JackNframesT = u32;
pub type JackTimeT = u64;
pub type JackUuidT = u64;

bitflags!(
    #[repr(C)]
    flags JackOptions: u32 {
        static JackNullOption    = 0x000000,
        static JackNoStartServer = 0x000001,
        static JackUseExactName  = 0x000010,
        static JackServerName    = 0x000100,
        static JackLoadName      = 0x001000,
        static JackLoadInit      = 0x010000,
        static JackSessionID     = 0x100000
    }
)

bitflags!(
    #[repr(C)]
    flags JackStatus: u32 {
        static JackNullStatus = 0x00,
        static JackFailure = 0x01,
        static JackInvalidOption = 0x02,
        static JackNameNotUnique = 0x04,
        static JackServerStarted = 0x08,
        static JackServerFailed = 0x10,
        static JackServerError = 0x20,
        static JackNoSuchClient = 0x40,
        static JackLoadFailure = 0x80,
        static JackInitFailure = 0x100,
        static JackShmFailure = 0x200,
        static JackVersionError = 0x400,
        static JackBackendError = 0x800,
        static JackClientZombie = 0x1000
    }
)

bitflags!(
    #[repr(C)]
    flags JackPortFlags: u32 {
        static JackPortIsInput = 0x1,
        static JackPortIsOutput = 0x2,
        static JackPortIsPhysical = 0x4,
        static JackPortCanMonitor = 0x8,
        static JackPortIsTerminal = 0x10,
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
