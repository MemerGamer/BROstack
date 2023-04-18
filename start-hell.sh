#!/bin/bash
cd luigi

cargo run --release "$(../bf-compiler/brainfuck.py ../mario.bf)" "$(octave ../giuseppe.m 2>/dev/null)"
