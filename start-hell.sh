#!/bin/bash
cd luigi
cargo run --release "$(../bf ../mario.bf)" "$(octave ../giuseppe.m 2>/dev/null)"
