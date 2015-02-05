#![allow(non_upper_case_globals)]
#![allow(unstable)]
#[macro_use] extern crate rustc_bitflags;

//#[crate_type = "lib"]

extern crate libc;
extern crate collections;

use libc::size_t;
use std::ffi::CString;
use std::string::String;
use std::str;
use std::vec::Vec;

pub use types:: {
    // main types
    JackNframesT,
    JackPositionT,
    JackPositionBitsT,
    JackTimeT,
    JackUuidT,
    JackNativeThreadT,

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

    JackMidiDataT,
    JackMidiEvent,
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
pub static JACK_DEFAULT_MIDI_TYPE:&'static str = "8 bit raw midi";

pub type JackProcessCallback<T> = fn(JackNframesT, *mut T)->isize;

pub type JackDefaultAudioSampleT = f32;

#[link(name = "jack")]
extern {
    // core
    fn jack_client_open(name: *const libc::c_char, options: types::JackOptions, status: &types::JackStatus) -> *mut jack_client_t;
    fn jack_client_close(client: *mut jack_client_t) -> libc::c_int;
    fn jack_client_name_size() -> libc::c_int;
    fn jack_get_client_name(client: *mut jack_client_t) -> *const libc::c_char;
    fn jack_get_uuid_for_client_name(client: *mut jack_client_t, name: *const libc::c_char) -> *const libc::c_char;
    fn jack_get_client_name_by_uuid(client: *mut jack_client_t, uuid: *const libc::c_char) -> *const libc::c_char;
    fn jack_activate(client: *mut jack_client_t) -> libc::c_int;
    fn jack_deactivate(client: *mut jack_client_t) -> libc::c_int;
    fn jack_get_sample_rate(client: *mut jack_client_t) -> libc::c_uint;
    fn jack_set_process_callback(client: *mut jack_client_t, callback: JackProcessCallback<libc::c_void>, arg: *const libc::c_void) -> libc::c_int;
    fn jack_connect(client: *mut jack_client_t, source_port: *const libc::c_char, destination_port: *const libc::c_char) -> libc::c_int;
    fn jack_disconnect(client: *mut jack_client_t, source_port: *const libc::c_char, destination_port: *const libc::c_char) -> libc::c_int;
    fn jack_is_realtime(client: *mut jack_client_t) -> libc::c_int;
    fn jack_free(ptr: *mut libc::c_void) -> libc::c_void;
    fn jack_client_thread_id(arg1: *mut jack_client_t) -> JackNativeThreadT;
    fn jack_cycle_wait(client: *mut jack_client_t) -> JackNframesT;
    fn jack_cycle_signal(client: *mut jack_client_t, status: ::libc::c_int);

    // port ops with client
    fn jack_port_request_monitor_by_name(client: *mut jack_client_t,
                                         port_name: *const ::libc::c_char,
                                         onoff: ::libc::c_int) -> ::libc::c_int;
    fn jack_port_disconnect(client: *mut jack_client_t, port: *mut jack_port_t) -> ::libc::c_int;

    // ports
    fn jack_port_register(client: *mut jack_client_t,
                          port_name: *const libc::c_char, port_type: *const libc::c_char,
                          flags: types::JackPortFlags, buffer_size: libc::c_ulong) -> *mut jack_port_t;
    fn jack_port_unregister(client: *mut jack_client_t, port: *mut jack_port_t) -> libc::c_int;
    fn jack_port_get_buffer(port: *mut jack_port_t,  nframes: JackNframesT) -> *mut libc::c_void;
    fn jack_port_name(port: *mut jack_port_t) -> *const libc::c_char;
    fn jack_port_set_name(port: *mut jack_port_t, port_name: *const ::libc::c_char) -> ::libc::c_int;
    fn jack_port_set_alias(port: *mut jack_port_t, port_alias: *const ::libc::c_char) -> ::libc::c_int;
    fn jack_port_unset_alias(port: *mut jack_port_t, port_alias: *const ::libc::c_char) -> ::libc::c_int;
    fn jack_port_uuid(port: *mut jack_port_t) -> JackUuidT;
    fn jack_port_short_name(port: *mut jack_port_t) -> *const libc::c_char;
    fn jack_port_flags(port: *mut jack_port_t) -> JackPortFlags;
    fn jack_port_type(port: *mut jack_port_t) -> *const libc::c_char;
    fn jack_port_type_size() -> libc::c_int;
    fn jack_port_is_mine(client: *mut jack_client_t, port: *mut jack_port_t) -> libc::c_int;
    fn jack_port_connected(port: *mut jack_port_t) -> libc::c_int;
    fn jack_port_connected_to(port: *mut jack_port_t, port_name: *const libc::c_char) -> libc::c_int;
    fn jack_port_get_connections(port: *mut jack_port_t) -> *mut*mut libc::c_char;
    fn jack_port_get_all_connections(client: *const jack_client_t, port: *const jack_port_t) -> *mut*mut libc::c_char;
    fn jack_port_get_aliases(port: *const jack_port_t, aliases: *const*mut ::libc::c_char) -> ::libc::c_int;
    fn jack_port_name_size() -> ::libc::c_int;
    fn jack_port_request_monitor(port: *mut jack_port_t, onoff: ::libc::c_int) -> ::libc::c_int;
    fn jack_port_ensure_monitor(port: *mut jack_port_t, onoff: ::libc::c_int) -> ::libc::c_int;
    fn jack_port_monitoring_input(port: *mut jack_port_t) -> ::libc::c_int;

    // transport
    fn jack_transport_query(client: *mut jack_client_t, pos: *mut JackPositionT) -> JackTransportState;

    // midi
    fn jack_midi_get_event_count(port_buffer: *mut ::libc::c_void) -> JackNframesT;
    fn jack_midi_event_get(event: *mut JackMidiEvent, port_buffer: *mut ::libc::c_void, event_index: u32) -> ::libc::c_int;
    fn jack_midi_clear_buffer(port_buffer: *mut ::libc::c_void);
    fn jack_midi_event_reserve(port_buffer: *mut ::libc::c_void, time: JackNframesT, data_size: size_t) -> *mut JackMidiDataT;
    fn jack_midi_max_event_size(port_buffer: *mut ::libc::c_void) -> ::libc::size_t;
    //fn jack_midi_event_write(port_buffer: *mut ::libc::c_void, time: JackNframesT, data: *const JackMidiDataT, data_size: size_t) -> ::libc::c_int;
    fn jack_midi_get_lost_event_count(port_buffer: *mut ::libc::c_void) -> libc::types::common::c99::uint32_t;
}


pub struct JackClient {
    client: *mut jack_client_t,
    status: JackStatus,
}

impl Copy for JackClient {}
unsafe impl Send for JackClient {}
unsafe impl Sync for JackClient {}

impl JackClient {
    pub fn name_size() -> isize {
        unsafe {
            jack_client_name_size() as isize
        }
    }

    pub fn open(name: &str, options: types::JackOptions) -> JackClient {
        unsafe {
            let status = JackNullStatus;
            let innerclient = jack_client_open(CString::from_slice(name.as_bytes()).as_ptr(),options,&status);
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

    pub fn get_name(&self) -> &str {
        unsafe {
            let name = jack_get_client_name(self.client);
            let lname = std::mem::copy_lifetime(self,&name);
            let slice = std::ffi::c_str_to_bytes(lname);
            str::from_utf8(slice).unwrap()
        }
    }

    pub fn get_uuid_for_name(&self, name: &str) -> Option<&str> {
        unsafe {
            let uuid = jack_get_uuid_for_client_name(self.client,CString::from_slice(name.as_bytes()).as_ptr());
            if uuid.is_null() {
                None
            }
            else {
                let luuid = std::mem::copy_lifetime(self,&uuid);
                let slice = std::ffi::c_str_to_bytes(luuid);
                Some(str::from_utf8(slice).unwrap())
            }
        }
    }

    pub fn get_name_for_uuid(&self, uuid: &str) -> Option<&str> {
        unsafe {
            let name = jack_get_client_name_by_uuid(self.client,CString::from_slice(uuid.as_bytes()).as_ptr());
            if name.is_null() {
                None
            }
            else {
                let lname = std::mem::copy_lifetime(self,&name);
                let slice = std::ffi::c_str_to_bytes(lname);
                Some(str::from_utf8(slice).unwrap())
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

    pub fn thread_id(&self) -> JackNativeThreadT {
        unsafe { jack_client_thread_id(self.client) }
    }

    pub fn cycle_wait(&self) -> JackNframesT {
        unsafe { jack_cycle_wait(self.client) }
    }

    pub fn cycle_signal(&self, exit_thread: bool) {
        let statint = if exit_thread { 1 } else { 0 };
        unsafe { jack_cycle_signal(self.client,statint) }
    }

    pub fn register_port(&self, port_name: &str, port_type: &str, flags: JackPortFlags, buffer_size: u64) -> JackPort {
        unsafe {
            let port = jack_port_register(self.client,
                                          CString::from_slice(port_name.as_bytes()).as_ptr(),
                                          CString::from_slice(port_type.as_bytes()).as_ptr(),
                                          flags,
                                          buffer_size);
            JackPort { port: port }
        }
    }

    pub fn port_get_all_connections(&self, port: JackPort) -> Vec<String> {
        let mut vec = Vec::new();
        unsafe {
            let conns = jack_port_get_all_connections(self.client as *const jack_client_t,
                                                      port.port as *const jack_port_t);
            if !conns.is_null() {
                let mut idx = 0;
                while !((*(conns.offset(idx))).is_null()) {
                    let s = *(conns.offset(idx)) as *const i8;
                    let slice = std::ffi::c_str_to_bytes(&s);
                    vec.push(String::from_str(str::from_utf8(slice).unwrap()));
                    idx += 1;
                }
                jack_free(conns as *mut libc::c_void);
            }
        }
        vec
    }

    pub fn unregister_port(&self, port: &JackPort) -> bool {
        unsafe {
            jack_port_unregister(self.client,port.port) == 0
        }
    }

    pub fn disconnect_port(&self, port: &JackPort) -> bool {
        unsafe {
            jack_port_disconnect(self.client,port.port) == 0
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
                                   CString::from_slice(source_port.as_bytes()).as_ptr(),
                                   CString::from_slice(destination_port.as_bytes()).as_ptr());
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
                            CString::from_slice(source_port.as_bytes()).as_ptr(),
                            CString::from_slice(destination_port.as_bytes()).as_ptr()) == 0
        }
    }

    pub fn is_realtime(&self) -> bool {
        unsafe {
            jack_is_realtime(self.client) == 1
        }
    }

    pub fn request_monitor_by_name(&self, port_name: &str, on: bool) -> bool {
        let onoff = if on { 1 } else { 0 };
        unsafe {
            jack_port_request_monitor_by_name(self.client,CString::from_slice(port_name.as_bytes()).as_ptr(),onoff) == 0
        }
    }

}

pub struct JackPort {
    port: *mut jack_port_t,
}

impl Copy for JackPort {}
unsafe impl Send for JackPort {}
unsafe impl Sync for JackPort {}

impl JackPort {
    pub fn name_size() -> i32 {
        unsafe { jack_port_name_size() }
    }

    pub fn name(&self) -> &str {
        unsafe {
            let name = jack_port_name(self.port);
            let lname = std::mem::copy_lifetime(self,&name);
            let slice = std::ffi::c_str_to_bytes(lname);
            str::from_utf8(slice).unwrap()
        }
    }

    pub fn uuid(&self) -> JackUuidT {
        unsafe {
            jack_port_uuid(self.port)
        }
    }

    pub fn short_name(&self) -> &str {
        unsafe {
            let name = jack_port_short_name(self.port);
            let lname = std::mem::copy_lifetime(self,&name);
            let slice = std::ffi::c_str_to_bytes(lname);
            str::from_utf8(slice).unwrap()
        }
    }

    pub fn set_name(&self, name: &str) -> bool {
        unsafe {
            jack_port_set_name(self.port, CString::from_slice(name.as_bytes()).as_ptr()) == 0
        }
    }

    pub fn set_alias(&self, alias: &str) -> bool {
        unsafe {
            jack_port_set_alias(self.port, CString::from_slice(alias.as_bytes()).as_ptr()) == 0
        }
    }

    pub fn unset_alias(&self, alias: &str) -> bool {
        unsafe {
            jack_port_unset_alias(self.port, CString::from_slice(alias.as_bytes()).as_ptr()) == 0
        }
    }

    pub fn get_aliases(&self) -> Vec<&str> {
        unsafe {
            let ps = jack_port_name_size() as usize;
            let mut al1 = Vec::with_capacity(ps);
            let mut al2 = Vec::with_capacity(ps);
            let mut jack_as = [0 as *mut libc::c_char;2];
            jack_as[0] = al1.as_mut_slice().as_mut_ptr();
            jack_as[1] = al2.as_mut_slice().as_mut_ptr();
            let mut ret = Vec::with_capacity(2);
            let acnt = jack_port_get_aliases(self.port as *const jack_port_t,jack_as.as_slice().as_ptr());
            if acnt > 0 {
                let s = jack_as[0] as *const i8;
                let ls = std::mem::copy_lifetime(self,&s);
                let slice = std::ffi::c_str_to_bytes(ls);
                ret.push(str::from_utf8(slice).unwrap());
            }
            if acnt > 1 {
                let s = jack_as[1] as *const i8;
                let ls = std::mem::copy_lifetime(self,&s);
                let slice = std::ffi::c_str_to_bytes(ls);
                ret.push(str::from_utf8(slice).unwrap());
            }
            ret
        }
    }

    pub fn flags(&self) -> JackPortFlags {
        unsafe {
            jack_port_flags(self.port)
        }
    }

    pub fn get_type(&self) -> &str { // ugly name, but have to avoid type keyword
        unsafe {
            let tname = jack_port_type(self.port);
            let ltname = std::mem::copy_lifetime(self,&tname);
            let slice = std::ffi::c_str_to_bytes(ltname);
            str::from_utf8(slice).unwrap()
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
            !(jack_port_connected_to(self.port,CString::from_slice(port.as_bytes()).as_ptr()) == 0)
        }
    }

    pub fn get_buffer<T>(&self, nframes: JackNframesT) -> *mut T {
        unsafe {
            jack_port_get_buffer(self.port,nframes) as *mut T
        }
    }

    pub fn get_vec_buffer<T>(&self, nframes: JackNframesT) -> Vec<T> {
        let buf:* mut T = self.get_buffer(nframes);
        unsafe {
            Vec::from_raw_buf(buf as *const T,nframes as usize)
        }
    }

    pub fn get_midi_buffer(&self, nframes: JackNframesT) -> MidiBuffer {
        MidiBuffer {
            buffer: unsafe { jack_port_get_buffer(self.port, nframes) },
        }
    }

    pub fn get_connections(&self) -> Vec<String> {
        let mut vec = Vec::new();
        unsafe {
            let conns = jack_port_get_connections(self.port);
            if !conns.is_null() {
                let mut idx = 0;
                while !((*(conns.offset(idx))).is_null()) {
                    let s = *(conns.offset(idx)) as *const i8;
                    let slice = std::ffi::c_str_to_bytes(&s);
                    vec.push(String::from_str(str::from_utf8(slice).unwrap()));
                    idx += 1;
                }
                jack_free(conns as *mut libc::c_void);
            }
        }
        vec
    }

    pub fn get_all_connections(&self, client: JackClient) -> Vec<String> {
        let mut vec = Vec::new();
        unsafe {
            let conns = jack_port_get_all_connections(client.client as *const jack_client_t,
                                                      self.port as *const jack_port_t);
            if !conns.is_null() {
                let mut idx = 0;
                while !((*(conns.offset(idx))).is_null()) {
                    let s = *(conns.offset(idx)) as *const i8;
                    let slice = std::ffi::c_str_to_bytes(&s);
                    vec.push(String::from_str(str::from_utf8(slice).unwrap()));
                    idx += 1;
                }
                jack_free(conns as *mut libc::c_void);
            }
        }
        vec
    }

    /// Request a port turn monitoring on or off.  On if on==true.
    pub fn request_monitor(&self, on:bool) -> bool {
        let onoff = if on { 1 } else { 0 };
        unsafe { jack_port_request_monitor(self.port,onoff) == 0 }
    }

    pub fn ensure_monitor(&self, on:bool) -> bool {
        let onoff = if on { 1 } else { 0 };
        unsafe { jack_port_ensure_monitor(self.port,onoff) == 0 }
    }

    pub fn monitoring_input(&self) -> bool {
        unsafe { jack_port_monitoring_input(self.port) != 0 }
    }
}

pub struct MidiBuffer {
    buffer: *mut libc::c_void,
}

impl Copy for MidiBuffer {}

impl MidiBuffer {
    /// Reserve event on this buffer at time with size data_size
    pub fn reserve_event(&self, time: JackNframesT, data_size: size_t) -> ReservedMidiEvent {
        let resbuf = unsafe {jack_midi_event_reserve(self.buffer,time,data_size)};
        ReservedMidiEvent {
            buffer: resbuf,
            len: data_size,
        }
    }

    pub fn clear_buffer(&self) {
        unsafe {
            jack_midi_clear_buffer(self.buffer);
        }
    }

    pub fn get_event_count(&self) -> JackNframesT {
        unsafe { jack_midi_get_event_count(self.buffer) }
    }

    pub fn get_lost_event_count(&self) -> u32 {
        unsafe { jack_midi_get_lost_event_count(self.buffer) as u32 }
    }

    pub fn get_midi_event(&self, event_index: u32) -> Option<JackMidiEvent> {
        unsafe {
            let mut ret:JackMidiEvent = std::mem::zeroed();
            if jack_midi_event_get(&mut ret,self.buffer,event_index) != 0 {
                None
            } else {
                Some(ret)
            }
        }
    }

    pub fn max_event_size(&self) -> size_t {
        unsafe {
            jack_midi_max_event_size(self.buffer)
        }
    }
}

pub struct ReservedMidiEvent {
    buffer: *mut JackMidiDataT,
    len: size_t,
}

impl Copy for ReservedMidiEvent {}

impl ReservedMidiEvent {
    pub fn write_data(&self, index: u32, data: JackMidiDataT) {
        if index as u64 >= self.len {
            panic!("Out of bounds trying to write to midi event");
        }
        unsafe {
            let p = self.buffer.offset(index as isize);
            *p = data;
        }
    }

    pub fn write_vec(&self, vec: &Vec<JackMidiDataT>) {
        if vec.len() > self.len as usize {
            panic!("Too much data passed to write_vec");
        }
        unsafe {
            std::ptr::copy_memory(self.buffer,vec.as_ptr(),self.len as usize);
        }
    }
}


