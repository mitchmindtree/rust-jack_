#[comment = "jack bindings"]
//#[crate_type = "lib"]

extern crate libc;
extern crate collections;
use std::vec::Vec;
//use libc::size_t;

pub use types:: {
    // main types
    JackNframesT,
    JackPositionT,
    JackPositionBitsT,
    JackTimeT,
    JackUuidT,

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
};

pub use types::JackTransportState:: {
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
    fn jack_connect(client: *mut jack_client_t, source_port: *const libc::c_char, destination_port: *const libc::c_char) -> libc::c_int;
    fn jack_disconnect(client: *mut jack_client_t, source_port: *const libc::c_char, destination_port: *const libc::c_char) -> libc::c_int;
    fn jack_is_realtime(client: *mut jack_client_t) -> libc::c_int;
    fn jack_free(ptr: *mut libc::c_void) -> libc::c_void;

    // ports
    fn jack_port_register(client: *mut jack_client_t,
                          port_name: *const libc::c_char, port_type: *const libc::c_char,
                          flags: types::JackPortFlags, buffer_size: libc::c_ulong) -> *mut jack_port_t;
    fn jack_port_unregister(client: *mut jack_client_t, port: *mut jack_port_t) -> libc::c_int;
    fn jack_port_get_buffer(port: *mut jack_port_t,  nframes: JackNframesT) -> *mut libc::c_void;
    fn jack_port_name(port: *mut jack_port_t) -> *const libc::c_char;
    fn jack_port_uuid(port: *mut jack_port_t) -> JackUuidT;
    fn jack_port_short_name(port: *mut jack_port_t) -> *const libc::c_char;
    fn jack_port_flags(port: *mut jack_port_t) -> JackPortFlags;
    fn jack_port_type(port: *mut jack_port_t) -> *const libc::c_char;
    fn jack_port_type_size() -> libc::c_int;
    fn jack_port_is_mine(client: *mut jack_client_t, port: *mut jack_port_t) -> libc::c_int;
    fn jack_port_connected(port: *mut jack_port_t) -> libc::c_int;
    fn jack_port_connected_to(port: *mut jack_port_t, port_name: *const libc::c_char) -> libc::c_int;
    fn jack_port_get_connections(port: *mut jack_port_t) -> *mut*mut libc::c_char;

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
            collections::string::String::from_raw_buf(name as *const u8)
        }
    }

    pub fn get_uuid_for_name(&self, name: &str) -> Option<String> {
        unsafe {
            let uuid = jack_get_uuid_for_client_name(self.client,name.to_c_str().as_ptr());
            if uuid.is_null() {
                None
            }
            else {
                Some(collections::string::String::from_raw_buf(uuid as *const u8))
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
                                          flags,
                                          buffer_size);
            JackPort { port: port }
        }
    }

    pub fn unregister_port(&self, port: &JackPort) -> bool {
        unsafe {
            jack_port_unregister(self.client,port.port) == 0
        }
    }

    pub fn port_is_mine(&self, port: JackPort) -> bool {
        unsafe {
            !(jack_port_is_mine(self.client,port.port) == 0)
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

    pub fn connect(&self, source_port: &str, destination_port: &str) -> Result<(), String> { // todo: convert to JackError or something like that
        unsafe {
            let res = jack_connect(self.client,
                                   source_port.to_c_str().as_ptr(),
                                   destination_port.to_c_str().as_ptr());
            if res == 0 {
                Ok(())
            }
            else if res == 17 { //EEXIST
                Err("Ports already connected".to_string())
            }
            else {
                Err("Unknown error connecting port".to_string())
            }
        }
    }

    pub fn disconnect(&self, source_port: &str, destination_port: &str) -> bool {
        unsafe {
            jack_disconnect(self.client,
                            source_port.to_c_str().as_ptr(),
                            destination_port.to_c_str().as_ptr()) == 0
        }
    }

    pub fn is_realtime(&self) -> bool {
        unsafe {
            jack_is_realtime(self.client) == 1
        }
    }

}

impl JackPort {
    pub fn name(&self) -> String {
        unsafe {
            let name = jack_port_name(self.port);
            collections::string::String::from_raw_buf(name as *const u8)
        }
    }

    pub fn uuid(&self) -> JackUuidT {
        unsafe {
            jack_port_uuid(self.port)
        }
    }

    pub fn short_name(&self) -> String {
        unsafe {
            let name = jack_port_short_name(self.port);
            collections::string::String::from_raw_buf(name as *const u8)
        }
    }

    pub fn flags(&self) -> JackPortFlags {
        unsafe {
            jack_port_flags(self.port)
        }
    }

    pub fn get_type(&self) -> String { // ugly name, but have to avoid type keyword
        unsafe {
            let tname = jack_port_type(self.port);
            collections::string::String::from_raw_buf(tname as *const u8)
        }
    }

    pub fn type_size() -> i32 {
        unsafe {
            jack_port_type_size() as i32
        }
    }

    pub fn connected(&self) -> i32 {
        unsafe {
            jack_port_connected(self.port)
        }
    }

    pub fn connected_to(&self, port: &str) -> bool {
        unsafe {
            !(jack_port_connected_to(self.port, port.to_c_str().as_ptr()) == 0)
        }
    }

    pub fn get_buffer(&self, nframes: JackNframesT) -> *mut f32 {
        unsafe {
            jack_port_get_buffer(self.port,nframes) as *mut f32
        }
    }

    pub fn get_vec_buffer(&self, nframes: JackNframesT) -> Vec<f32> {
        let buf = self.get_buffer(nframes);
        unsafe {
            Vec::from_raw_buf(buf as *const f32,nframes as uint)
        }
    }

    pub fn get_connections(&self) -> Vec<String> {
        let mut vec = Vec::new();
        unsafe {
            let conns = jack_port_get_connections(self.port);
            if conns.is_not_null() {
                let mut idx = 0;
                while (*(conns.offset(idx))).is_not_null() {
                    vec.push(collections::string::String::from_raw_buf(*(conns.offset(idx)) as *const u8));
                    idx += 1;
                }
                jack_free(conns as *mut libc::c_void);
            }
        }
        vec
    }
}
