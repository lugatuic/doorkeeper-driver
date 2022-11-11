use std::env;
use evdev::{Device, InputEventKind, Key};
use std::fmt;
mod driver;

use driver::ShiftState;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dev_path = args.get(1).expect("Please specify event device!");
    let mut device = Device::open(dev_path).expect(":(");
    let mut state = ShiftState::NoShift;

    loop {
        let fes = device.fetch_events().expect("No events :(");

        for ev in fes {
            match ev.kind() {
                InputEventKind::Key(k) => {
                    if ev.value() == 1 {
                        if k == Key::KEY_LEFTSHIFT || k == Key::KEY_RIGHTSHIFT {
                            state = ShiftState::Shift;
                        }
                        println!("Key pressed: {k:?}, Caps status {}",state);
                    } else if ev.value() == 0 {
                        if k == Key::KEY_LEFTSHIFT || k == Key::KEY_RIGHTSHIFT {
                            state = ShiftState::NoShift;
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
