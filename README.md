# Door-Driver

## An EVDEV driver written in Rust

- The card reader is basically the keyboard, presented in /dev/input/eventX.
- The Rust program hijacks the input device to prevent spurious inputs to the OS.
  (Technically, the raspberry pi would just be sitting at the TTY login prompt. Don't want
  the input to also go there).
- Key press and unpress events are processed with [libxkbcommon](https://xkbcommon.org/)
  to make sure special characters are handled. Example: SHIFT + 3 = #
- The utf8 result from above is printed to STDOUT (flushed at after each character).

## Shell script (Submodule)

- This is the legacy card_reader.sh (now called `new_card_reader.sh` script modified to have `?` as the delimiter.
- UIC ID Cards have ? as the last character.
- The output of the EVDEV Driver is piped to this shell script.
- This script contains a list of UINs allowed to open the door.
- If the card is of an allowed UIN, the python scirpt to open the door is called.

## Deployment Information

- Systemd user service called `doorkeeper.service`
- This service calls a shell script called `fuck-systemd.sh`
- This shell script calls the EVDEV Driver with its output piped to `the shell script in the submodule.
- Systemd messes up I/O redirection so the intermediary script is required.
