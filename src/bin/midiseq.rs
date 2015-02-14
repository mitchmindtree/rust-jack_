#![feature(collections,core,io,env,rustc_private,std_misc)]

extern crate collections;
extern crate getopts;
extern crate jack;


use jack::{JackNframesT,JackClient};
use std::env::args;
use std::old_io::timer;
use std::str::FromStr;
use std::time::duration::Duration;

fn print_usage() {
	  println!("usage: midiseq name nsamp [startindex note nsamp] ...... [startindex note nsamp]");
	  println!(" eg: jack_midiseq Sequencer 24000 0 60 8000 12000 63 8000");
	  println!(" will play a 1/2 sec loop (if srate is 48khz) with a c4 note at the start of the loop");
	  println!(" that lasts for 12000 samples, then a d4# that starts at 1/4 sec that lasts for 800 samples");
}

struct Note {
    freq: u8,
    start: JackNframesT,
    length: JackNframesT,
}

struct CallbackData {
    notes: Vec<Note>,
    loop_nsamp: JackNframesT,
    loop_index: JackNframesT,
    port: jack::JackPort,
}

fn process(nframes: JackNframesT, data:* mut CallbackData) -> isize {
    let cbd = unsafe { &mut *data };
    let midi_buf = cbd.port.get_midi_buffer(nframes);
    midi_buf.clear_buffer();

    for i in range(0,nframes) {
        for note in cbd.notes.iter() {
            if note.start == cbd.loop_index {
                let event = midi_buf.reserve_event(i,3);
                event.write_data(0,0x90); // note on
                event.write_data(1,note.freq);
                event.write_data(2,64); // velocity
            }
            else if note.start + note.length == cbd.loop_index {
                let event = midi_buf.reserve_event(i,3);
                event.write_data(0,0x80); // note off
                event.write_data(1,note.freq);
                event.write_data(2,64); // velocity
            }
        }
        cbd.loop_index =
            if cbd.loop_index + 1 >= cbd.loop_nsamp {
                0
            } else {
                cbd.loop_index + 1
            }
    }
    0
}


fn get_nframes_arg(arg: &collections::string::String) -> JackNframesT {
    FromStr::from_str(arg.as_slice()).unwrap()
}

fn main() {
    let mut argsi = args();
    argsi.next(); // strip off program name
    let mut args: Vec<String> = Vec::with_capacity(argsi.size_hint().0);
    for a in argsi { args.push(a); }

    if args.len() < 6 || (args.len()-3)%3 != 0 {
        print_usage();
        return;
    }

    let client = JackClient::open(args[1].as_slice(), jack::JackNullOption);
    let outport = client.register_port("out",jack::JACK_DEFAULT_MIDI_TYPE, jack::JackPortIsOutput, 0);

    let num_notes = (args.len()-3)/3;
    let mut notes = Vec::with_capacity(num_notes);

     for i in range(0,num_notes) {
         let start = get_nframes_arg(&args[3 + 3*i]);
         let freq:u8 = FromStr::from_str(args[4 + 3*i].as_slice()).unwrap();
         let length = get_nframes_arg(&args[5 + 3*i]);
         notes.push(Note {
             freq: freq,
             start: start,
             length: length,
         });
     }

    let mut cbdata = CallbackData {
        notes: notes,
        loop_nsamp: get_nframes_arg(&args[2]),
        loop_index: 0,
        port: outport,
    };

    client.set_process_callback(process,&mut cbdata);

    if !client.activate() {
        println!("can't activate")
    }

    loop {
        timer::sleep(Duration::minutes(1));
    }
}

