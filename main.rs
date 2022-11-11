use evdev::{Device, Vec, EventType, Key, InputEventKind};

fn main() {
    let mut device = Device::open("/dev/input/event0").expect(":(");
    let mut stack 'static = Vec::new();
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
                    println!("Event for {k:?}");
                    stack.push(

                },
                _ => (),
            }
        }
    }
}
