import digitalio
import board
import touchio
import usb_midi
import adafruit_midi
import board
from time import sleep
from adafruit_midi.note_on  import NoteOn
from adafruit_midi.note_off  import NoteOff
from adafruit_debouncer import Debouncer, Button

midi_velocity = 64  # midpoint
midi_channel = 0  # 0-15
midi = adafruit_midi.MIDI(midi_out=usb_midi.ports[1])

rows = [
    digitalio.DigitalInOut(board.GP18),
    digitalio.DigitalInOut(board.GP19),
    digitalio.DigitalInOut(board.GP20),
    digitalio.DigitalInOut(board.GP21),
    digitalio.DigitalInOut(board.GP22),
    digitalio.DigitalInOut(board.GP23),
    digitalio.DigitalInOut(board.GP24),
    digitalio.DigitalInOut(board.GP25),
]

columns = [
    digitalio.DigitalInOut(board.GP17),
    digitalio.DigitalInOut(board.GP16),
    digitalio.DigitalInOut(board.GP15),
    digitalio.DigitalInOut(board.GP14),
]


note_offset = 60
# playing = [0] * 12
playing = 0

for pin in columns:
    pin.direction = digitalio.Direction.INPUT
    pin.pull = digitalio.Pull.DOWN
    # pin.drive_mode = digitalio.DriveMode.OPEN_DRAIN

for pin in rows:
    pin.direction = digitalio.Direction.OUTPUT
    # pin.drive_mode = digitalio.DriveMode.PUSH_PULL


while True:
    for i, row in enumerate(rows):
        row.value = True 
        # sleep(2)
        
        for j, col in enumerate(columns):
            note = note_offset + i + (4 * j)
            # print(note)
            # print("row.value : ", col.value)

            if col.value and not playing:
                playing = note    
                # playing[i] = note
                midi.send( NoteOn(note, midi_velocity), channel=midi_channel )
                print(f"playing note: {note}")
                print(f"{i} + (4 * {j})")
            elif not col.value and note == playing:
                midi.send( NoteOff(playing, midi_velocity), channel=midi_channel )
                print(f"stopping note: {playing}")
                playing = 0

        row.value = False

        # if key_switch.value and not playing[i]:
        #     note = note_offset + i
        #     playing[i] = note
        #     midi.send( NoteOn(note, midi_velocity), channel=midi_channel )
        #     print(f"playing note: {note}")
        # elif key_switch.value and playing[i]:
        #     note = playing[i]
        #     midi.send( NoteOff(note, midi_velocity), channel=midi_channel )
        #     playing[i] = 0
        #     print(f"stopping note: {note}")

        # touch_vals[i] = value







