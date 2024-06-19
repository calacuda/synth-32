import busio
import digitalio
import board
import usb_midi
import adafruit_midi
import board
import json
from adafruit_midi.note_on  import NoteOn
from adafruit_midi.note_off  import NoteOff
from adafruit_debouncer import Debouncer, Button

midi_velocity = 64  # midpoint
midi_channel = 0  # 1-15
midi = adafruit_midi.MIDI(midi_out=usb_midi.ports[1])
uart = busio.UART(tx=board.GP0, rx=board.GP1, baudrate=460800, timeout=0.001)

rows_1 = [
    digitalio.DigitalInOut(board.GP7),
    digitalio.DigitalInOut(board.GP8),
    digitalio.DigitalInOut(board.GP9),
    digitalio.DigitalInOut(board.GP10),
    digitalio.DigitalInOut(board.GP11),
    digitalio.DigitalInOut(board.GP12),
    digitalio.DigitalInOut(board.GP13),
    digitalio.DigitalInOut(board.GP14),
]

cols_1 = [
    digitalio.DigitalInOut(board.GP3),
    digitalio.DigitalInOut(board.GP4),
    digitalio.DigitalInOut(board.GP5),
    digitalio.DigitalInOut(board.GP6),
]

rows_2 = [
    digitalio.DigitalInOut(board.GP19),
    digitalio.DigitalInOut(board.GP20),
    digitalio.DigitalInOut(board.GP21),
    digitalio.DigitalInOut(board.GP22),
    digitalio.DigitalInOut(board.GP23),
    digitalio.DigitalInOut(board.GP24),
    digitalio.DigitalInOut(board.GP25),
    digitalio.DigitalInOut(board.GP2),
]

cols_2 = [
    digitalio.DigitalInOut(board.GP15),
    digitalio.DigitalInOut(board.GP16),
    digitalio.DigitalInOut(board.GP17),
    digitalio.DigitalInOut(board.GP18),
]

note_offset = 24
playing = [None] * 64

for pin in cols_1:
    pin.direction = digitalio.Direction.INPUT
    pin.pull = digitalio.Pull.DOWN

for pin in cols_2:
    pin.direction = digitalio.Direction.INPUT
    pin.pull = digitalio.Pull.DOWN

for pin in rows_1:
    pin.direction = digitalio.Direction.OUTPUT

for pin in rows_2:
    pin.direction = digitalio.Direction.OUTPUT

while True:
    for i, (row_1, row_2) in enumerate(zip(rows_1, rows_2)):
        row_1.value = True 
        row_2.value = True 

        for j, (col_1, col_2) in enumerate(zip(cols_1, cols_2)):
            playing_1_i = i + (8 * j)
            playing_2_i = i + (8 * j) + 32
            note_1 = playing_1_i + note_offset
            note_2 = playing_2_i + note_offset

            if col_1.value and not playing[playing_1_i]:
                playing[playing_1_i] = True    
                # playing[i] = note
                cmd = { "entity": "PlayNote", "set": True, "args": note_1 }
                json.dump(cmd, uart)
                midi.send( NoteOn(note_1, midi_velocity), channel=midi_channel )
                # print(json.dumps(cmd))
                print(f"playing note: {note_1}")
                # print(f"{i} + (4 * {j})")
            elif not col_1.value and playing[playing_1_i]:
                midi.send( NoteOff(note_1, midi_velocity), channel=midi_channel )
                print(f"stopping note: {note_1}")
                cmd = { "entity": "StopNote", "set": True, "args": note_1 }
                json.dump(cmd, uart)
                playing[playing_1_i] = False

            # print(f"len playing {len(playing)}, playing_2_i {playing_2_i}")
            if col_2.value and not playing[playing_2_i]:
                playing[playing_2_i] = True    
                # playing[i] = note
                cmd = { "entity": "PlayNote", "set": True, "args": note_2 }
                json.dump(cmd, uart)
                midi.send( NoteOn(note_2, midi_velocity), channel=midi_channel )
                print(f"playing note: {note_2}")
                # print(f"{i} + (4 * {j})")
            elif not col_2.value and playing[playing_2_i]:
                midi.send( NoteOff(note_2, midi_velocity), channel=midi_channel )
                print(f"stopping note: {note_2}")
                cmd = { "entity": "StopNote", "set": True, "args": note_2 }
                json.dump(cmd, uart)
                playing[playing_2_i] = False

        row_1.value = False
        row_2.value = False
