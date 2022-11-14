use evdev::{Device, InputEvent, InputEventKind, Key};
use xkbcommon::xkb;
use std::env;
use std::fmt;
mod driver;

use driver::imp::{get_state, is_shift};
use driver::ShiftState;

fn process_event(ev: InputEvent, xk: &mut xkb::State) -> Option<Key> {
    match ev.kind() {
        InputEventKind::Key(k) => {
            if let Some(shift_state) = get_state(ev.value(), k) {
                if let ShiftState::Shift = shift_state {
                    _ = xk.update_key(xkb::KEY_Shift_L, xkb::KeyDirection::Down);
                    eprintln!("Shift down");
                } else {
                    _ = xk.update_key(xkb::KEY_Shift_L, xkb::KeyDirection::Up);
                    eprintln!("Shift up");
                }
            }
            Some(k)
        }
        _ => None,
    }
}

fn extract_key(ev: InputEvent) -> Option<Key> {
    match ev.kind() {
        InputEventKind::Key(k) => Some(k),
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dev_path = args.get(1).expect("Please specify event device!");
    let mut device = Device::open(dev_path).expect(":(");
    let mut _state = ShiftState::NoShift;
    let ev_xkb_offset = 8;
    let xkb_ctx = xkb::Context::new(0);
    
    let xkb_keymap = xkb::Keymap::new_from_names(&xkb_ctx, "", "", "", "", None, 0)
        .expect("Couldn't get keymap :(");

    let mut xkb_state = xkb::State::new(&xkb_keymap);
   
    loop {
        let fes = device.fetch_events().expect("No events :(");

        for ev in fes {

            let k = match extract_key(ev) {
                Some(x) => x,
                None => continue,
            };

            let up_down = if ev.value() == 1 {
                xkb::KeyDirection::Down
            } else {
                xkb::KeyDirection::Up
            };

            let kode = (k.code() + ev_xkb_offset) as u32;

            _ = xkb_state.update_key(kode, up_down);

            let str = xkb_state.key_get_one_sym(kode);

            println!("Read: {str:?}");
            // if let Some(k) = process_event(ev, &mut xkb_state) {
            //     let kode: u32 = (k.code() + ev_xkb_offset) as u32;
            //     if !is_shift(k) && ev.value() == 1 {
            //         // println!("Key pressed: {k:?}, {}", state);
            //         xkb_state.update_key(kode, xkb::KeyDirection::Down);
            //         let ksym = xkb_state.key_get_syms(kode);
            //         let s = xkb_state.key_get_utf8(kode);
            //         println!("Received: {ksym:?}\n");
            //     } else {
            //         xkb_state.update_key(kode, xkb::KeyDirection::Up);
            //     }
            // }
        }
    }
}
