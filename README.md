# Door-Driver

## An EVDEV driver written in Rust

- The card reader is basically the keyboard, presented in /dev/input/eventX.
- The Rust program hijacks the input device to prevent spurious inputs to the OS.
  (Technically, the raspberry pi would just be sitting at the TTY login prompt. Don't want
  the input to also go there).
- Key press and unpress events are processed with [libxkbcommon](https://xkbcommon.org/)
  to make sure special characters are handled. Example: SHIFT + 3 = #
- The utf8 result from above is printed to STDOUT (flushed at after each character).
