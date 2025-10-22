//! Guitar and pedal-steel neck representation and note identification.
//!
//! This module defines a `Guitar` struct representing a guitar or pedal-steel
//! instrument, along with functions to identify notes on the neck based on a
//! given tuning and copedent (pedal and lever changes). It also includes
//! functionality to find frets that contain all chord tones for a specified chord.

use crate::{
    copedent::{Position, pedal_and_levers},
    tunings::tuning,
};
use rust_music_theory::{
    chord::Chord,
    note::{Note, Notes, Pitch},
    scale::Direction,
};
use std::collections::{HashMap, HashSet};

/// Representation of a guitar or pedal-steel instrument
pub struct Guitar {
    pub name: String,
    pub tuning: Vec<Pitch>,
}

impl Guitar {
    pub fn new(name: &str, notes: &str) -> Self {
        Self {
            name: name.to_string(),
            tuning: tuning(notes),
        }
    }
}

/// Identify notes on the guitar neck for a given position and return their positions
#[derive(Debug, Clone)]
pub struct NeckPositions {
    pub pitch: Pitch,
    pub note_name: String,
    pub string: usize,
    pub fret: usize,
}

fn display_as_flats_or_sharps(notes: &[Note]) -> Direction {
    if notes.iter().any(|n| n.pitch.accidental < 0) {
        Direction::Descending
    } else {
        Direction::Ascending
    }
}

fn populate_neck_pitches(
    guitar: &Guitar,
    position: &[Position],
    direction: Direction,
) -> Vec<Vec<Pitch>> {
    let pedal_and_levers = pedal_and_levers(position);

    let mut neck = Vec::new();
    for i in 0..guitar.tuning.len() {
        let row = (0..12)
            .map(|j| {
                Pitch::from_u8_with_direction(
                    guitar.tuning[i].into_u8()
                        + pedal_and_levers.copedent_change[i]
                        + u8::try_from(j).unwrap_or(0),
                    direction,
                )
            })
            .collect::<Vec<_>>();
        neck.push(row);
    }

    neck
}

/// Identify notes on the guitar neck for a given position and return their positions
pub fn identify_notes_on_neck(
    guitar: &Guitar,
    position: &[Position],
    notes: &[Note],
) -> Vec<NeckPositions> {
    let direction = display_as_flats_or_sharps(notes);
    let neck = populate_neck_pitches(guitar, position, direction);

    let mut neck_positions = Vec::new();
    for (i, row) in neck.iter().enumerate() {
        for (j, pitch) in row.iter().enumerate() {
            if notes.iter().any(|note| note.pitch == *pitch) {
                neck_positions.push(NeckPositions {
                    pitch: *pitch,
                    note_name: format!("{pitch}"),
                    string: i,
                    fret: j,
                });
            }
        }
    }
    neck_positions
}

/// Find frets that contain all chord tones and return their positions
pub fn frets_with_all_chord_tones(
    neck_positions: &[NeckPositions],
    chord: &Chord,
) -> Vec<NeckPositions> {
    let chord_pitches: HashSet<_> = chord
        .notes()
        .iter()
        .map(|n| format!("{}", n.pitch))
        .collect();
    let mut fret_map: HashMap<usize, Vec<&NeckPositions>> = HashMap::new();

    // Group NeckPositions by fret
    for pos in neck_positions {
        fret_map.entry(pos.fret).or_default().push(pos);
    }

    // Collect NeckPositions for frets that have all chord tones
    fret_map
        .into_iter()
        .filter_map(|(_fret, positions)| {
            let pitches_on_fret: HashSet<_> =
                positions.iter().map(|p| format!("{}", p.pitch)).collect();
            if chord_pitches.is_subset(&pitches_on_fret) {
                Some(
                    positions
                        .into_iter()
                        .cloned()
                        .collect::<Vec<NeckPositions>>(),
                )
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guitar_creation() {
        let notes = "E, C#, A, F#, E, C#, A, F#";
        let guitar = Guitar::new("A6th lap steel", notes);
        assert_eq!(guitar.name, "A6th lap steel");
        assert_eq!(guitar.tuning, tuning(notes));
        assert_eq!(guitar.tuning.len(), 8);
    }

    #[test]
    fn test_identify_notes_on_neck() {
        let guitar = Guitar::new("Test Guitar", "E");
        let position = vec![Position::Open];
        let chord = Chord::from_regex("E major").unwrap();

        let neck_positions = identify_notes_on_neck(&guitar, &position, &chord.notes());

        assert_eq!(neck_positions.len(), 3);
        assert_eq!(neck_positions[0].string, 0);
        assert_eq!(neck_positions[0].fret, 0);
        assert_eq!(neck_positions[0].note_name, "E");
        assert_eq!(neck_positions[1].string, 0);
        assert_eq!(neck_positions[1].fret, 4);
        assert_eq!(neck_positions[1].note_name, "G#");
        assert_eq!(neck_positions[2].string, 0);
        assert_eq!(neck_positions[2].fret, 7);
        assert_eq!(neck_positions[2].note_name, "B");
    }

    #[test]
    fn test_frets_with_all_chord_tones() {
        let guitar = Guitar::new("Test Guitar", "E, G#, B");
        let position = vec![Position::Open];
        let chord = Chord::from_regex("E major").unwrap();

        let neck_positions = identify_notes_on_neck(&guitar, &position, &chord.notes());
        let frets = frets_with_all_chord_tones(&neck_positions, &chord);

        assert_eq!(frets.len(), 3);
        assert_eq!(frets[0].string, 0);
        assert_eq!(frets[0].fret, 0);
        assert_eq!(frets[0].note_name, "E");
        assert_eq!(frets[1].string, 1);
        assert_eq!(frets[1].fret, 0);
        assert_eq!(frets[1].note_name, "G#");
        assert_eq!(frets[2].string, 2);
        assert_eq!(frets[2].fret, 0);
        assert_eq!(frets[2].note_name, "B");
    }
}
