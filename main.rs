use evdev::{Device, EventType, InputEventKind, Key};

fn main() {
    let mut device = Device::open("/dev/input/event0").expect(":(");
    // let mut stack = Vec::new();
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
                        println!("Key {k:?} pressed");
                    };
                }
                _ => (),
            }
        }
    }
}
