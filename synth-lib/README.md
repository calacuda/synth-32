# synth-32-lib

A rust library that defines some common code for audio synthesis, useful for both the main audio synth and plugin modules. I broke this out to its own sub-project so peripherals can use it as well as the main audio synthiziser.

## Features

- ADBDR envelope
- ADSR envelope
- sin wave synthesis
- tremolo effect
  - speed control
  - depth control
- echo effect
  - speed control
  - volume control

## TODOs

**NOTE**: These are not nessesarily listed in priority order.

- [ ] add a third envelope
- [ ] add square wave oscilator
- [ ] add triangle wave oscilator
- [ ] add sawtooth wave oscilator
