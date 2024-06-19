# Keyboard-Controller

This directory contains the [Circuit Python](https://circuitpython.org/) `code.py` file.

## Why Circuit Python? (and not, Arduino, Platformio, or Rust?)

I chose to write the keyboard controller in Circuit Python because it allows the [Raspberry pi Pico](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) to mount as a USB [MIDI](https://en.wikipedia.org/wiki/MIDI) device and still get access to a serial console for debugging, and it allowed me to reprogram the pico without needing to pressing the boot select button. If it were programmed using Platformio and Arduino in a way that supported USB MIDI, then I'd need to press the boot select button, unplug, and then replug the pico every time I wanted to flash new firemware because of how USB MIDI is handled in that environment. Circuit Python was just easier.
