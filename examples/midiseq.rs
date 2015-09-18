// libjack bindings for Rust
// Copyright (C) 2015 Nick Lanham

// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301
// USA

extern crate jack;


use jack::{JackNframesT,JackClient};
use std::env::args;
use std::str::FromStr;

fn print_usage() {
	  println!("usage: midiseq name nsamp [startindex note nsamp] ...... [startindex note nsamp]");
	  println!(" eg: midiseq Sequencer 24000 0 60 8000 12000 63 8000");
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

    for i in 0..nframes {
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


fn get_nframes_arg(arg: &str) -> JackNframesT {
    FromStr::from_str(arg).unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 6 || (args.len()-3)%3 != 0 {
        print_usage();
        return;
    }

    let client = JackClient::open(&args[1][..], jack::JackNullOption);
    let outport = client.register_port("out",jack::JACK_DEFAULT_MIDI_TYPE, jack::JackPortIsOutput, 0);

    let num_notes = (args.len()-3)/3;
    let mut notes = Vec::with_capacity(num_notes);

     for i in 0..num_notes {
         let start = get_nframes_arg(&args[3 + 3*i]);
         let freq:u8 = FromStr::from_str(&args[4 + 3*i]).unwrap();
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
        std::thread::sleep_ms(60000);
    }
}

