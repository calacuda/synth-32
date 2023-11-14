# Synth-32.rs

An ESP32-s3 powered musical synthisiser with a plugin/module/add-on system and the ability to extend the keyboard while increacing polyphony.

## Key Features

- Add-On Modules to add effects, oscilators, controls, or even more keys/extra hardware, by plugging addon cards into the I2S in/out and UART. more [here](#add-on-system)
- built in tremolo effect
- built in echo effect

## Add-on System

This synth supports plugging in add-ons (also called "peripheral" or "plugins" else where in the docs). These add-ons plug into each other then back into the synth in an chain. This chain receives state change updates via UART, and audio samples via I2S in. -- This information get propegated down the chain. Meaing each link gets this information from the previous link. -- The link will then output the sample it generated or modified, via I2S. (If the sample was unmodified, then the link will simple echo the sample transparently to the next link in the add-on chain.) The links can also send commands to to the synth to change the state, play/stop notes, etc.

Bassically, each link in the add-on chain gets updated about the synth's state, can modify the samples it generates, generate its own samples, and/or send controls back to the synth. The consequence of this however, is that the synth will no longer output its samples directly because they're redirected to the add-on chain. Because of this, it is _crutial_ that each add-on in the chain either echos its I2S input to its I2S output, or, that it take its generated sample and merges it with the input sample.

## Directory Overview

| **Directory** | **Description**                                                                                                                            |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `/synth-32/`  | the main src code for the controls. prodused the flashable firmware.                                                                       |
| `/synth-lib/` | Code responsible for audio synthesis and built in effects. It's in a separate folder so other add-on modules can use it more easily.       |
| `/modules/`   | stores the code for modules. it is in a separate directory so the code can be written in other languages and/or for other microcontrollers |

## Planned Add-Ons

- [ ] extra oscilators
- [ ] extra keys (and oscilators and envelope switching and wave form switching)
- [ ] plotter w/ led matrix or small screen. (will plot the wave form sent to its I2S in).
- [ ] drum synth and click-track generator (click track should be able to output to a dedicated I2S DAC so only the player can here it.)
- [ ] drone note/chord synth (will auto repeat when note dies. will have a knob to control how long it takes to repeat the drone after it ends)
- [ ] looper (when set to record, it will record all UART state updates send to it, then echo them back as commands via UART)
- [ ] recorder (with line-out)
- [ ] arpeggiator

## TODOs

- add envelope filter switching
- add square waves
- add triangle waves
- add sawtooth waves
- add rotary encoder to switch between sine, square, triangle, and sawtooth waves.
