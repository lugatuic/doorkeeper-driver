use crate::*;
use evdev::Key;

use driver::ShiftState;

impl fmt::Display for ShiftState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ShiftState::Shift => write!(f, "Shift on"),
            ShiftState::NoShift => write!(f, "Shift Off"),
        }
    }
}

pub fn get_state(input_value: i32, k: Key) -> Option<ShiftState> {
    match (input_value, k) {
        (1, Key::KEY_LEFTSHIFT) |
        (1, Key::KEY_RIGHTSHIFT) => Some(ShiftState::Shift),
        (0, Key::KEY_LEFTSHIFT) |
        (0, Key::KEY_RIGHTSHIFT) => Some(ShiftState::NoShift),
        _ => None,
    }
}
