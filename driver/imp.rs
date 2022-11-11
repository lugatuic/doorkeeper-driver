use crate::*;

impl fmt::Display for ShiftState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ShiftState::Shift => write!(f, "Shift on"),
            ShiftState::NoShift => write!(f, "Shift Off"),
        }
    }
}
