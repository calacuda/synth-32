# SYNTH-32

This is the code for the audio synthesis part of the project. It is written for the esp32-s3. The code found in this folder is responsible for reading controls (key, knobs, and buttons), synthesizing audio, and controlling peripherals.

## How

This code relies on the dual cores of the esp32-s3, one is used to generate samples and spit them out to the I2S DAC, and the other is used to read the controls and change the perameters of the audio synthisizer. Audio synthesis is handled by the "`Synth`" struct (source code can be found at: [../synth-lib/src/synth.rs](/synth-lib/src/synth.rs). this file path is relative to this README.) Reading the state of the controls is done with the "`Scanner`" struct (src code found here: [./src/controls/scanner.rs](./src/controls/scanner.rs).)

## TODOs

add UART command system
