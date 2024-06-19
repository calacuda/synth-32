#include <Arduino.h>
#include <math.h>
#include <float.h>
#include <ArduinoJson.h>

// Error check and exiting.
#define CHECK_ERROR(proc) {\
    if( (err = proc) != 0) {\
        err_line = __LINE__;\
        goto on_error;\
    }\
}
#define CFG_TUD_MIDI 1

int mux_pins[4] = {6, 7, 10, 8};
int mux_adc_pin = A2;
int pitch_wheel_pin = A0;
int mod_wheel_pin = A1;

double attack = 0.0;
double decay = 0.0;
double sustain = 0.0;
double cutoff = 0.0;
double resonance = 0.0;
double pitch_bend = 0.0;
double volume = 0.0;

void mux_activate(int num) {
    digitalWrite(mux_pins[0], bitRead(num, 0));
    digitalWrite(mux_pins[1], bitRead(num, 1));
    digitalWrite(mux_pins[2], bitRead(num, 2));
    digitalWrite(mux_pins[3], bitRead(num, 3));
}

void setup() {
    Serial1.begin(460800);
    Serial.begin(115200);

    pinMode(mux_adc_pin, INPUT);
    // pinMode(pitch_wheel_pin, INPUT_PULLDOWN);
    // pinMode(mod_wheel_pin, INPUT_PULLDOWN);

    pinMode(mux_pins[0], OUTPUT);
    digitalWrite(mux_pins[0], LOW);
    pinMode(mux_pins[1], OUTPUT);
    digitalWrite(mux_pins[1], LOW);
    pinMode(mux_pins[2], OUTPUT);
    digitalWrite(mux_pins[2], LOW);
    pinMode(mux_pins[3], OUTPUT);
    digitalWrite(mux_pins[3], LOW);

    // while (!Serial.available()) {}

    Serial.println("*** SETUP DONE ***");
}

void loop() {
    // Serial1.print("[");
    JsonDocument mega_doc;
    JsonArray array = mega_doc.to<JsonArray>();
    mux_activate(0);
    delay(3);
    double new_attack = (double) analogRead(mux_adc_pin) / 1024.0;
    // Serial.println(new_attack);

    if (fabs(new_attack - attack) > 0.01) {
        JsonDocument doc;
        
        doc["entity"] = "EnvAttack";
        doc["set"] = true;
        doc["args"] = new_attack;

        attack = new_attack;

        array.add(doc);
        // // Serial1.flush();
        // Serial1.print(",");
    }

    mux_activate(1);
    delay(3);
    double new_decay = (double) analogRead(mux_adc_pin) / 1024.0;
    // Serial.println(new_decay);

    if (fabs(new_decay - decay) > 0.01) {
        JsonDocument doc;
        
        doc["entity"] = "EnvDecay";
        doc["set"] = true;
        doc["args"] = new_decay;

        decay = new_decay;

        array.add(doc);
        // // Serial1.flush();
        // Serial1.print(",");
    }

    mux_activate(2);
    delay(3);
    double new_sus = (double) analogRead(mux_adc_pin) / 1024.0;
    // Serial.println(new_sus);

    if (fabs(new_sus - sustain) > 0.01) {
        JsonDocument doc;
        
        doc["entity"] = "EnvSustain";
        doc["set"] = true;
        doc["args"] = new_sus;

        sustain = new_sus;

        array.add(doc);
        // // Serial1.flush();
        // Serial1.print(",");
    }

    mux_activate(3);
    delay(3);
    double new_cutoff = (double) analogRead(mux_adc_pin) / 1024.0;
    // Serial.println(new_cutoff);

    if (fabs(new_cutoff - cutoff) > 0.01) {
        JsonDocument doc;
        
        doc["entity"] = "LowPassCutoff";
        doc["set"] = true;
        doc["args"] = new_cutoff;

        cutoff = new_cutoff;

        array.add(doc);
        // // Serial1.flush();
        // Serial1.print(",");
    }

    mux_activate(4);
    delay(3);
    double new_res = (double) analogRead(mux_adc_pin) / 1024.0;
    // Serial.println(new_res);

    if (fabs(new_res - resonance) > 0.01) {
        JsonDocument doc;
        
        doc["entity"] = "LowPassResonance";
        doc["set"] = true;
        doc["args"] = new_res;

        resonance = new_res;

        array.add(doc);
        // // Serial1.flush();
        // Serial1.print(",");
    }

    mux_activate(7);
    delay(3);
    double new_volume = (double) analogRead(mux_adc_pin) / 1024;
    // Serial.println(analogRead(mux_adc_pin));

    if (fabs(new_volume - volume) > 0.01) {
        JsonDocument doc;
        
        doc["entity"] = "Volume";
        doc["set"] = true;
        doc["args"] = new_volume;

        volume = new_volume;

        array.add(doc);
        // // Serial1.flush();
        // Serial1.print(",");
    }

    double new_pitch_bend = (double) analogRead(pitch_wheel_pin) / 1024;
    // Serial.println(new_pitch_bend);

    mux_activate(8);
    delay(3);
    if (fabs(new_pitch_bend - pitch_bend) > 0.01) {
        JsonDocument doc;
        
        doc["entity"] = "BendNote";
        doc["set"] = true;
        doc["args"] = new_pitch_bend;

        pitch_bend = new_pitch_bend;

        array.add(doc);
        // // Serial1.flush();
        // Serial1.print(",");
    }

    // double new_mod_wheel = (double) analogRead(mod_wheel_pin) / 1024;
    // // Serial.println(new_pitch_bend);

    // mux_activate(9);
    // if (fabs(new_mod_wheel - ) > 0.01) {
    //     JsonDocument doc;
        
    //     doc["entity"] = "BendNote";
    //     doc["set"] = true;
    //     doc["args"] = new_pitch_bend;

    //     pitch_bend = new_pitch_bend;

    //     serializeJson(doc, Serial1);
    //     // Serial1.flush();
    //     Serial1.print(",");
    // }

    serializeJson(mega_doc, Serial1);
    delay(4);
}