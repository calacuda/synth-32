#include <Arduino.h>
#include <math.h>
#include <tinycbor.h>
#include <float.h>
#include <Adafruit_TinyUSB.h>
#include <MIDI.h>

/*
cbor data should be in the form of this json example:
{
  entity: <Play/Stop>Note,
  set: <true/false>
  args: <MIDI-NOTE>
}
*/

// Error check and exiting.
#define CHECK_ERROR(proc) {\
    if( (err = proc) != 0) {\
        err_line = __LINE__;\
        goto on_error;\
    }\
}
#define CFG_TUD_MIDI 1

//MIDI CC (change control) assignments
const int CHANNEL         = 0; // MIDI channel 
const int LEFT_BUTTON_CC  = 4; // Play/Pause Deck A
const int PLAY_NOTE_CC    = 1; // play note
const int STOP_NOTE_CC    = 0; // stop note

Adafruit_USBD_MIDI usb_midi;
MIDI_CREATE_INSTANCE(Adafruit_USBD_MIDI, usb_midi, MIDI);

int midi_note_offset = 0;
int second_half_offset = 32;
const int max_polyphony = 64;
bool playing[max_polyphony]; 
bool last_playing[max_polyphony]; 

int max_row_i = 16;
int max_col_i = 8;
int row_pins_1[8] = {7, 8, 9, 10, 11, 12, 13, 14}; // the pins used to control the row mux switch.
// int row_pins[4] = {7, 8, 9, 10}; // the pins used to control the row mux switch.
int col_pins_1[4] = {3, 4, 5, 6}; // the pins used to control the col mix switch.
// int col_pins[3] = {4, 5, 6}; // the pins used to control the col mix switch.
int row_pins_2[8] = {19, 20, 21, 22, 23, 24, 25, 2}; // the pins used to control the row mux switch.
int col_pins_2[4] = {15, 16, 17, 18}; // the pins used to control the col mix switch.

// void enable_row(int row);

void setup() {
    // put your setup code here, to run once:
    // Serial.begin(115200);
    // Serial1.begin(115200);
    TinyUSB_Device_Init(0);
    MIDI.begin(MIDI_CHANNEL_OMNI);

    for (int i; i < 8; i++) {
        pinMode(row_pins_1[i], OUTPUT);
        // digitalWrite(row_pins_1[i], LOW);
        pinMode(row_pins_2[i], OUTPUT);
        // digitalWrite(row_pins_2[i], LOW);
    } 
    
    for (int i; i < 4; i++) {
        pinMode(col_pins_1[i], INPUT_PULLDOWN);
        pinMode(col_pins_1[i], INPUT_PULLDOWN);
    }

    for (int i = 0; i < max_polyphony; i ++) {
        playing[i] = false;
        last_playing[i] = false;
    }

    // delay(2500);
    pinMode(25, OUTPUT);
    digitalWrite(25, HIGH);    
    Serial.println("setup complete");
}

void loop() {
    // put your main code here, to run repeatedly:
    // digitalWrite(row_pin, HIGH);

    // while (true) {
    //     Serial.println("test...");
    //     enable_row(0);
    //     enable_col(1);
    //     Serial.println(digitalRead(col_pin));
    //     Serial.flush();
    //     delay(1000);
    // }    

    // TODO: scan keyboard
    for (int row_i = 0; row_i < 8; row_i++) {
        // scan first half
        digitalWrite(row_pins_1[row_i], HIGH);
        digitalWrite(row_pins_2[row_i], HIGH);

        for (int col_i = 0; col_i < 4; col_i ++) {
            int playing_i_1 = (col_i * 8) + row_i;
            int playing_i_2 = (col_i * 8) + row_i + second_half_offset;
            playing[playing_i_1] = digitalRead(col_pins_1[col_i]);
            playing[playing_i_2] = digitalRead(col_pins_2[col_i]);
            // Serial.println(digitalRead(col_pin));
            // if (digitalRead(col_pins_1[col_i])) {
            //     Serial.printf("row = %d, col = %d, playing = %d\n", row_i, col_i, playing_i_1);
            // }
            // if (digitalRead(col_pins_2[col_i])) {
            //     Serial.printf("row = %d, col = %d, playing = %d\n", row_i, col_i, playing_i_2);
            // } 
            // else {
            //     Serial.println("nothing");
            // }
        } 
    }
    
    // TODO: get knob data form knob micro

    // send playing data to synth over Serial1

    if (TinyUSBDevice.mounted()) {
        // TODO: send midi
        for (int i = 0; i < max_polyphony; i ++) {
            int note = midi_note_offset + i;

            if (playing[i] && !last_playing[i]) {
                // Serial.printf("playing note: %d\n", i);
                MIDI.sendNoteOn(note, 127, 1);
            } else if (!playing[i] && last_playing[i]) {
                // Serial.printf("stopping note: %d\n", i);
                MIDI.sendNoteOff(note, 0, 1);                
            }

            last_playing[i] = playing[i];
        }

        while (MIDI.read()) {}
    }
}