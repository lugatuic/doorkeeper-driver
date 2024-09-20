use evdev::{Device, InputEventKind};
use std::env;
use std::io::{self, Write};
use xkbcommon::xkb;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let dev_path = args.get(1).expect("Please specify event device!");
    let mut device = Device::open(dev_path).expect(":(");
    let ev_xkb_offset = 8;
    let xkb_ctx = xkb::Context::new(0);

    let xkb_keymap = xkb::Keymap::new_from_names(&xkb_ctx, "", "", "", "", None, 0)
        .expect("Couldn't get keymap :(");

    let mut xkb_state = xkb::State::new(&xkb_keymap);

    device.grab ().expect ("couldnt grab :(");

    loop {
        let fes = device.fetch_events().expect("No events :(");

        for ev in fes {
            let k = match ev.kind() {
                InputEventKind::Key(x) => x,
                _ => continue,
            };

            let up_down = if ev.value() == 1 {
                xkb::KeyDirection::Down
            } else {
                xkb::KeyDirection::Up
            };

            let kode = (k.code() + ev_xkb_offset) as u32;

            let str = xkb_state.key_get_utf8(kode);

            if ev.value() == 2 && !xkb_keymap.key_repeats(kode) {
                //                 eprintln!("Ignoring repeat...");
                continue;
            }
            _ = xkb_state.update_key(kode, up_down);

            if ev.value() == 1 {
                print!("{str}");
                io::stdout ().flush ().unwrap ();
            }
        }
    }

}
