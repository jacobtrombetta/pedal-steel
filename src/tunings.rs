//! Tuning helpers for pedal-steel CLI.
//!
//! Contains functions to parse tuning strings into vectors of Pitch objects.

use rust_music_theory::note::{NoteLetter, Pitch};

/// Parse a comma-separated string of note names into a vector of Pitch objects
pub fn tuning(notes: &str) -> Vec<Pitch> {
    let mut pitches = Vec::new();

    for raw_note in notes.split(',') {
        let note = raw_note.trim().to_ascii_uppercase();

        match note.as_str() {
            "AB" => pitches.push(Pitch::new(NoteLetter::A, -1)),
            "A" => pitches.push(Pitch::new(NoteLetter::A, 0)),
            "A#" => pitches.push(Pitch::new(NoteLetter::A, 1)),
            "BB" => pitches.push(Pitch::new(NoteLetter::B, -1)),
            "B" => pitches.push(Pitch::new(NoteLetter::B, 0)),
            "C" => pitches.push(Pitch::new(NoteLetter::C, 0)),
            "C#" => pitches.push(Pitch::new(NoteLetter::C, 1)),
            "DB" => pitches.push(Pitch::new(NoteLetter::D, -1)),
            "D" => pitches.push(Pitch::new(NoteLetter::D, 0)),
            "D#" => pitches.push(Pitch::new(NoteLetter::D, 1)),
            "EB" => pitches.push(Pitch::new(NoteLetter::E, -1)),
            "E" => pitches.push(Pitch::new(NoteLetter::E, 0)),
            "F" => pitches.push(Pitch::new(NoteLetter::F, 0)),
            "F#" => pitches.push(Pitch::new(NoteLetter::F, 1)),
            "GB" => pitches.push(Pitch::new(NoteLetter::G, -1)),
            "G" => pitches.push(Pitch::new(NoteLetter::G, 0)),
            "G#" => pitches.push(Pitch::new(NoteLetter::G, 1)),
            _ => (),
        }
    }

    pitches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn we_can_get_correct_tuning_from_different_input_formats() {
        let tuning = tuning("A, a, A#, a#, Ab, AB, ab, aB");

        assert_eq!(tuning.len(), 8);
        assert_eq!(format!("{}", tuning[0]), "A");
        assert_eq!(format!("{}", tuning[1]), "A");
        assert_eq!(format!("{}", tuning[2]), "A#");
        assert_eq!(format!("{}", tuning[3]), "A#");
        assert_eq!(format!("{}", tuning[4]), "Ab");
        assert_eq!(format!("{}", tuning[5]), "Ab");
        assert_eq!(format!("{}", tuning[6]), "Ab");
        assert_eq!(format!("{}", tuning[7]), "Ab");
    }

    #[test]
    fn we_can_get_correct_tuning() {
        let tuning = tuning("F#, B, G, E, Db");

        assert_eq!(tuning.len(), 5);
        assert_eq!(format!("{}", tuning[0]), "F#");
        assert_eq!(format!("{}", tuning[1]), "B");
        assert_eq!(format!("{}", tuning[2]), "G");
        assert_eq!(format!("{}", tuning[3]), "E");
        assert_eq!(format!("{}", tuning[4]), "Db");
    }

    #[test]
    fn we_can_avoid_incorrect_turning_input() {
        let tuning = tuning("Xb, BD, P Don Helms,");
        assert_eq!(tuning.len(), 0);
    }
}
