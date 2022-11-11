use evdev::{Device, EventType, InputEventKind, Key};
use std::fmt;

enum State {
    Shift,
    NoShift
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            State::Shift => write!(f, "Shift on"),
            State::NoShift => write!(f, "Shift Off"),
        }
    }
}

fn main() {
    let mut device = Device::open("/dev/input/event0").expect(":(");
    let mut state = State::NoShift;
    loop {
        // let maybe_keypress = device.get_key_state();

        // match maybe_keypress {
        //     Err(e) => println!("Error: {e:?}"),
        //     Ok(x) => println!("{x:?}\n"),
        // }
        let fes = device.fetch_events().expect("No events :(");

        for ev in fes {
            match ev.kind() {
                InputEventKind::Key(k) => {
                    if ev.value() == 1 {
                        if k == Key::KEY_LEFTSHIFT || k == Key::KEY_RIGHTSHIFT {
                            state = State::Shift;
                        }
                        println!("Key pressed: {k:?}, Caps status {}",state);
                    } else if ev.value() == 0 {
                        if k == Key::KEY_LEFTSHIFT || k == Key::KEY_RIGHTSHIFT {
                            state = State::NoShift;
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
