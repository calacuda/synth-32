// use crate::notes::NOTES;
// use crate::Note;

const BEAT: f64 = 1_000_000.0;

pub const SONG: [(&str, u32, u32); 20] = [
    // Phrase 1 section 1
    ("F3", (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
    ("F3", (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
    ("Ab3", (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
    ("Ab3", (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
    ("F3", (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
    ("C3", (BEAT / 8.0) as u32, (BEAT / 16.0) as u32),
    ("Eb3", (BEAT / 2.0) as u32, (BEAT / 16.0) as u32),
    ("F3", (BEAT / 4.0) as u32, (BEAT / 4.0) as u32),
    // Chords
    ("D5", ((BEAT / 4.0) * 3.0) as u32, (BEAT / 32.0) as u32),
    ("C5", (BEAT / 2.0) as u32, (BEAT / 4.0) as u32),
    // Phrase 1 section 2
    ("F3", (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
    ("F3", (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
    ("Ab3", (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
    ("C4", (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
    ("C4", (BEAT / 16.0) as u32, (BEAT / 32.0) as u32),
    ("C#4", (BEAT / 16.0) as u32, (BEAT / 32.0) as u32),
    ("C4", (BEAT / 16.0) as u32, (BEAT / 32.0) as u32),
    ("A#3", (BEAT / 4.0) as u32, (BEAT / 4.0) as u32),
    // Chords
    ("D5", ((BEAT / 4.0) * 3.0) as u32, (BEAT / 32.0) as u32),
    ("C5", (BEAT / 2.0) as u32, (BEAT / 4.0) as u32),
];
