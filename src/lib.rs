#[comment = "jack bindings"]
//#[crate_type = "lib"]

extern crate libc;
//use libc::size_t;

pub use types:: {
    // main types
    JackNframesT,
    JackPositionT,
    JackTimeT,
    JackPositionBitsT,

    JackOptions,
    // options
    JackNullOption,
    JackNoStartServer,
    JackUseExactName,
    JackServerName,
    JackLoadName,
    JackLoadInit,
    JackSessionID,

    JackStatus,
    // Status Types
    JackNullStatus,
    JackFailure,
    JackInvalidOption,
    JackNameNotUnique,
    JackServerStarted,
    JackServerFailed,
    JackServerError,
    JackNoSuchClient,
    JackLoadFailure,
    JackInitFailure,
    JackShmFailure,
    JackVersionError,
    JackBackendError,
    JackClientZombie,

    JackPortFlags,
    JackPortIsInput,
    JackPortIsOutput,
    JackPortIsPhysical,
    JackPortCanMonitor,
    JackPortIsTerminal,

    JackTransportState,
    //transport states
    JackTransportStopped,
	  JackTransportRolling,
	  JackTransportLooping,
	  JackTransportStarting,
};

mod types;


#[repr(C)]
struct jack_client_t;

#[repr(C)]
struct jack_port_t;

pub static JACK_DEFAULT_AUDIO_TYPE:&'static str = "32 bit float mono audio";

pub type JackProcessCallback<T> = fn(JackNframesT, *mut T)->int;

pub type JackDefaultAudioSampleT = f32;

#[link(name = "jack")]
extern {
    // core
    fn jack_client_open(name: *const libc::c_char, options: types::JackOptions, status: &types::JackStatus) -> *mut jack_client_t;
    fn jack_client_close(client: *mut jack_client_t) -> libc::c_int;
    fn jack_client_name_size() -> libc::c_int;
    fn jack_get_client_name(client: *mut jack_client_t) -> *const libc::c_char;
    fn jack_get_uuid_for_client_name(client: *mut jack_client_t, name: *const libc::c_char) -> *const libc::c_char;
    fn jack_activate(client: *mut jack_client_t) -> libc::c_int;
    fn jack_deactivate(client: *mut jack_client_t) -> libc::c_int;
    fn jack_get_sample_rate(client: *mut jack_client_t) -> libc::c_uint;
    fn jack_set_process_callback(client: *mut jack_client_t, callback: JackProcessCallback<libc::c_void>, arg: *const libc::c_void) -> libc::c_int;

    // ports
    fn jack_port_register(client: *mut jack_client_t,
                          port_name: *const libc::c_char, port_type: *const libc::c_char,
                          flags: libc::c_ulong, buffer_size: libc::c_ulong) -> *mut jack_port_t;
    fn jack_port_name(port: *mut jack_port_t) -> *const libc::c_char;
    fn jack_port_get_buffer(port: *mut jack_port_t,  nframes: JackNframesT) -> *mut libc::c_void;

    // transport
    fn jack_transport_query(client: *mut jack_client_t, pos: *mut JackPositionT) -> JackTransportState;
}


pub struct JackClient {
    client: *mut jack_client_t,
    status: JackStatus,
}

pub struct JackPort {
    port: *mut jack_port_t,
}

impl JackClient {
    pub fn name_size() -> int {
        unsafe {
            jack_client_name_size() as int
        }
    }

    pub fn open(name: &str, options: types::JackOptions) -> JackClient {
        unsafe {
            let status = JackNullStatus;
            let innerclient = jack_client_open(name.to_c_str().as_ptr(),options,&status);
            JackClient { client: innerclient, status: status }
        }
    }

    pub fn close(&self) -> bool {
        unsafe {
            jack_client_close(self.client) == 0
        }
    }

    pub fn status(&self) -> JackStatus {
        self.status
    }

    pub fn get_name(&self) -> String {
        unsafe {
            let name = jack_get_client_name(self.client);
            std::string::raw::from_buf(name as *const u8)
        }
    }

    pub fn get_uuid_for_name(&self, name: &str) -> Option<String> {
        unsafe {
            let uuid = jack_get_uuid_for_client_name(self.client,name.to_c_str().as_ptr());
            if uuid.is_null() {
                None
            }
            else {
                Some(std::string::raw::from_buf(uuid as *const u8))
            }
        }
    }

    pub fn activate(&self) -> bool {
        unsafe {
            jack_activate(self.client) == 0
        }
    }

    pub fn deactivate(&self) -> bool {
        unsafe {
            jack_deactivate(self.client) == 0
        }
    }

    pub fn register_port(&self, port_name: &str, port_type: &str, flags: JackPortFlags, buffer_size: u64) -> JackPort {
        unsafe {
            let port = jack_port_register(self.client,
                                          port_name.to_c_str().as_ptr(),
                                          port_type.to_c_str().as_ptr(),
                                          flags as u64,
                                          buffer_size);
            JackPort { port: port }
        }
    }

    pub fn sample_rate(&self) -> JackNframesT {
        unsafe {
            jack_get_sample_rate(self.client)
        }
    }

    pub fn set_process_callback<T>(&self, callback: JackProcessCallback<T>, arg: *mut T) -> bool {
        unsafe {
            //jack_set_process_callback(self.client,callback as JackProcessCallback<libc::c_void>,arg as *const libc::c_void) == 0
            jack_set_process_callback(self.client,std::mem::transmute(callback),arg as *const libc::c_void) == 0
        }
    }

    pub fn query_transport(&self) -> (JackTransportState, JackPositionT) {
        unsafe {
            let mut pos:JackPositionT = std::mem::zeroed();
            let state = jack_transport_query(self.client,&mut pos);
            (state,pos)
        }
    }

}

impl JackPort {
    pub fn name(&self) -> String {
        unsafe {
            let name = jack_port_name(self.port);
            std::string::raw::from_buf(name as *const u8)
        }
    }

    pub fn get_buffer(&self, nframes: JackNframesT) -> *mut f32 {
        unsafe {
            jack_port_get_buffer(self.port,nframes) as *mut f32
        }
    }
}
