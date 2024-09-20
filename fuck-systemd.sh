#!/bin/sh
./target/debug/door-driver /dev/input/event0 | ./new_card_reader.sh
