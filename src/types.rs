pub type JackNframesT = u32;
pub type JackTimeT = u64;

#[repr(C)]
pub enum JackOptions {
    /**
     * Null value to use when no option bits are needed.
     */
    JackNullOption = 0x00,

    /**
     * Do not automatically start the JACK server when it is not
     * already running.  This option is always selected if
     * \$JACK_NO_START_SERVER is defined in the calling process
     * environment.
     */
    JackNoStartServer = 0x01,

    /**
     * Use the exact client name requested.  Otherwise, JACK
     * automatically generates a unique one, if needed.
     */
    JackUseExactName = 0x02,

    /**
     * Open with optional <em>(char *) server_name</em> parameter.
     */
    JackServerName = 0x04,

    /**
     * Load internal client from optional <em>(char *)
     * load_name</em>.  Otherwise use the @a client_name.
     */
    JackLoadName = 0x08,

    /**
     * Pass optional <em>(char *) load_init</em> string to the
     * jack_initialize() entry point of an internal client.
     */
    JackLoadInit = 0x10,

    /**
     * pass a SessionID Token this allows the sessionmanager to identify the client again.
     */
    JackSessionID = 0x20,
}

#[repr(C)]
pub enum JackStatus {
    /** Added for rust, an empty status */
    JackNullStatus = 0x00,

    /**
     * Overall operation failed.
     */
    JackFailure = 0x01,

    /**
     * The operation contained an invalid or unsupported option.
     */
    JackInvalidOption = 0x02,

    /**
     * The desired client name was not unique.  With the @ref
     * JackUseExactName option this situation is fatal.  Otherwise,
     * the name was modified by appending a dash and a two-digit
     * number in the range "-01" to "-99".  The
     * jack_get_client_name() function will return the exact string
     * that was used.  If the specified @a client_name plus these
     * extra characters would be too long, the open fails instead.
     */
    JackNameNotUnique = 0x04,

    /**
     * The JACK server was started as a result of this operation.
     * Otherwise, it was running already.  In either case the caller
     * is now connected to jackd, so there is no race condition.
     * When the server shuts down, the client will find out.
     */
    JackServerStarted = 0x08,

    /**
     * Unable to connect to the JACK server.
     */
    JackServerFailed = 0x10,

    /**
     * Communication error with the JACK server.
     */
    JackServerError = 0x20,

    /**
     * Requested client does not exist.
     */
    JackNoSuchClient = 0x40,

    /**
     * Unable to load internal client
     */
    JackLoadFailure = 0x80,

    /**
     * Unable to initialize client
     */
    JackInitFailure = 0x100,

    /**
     * Unable to access shared memory
     */
    JackShmFailure = 0x200,

    /**
     * Client's protocol version does not match
     */
    JackVersionError = 0x400,

    /*
     * BackendError
     */
    JackBackendError = 0x800,

    /*
     * Client is being shutdown against its will
     */
    JackClientZombie = 0x1000,
}

#[repr(C)]
pub enum JackPortFlags {

    /**
     * if JackPortIsInput is set, then the port can receive
     * data.
     */
    JackPortIsInput = 0x1,

    /**
     * if JackPortIsOutput is set, then data can be read from
     * the port.
     */
     JackPortIsOutput = 0x2,

    /**
     * if JackPortIsPhysical is set, then the port corresponds
     * to some kind of physical I/O connector.
     */
    JackPortIsPhysical = 0x4,

    /**
     * if JackPortCanMonitor is set, then a call to
     * jack_port_request_monitor() makes sense.
     *
     * Precisely what this means is dependent on the client. A typical
     * result of it being called with TRUE as the second argument is
     * that data that would be available from an output port (with
          * JackPortIsPhysical set) is sent to a physical output connector
     * as well, so that it can be heard/seen/whatever.
     *
     * Clients that do not control physical interfaces
     * should never create ports with this bit set.
     */
    JackPortCanMonitor = 0x8,

    /**
     * JackPortIsTerminal means:
     *
     *	for an input port: the data received by the port
     *                    will not be passed on or made
     *		           available at any other port
     *
     * for an output port: the data available at the port
     *                    does not originate from any other port
     *
     * Audio synthesizers, I/O hardware interface clients, HDR
     * systems are examples of clients that would set this flag for
     * their ports.
     */
    JackPortIsTerminal = 0x10,
}

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
