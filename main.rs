use std::env;
use evdev::{Device, InputEvent, InputEventKind, Key};
use std::fmt;
mod driver;

use driver::ShiftState;
use driver::imp::get_state;

fn process_event(ev: InputEvent, state: &mut ShiftState) -> Option<Key> {
    match ev.kind() {
        InputEventKind::Key(k) => {
            *state = get_state(ev.value(), k);
            Some(k)
        }
        _ => None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dev_path = args.get(1).expect("Please specify event device!");
    let mut device = Device::open(dev_path).expect(":(");
    let mut state = ShiftState::NoShift;

    loop {
        let fes = device.fetch_events().expect("No events :(");

        for ev in fes {
            if let Some(k) = process_event(ev, &mut state) {
                println!("Key pressed: {k:?}, {}", state);
            }
        }
    }
}
