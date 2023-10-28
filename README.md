# Synth-32

Synth-32 is an esp-32 based, polyphonic, wave table synth, written in rust.

## Features

- [x] sine wave
- [] square wave
- [] triangle wave
- [] sawtooth wave
- [] attack setting
- [] decay setting
- [x] tremolo

## Software Notes

When compiled in debug mode the synth supports a max of 4 oscilators. But, when compiled in release mode, it supports a max of 5 oscilators. by default there is one oscilator per note.

## Harware Notes

TODO: design hardware.

## TODOs

implement the remaining features.
