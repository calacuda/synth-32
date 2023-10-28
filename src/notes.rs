use crate::Note;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref NOTES: HashMap<&'static str, Note> = {
        let mut notes = HashMap::new();
        notes.insert("C0", 16.35);
        notes.insert("C#0", 17.32);
        notes.insert("Db0", 17.32);
        notes.insert("D0", 18.35);
        notes.insert("D#0", 19.45);
        notes.insert("Eb0", 19.45);
        notes.insert("E0", 20.6);
        notes.insert("F0", 21.83);
        notes.insert("F#0", 23.12);
        notes.insert("Gb0", 23.12);
        notes.insert("G0", 24.5);
        notes.insert("G#0", 25.96);
        notes.insert("Ab0", 25.96);
        notes.insert("A0", 27.5);
        notes.insert("A#0", 29.14);
        notes.insert("Bb0", 29.14);
        notes.insert("B0", 30.87);
        notes.insert("C1", 32.7);
        notes.insert("C#1", 34.65);
        notes.insert("Db1", 34.65);
        notes.insert("D1", 36.71);
        notes.insert("D#1", 38.89);
        notes.insert("Eb1", 38.89);
        notes.insert("E1", 41.2);
        notes.insert("F1", 43.65);
        notes.insert("F#1", 46.25);
        notes.insert("Gb1", 46.25);
        notes.insert("G1", 49.0);
        notes.insert("G#1", 51.91);
        notes.insert("Ab1", 51.91);
        notes.insert("A1", 55.0);
        notes.insert("A#1", 58.27);
        notes.insert("Bb1", 58.27);
        notes.insert("B1", 61.74);
        notes.insert("C2", 65.41);
        notes.insert("C#2", 69.3);
        notes.insert("Db2", 69.3);
        notes.insert("D2", 73.42);
        notes.insert("D#2", 77.78);
        notes.insert("Eb2", 77.78);
        notes.insert("E2", 82.41);
        notes.insert("F2", 87.31);
        notes.insert("F#2", 92.5);
        notes.insert("Gb2", 92.5);
        notes.insert("G2", 98.0);
        notes.insert("G#2", 103.83);
        notes.insert("Ab2", 103.83);
        notes.insert("A2", 110.0);
        notes.insert("A#2", 116.54);
        notes.insert("Bb2", 116.54);
        notes.insert("B2", 123.47);
        notes.insert("C3", 130.81);
        notes.insert("C#3", 138.59);
        notes.insert("Db3", 138.59);
        notes.insert("D3", 146.83);
        notes.insert("D#3", 155.56);
        notes.insert("Eb3", 155.56);
        notes.insert("E3", 164.81);
        notes.insert("F3", 174.61);
        notes.insert("F#3", 185.0);
        notes.insert("Gb3", 185.0);
        notes.insert("G3", 196.0);
        notes.insert("G#3", 207.65);
        notes.insert("Ab3", 207.65);
        notes.insert("A3", 220.0);
        notes.insert("A#3", 233.08);
        notes.insert("Bb3", 233.08);
        notes.insert("B3", 246.94);
        notes.insert("C4", 261.63);
        notes.insert("C#4", 277.18);
        notes.insert("Db4", 277.18);
        notes.insert("D4", 293.66);
        notes.insert("D#4", 311.13);
        notes.insert("Eb4", 311.13);
        notes.insert("E4", 329.63);
        notes.insert("F4", 349.23);
        notes.insert("F#4", 369.99);
        notes.insert("Gb4", 369.99);
        notes.insert("G4", 392.0);
        notes.insert("G#4", 415.3);
        notes.insert("Ab4", 415.3);
        notes.insert("A4", 440.0);
        notes.insert("A#4", 466.16);
        notes.insert("Bb4", 466.16);
        notes.insert("B4", 493.88);
        notes.insert("C5", 523.25);
        notes.insert("C#5", 554.37);
        notes.insert("Db5", 554.37);
        notes.insert("D5", 587.33);
        notes.insert("D#5", 622.25);
        notes.insert("Eb5", 622.25);
        notes.insert("E5", 659.25);
        notes.insert("F5", 698.46);
        notes.insert("F#5", 739.99);
        notes.insert("Gb5", 739.99);
        notes.insert("G5", 783.99);
        notes.insert("G#5", 830.61);
        notes.insert("Ab5", 830.61);
        notes.insert("A5", 880.0);
        notes.insert("A#5", 932.33);
        notes.insert("Bb5", 932.33);
        notes.insert("B5", 987.77);
        notes.insert("C6", 1046.5);
        notes.insert("C#6", 1108.73);
        notes.insert("Db6", 1108.73);
        notes.insert("D6", 1174.66);
        notes.insert("D#6", 1244.51);
        notes.insert("Eb6", 1244.51);
        notes.insert("E6", 1318.51);
        notes.insert("F6", 1396.91);
        notes.insert("F#6", 1479.98);
        notes.insert("Gb6", 1479.98);
        notes.insert("G6", 1567.98);
        notes.insert("G#6", 1661.22);
        notes.insert("Ab6", 1661.22);
        notes.insert("A6", 1760.0);
        notes.insert("A#6", 1864.66);
        notes.insert("Bb6", 1864.66);
        notes.insert("B6", 1975.53);
        notes.insert("C7", 2093.0);
        notes.insert("C#7", 2217.46);
        notes.insert("Db7", 2217.46);
        notes.insert("D7", 2349.32);
        notes.insert("D#7", 2489.02);
        notes.insert("Eb7", 2489.02);
        notes.insert("E7", 2637.02);
        notes.insert("F7", 2793.83);
        notes.insert("F#7", 2959.96);
        notes.insert("Gb7", 2959.96);
        notes.insert("G7", 3135.96);
        notes.insert("G#7", 3322.44);
        notes.insert("Ab7", 3322.44);
        notes.insert("A7", 3520.0);
        notes.insert("A#7", 3729.31);
        notes.insert("Bb7", 3729.31);
        notes.insert("B7", 3951.07);
        notes.insert("C8", 4186.01);
        notes.insert("C#8", 4434.92);
        notes.insert("Db8", 4434.92);
        notes.insert("D8", 4698.63);
        notes.insert("D#8", 4978.03);
        notes.insert("Eb8", 4978.03);
        notes.insert("E8", 5274.04);
        notes.insert("F8", 5587.65);
        notes.insert("F#8", 5919.91);
        notes.insert("Gb8", 5919.91);
        notes.insert("G8", 6271.93);
        notes.insert("G#8", 6644.88);
        notes.insert("Ab8", 6644.88);
        notes.insert("A8", 7040.0);
        notes.insert("A#8", 7458.62);
        notes.insert("Bb8", 7458.62);
        notes.insert("B8", 7902.13);

        notes
    };
}
