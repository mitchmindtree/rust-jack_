extern crate getopts;
extern crate jack;


use jack::{JackNframesT,JackClient};
use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use std::io::timer;
use std::time::duration::Duration;

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("Usage: {} [options]", program);
    println!("\u0020-f --frequency\tFrequency of beep (in Hz)\n\
              \u0020-A --amplitude\tMaximum application (between 0 and 1)\n\
              \u0020-D --duration\tDuration of beep (in ms)\n\
              \u0020-a --attack\tAttack (in percent of duration)\n\
              \u0020-d --decay\tDecay (in percent of duration)\n\
              \u0020-t --transport\tTransport aware\n\
              \u0020-b --bpm\tBeats per minute\n\
              \u0020-h --help\tUsage")
}


struct CallbackData {
    wavetable: Vec<f32>,
    offset: JackNframesT,
    client: jack::JackClient,
    port: jack::JackPort,
    transport_aware: bool,
}

unsafe fn process_silence(nframes: JackNframesT, data:&mut CallbackData) {
    let buf = (*data).port.get_buffer(nframes);
    std::ptr::set_memory(buf,0,nframes as uint);
}

unsafe fn process_audio(nframes: JackNframesT, data:&mut CallbackData) {
    let cbd = &mut *data;
    let buf = cbd.port.get_buffer(nframes);
    let wave_len = cbd.wavetable.len() as JackNframesT;
    let mut frames_left = nframes;

    while wave_len - cbd.offset < frames_left {
        let src = &(cbd.wavetable[cbd.offset as uint]) as *const f32;
        std::ptr::copy_memory(buf.offset((nframes-frames_left) as int),src,(wave_len - cbd.offset) as uint);
        frames_left -= wave_len - cbd.offset;
        cbd.offset = 0;
    }

    if frames_left > 0 {
        let src = &(cbd.wavetable[cbd.offset as uint]) as *const f32;
        std::ptr::copy_memory(buf.offset((nframes-frames_left) as int),src,frames_left as uint);
        cbd.offset += frames_left;
    }

    cbd.offset %= wave_len;
}

fn process(nframes: JackNframesT, data:* mut CallbackData) -> int {
    unsafe {
        let cbd = &mut *data;
        if (*data).transport_aware {
            let (state,pos) = cbd.client.query_transport();
            match state {
                jack::JackTransportRolling => {} // fall though and process_audio below
                _ => {
                    process_silence(nframes,cbd);
                    return 0;
                }
            }
            cbd.offset = pos.frame % cbd.wavetable.len() as u32;
        }
        process_audio(nframes,cbd);
    }
    0
}

fn get_numeric_arg<T: PartialOrd + std::from_str::FromStr>
    (matches: &getopts::Matches,
     opt: &str,
     default: Option<T>,
     min: Option<T>,
     max: Option<T>) -> T
{
    match matches.opt_str(opt) {
        Some(d) => {
            match from_str::<T>(d.as_slice()) {
                Some(v) => {
                    if (min.is_some() && v < min.unwrap()) ||
                       (max.is_some() && v > max.unwrap()) {
                           fail!("Invalid argument for option {}: {}",opt,d)
                       }
                    v
                }
                None => { fail!("Invalid argument for option {}: {}",opt,d) }
            }
        }
        None => {
            match default {
                Some(d) => { d }
                None => { fail!("Required argument not specified: {}",opt) }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();
    let opts = [
        optopt("a", "attack", "Attack (in percent of duration)", ""),
        optopt("A", "amplitude", "Amplitude of beep", ""),
        optopt("b", "bpm", "Bpm of beep", ""),
        optopt("d", "decay", "Decay (in percent of duration)", ""),
        optopt("D", "duration", "Duration of beep", ""),
        optopt("f", "frequency", "Frequency of beep", ""),
        optflag("t", "transport", "Transport aware"),
        optflag("h", "help", "Print this help menu")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }

    let bpm = get_numeric_arg(&matches,"b",None,Some(0_u32),None);
    let duration = get_numeric_arg(&matches,"D",Some(100_u32),Some(10_u32),None);
    let freq = get_numeric_arg(&matches,"f",Some(880_u32),Some(0_u32),None);
    let max_amp = get_numeric_arg(&matches,"A",Some(0.5_f32),Some(0_f32),Some(1_f32));
    let attack_percent = get_numeric_arg(&matches,"a",Some(1_u32),Some(0_u32),Some(100_u32));
    let decay_percent = get_numeric_arg(&matches,"d",Some(1_u32),Some(0_u32),Some(100_u32));
    let transport_aware = matches.opt_present("t");

    println!("Playing at bpm {}",bpm)

    let client = JackClient::open("metro", jack::JackNoStartServer);
    let outport = client.register_port("metro_port", jack::JACK_DEFAULT_AUDIO_TYPE, jack::JackPortIsOutput, 0);

    let sample_rate = client.sample_rate();

    let wave_length = 60 * sample_rate / bpm;
    let tone_length = sample_rate * duration / 1000;
	  let attack_length = tone_length * attack_percent / 100;
	  let decay_length = tone_length * decay_percent / 100;
    let scale:f32 = 2_f32 * 3.14159265358979323846264338327950288_f32 * freq as f32 / sample_rate as f32;

    if tone_length >= wave_length {
        println!("Invalid duration (tone length = {}, wave length = {}",tone_length, wave_length);
        return;
    }

    if attack_length + decay_length > tone_length {
        println!("Invalid attack/decay");
        return;
    }

    let mut wave: Vec<f32> = Vec::with_capacity(wave_length as uint);
    let mut amp: Vec<f32> = Vec::with_capacity(tone_length as uint);

    for i in range(0_u32, attack_length) {
        amp.push(max_amp * i as f32 / attack_length as f32)
    }

	  for _ in range(attack_length, tone_length - decay_length) {
		    amp.push(max_amp);
	  }

	  for i in range(tone_length - decay_length, tone_length) {
		    amp.push(max_amp * (i as f32 - tone_length as f32) / decay_length as f32)
	  }

    for i in range(0_u32, tone_length) {
        wave.push(amp[i as uint] * (scale * i as f32).sin());
    }
    for _ in range(tone_length, wave_length) {
        wave.push(0_f32);
    }

    let mut cbdata = CallbackData { wavetable: wave,
                                    offset: 0,
                                    client: client,
                                    port: outport,
                                    transport_aware: transport_aware };

    client.set_process_callback(process,&mut cbdata);

    if !client.activate() {
        println!("can't activate")
    }

    timer::sleep(Duration::minutes(1));
}
