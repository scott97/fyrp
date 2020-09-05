#!/bin/bash
cd rust/microcontroller
cargo objcopy --release -- -O ihex teensy.hex