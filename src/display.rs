//! Display helpers for pedal-steel CLI.
//!
//! Contains functions to pretty-print tunings, copedent charts, scales and
//! chord positions for a pedal-steel neck.

use crate::{
    copedent::{Position, copedent_change, position_name, position_string},
    guitar::{Guitar, NeckPositions, frets_with_all_chord_tones, identify_notes_on_neck},
    tunings::tuning,
};
use rust_music_theory::{
    chord::Chord,
    note::{Note, Notes, Pitch},
    scale::Scale,
};
use std::fmt::Write;
use strum::IntoEnumIterator;

/// Print the tuning of the guitar to the console
pub fn print_tuning(tuning: &[Pitch]) {
    tuning
        .iter()
        .enumerate()
        .for_each(|(i, p)| println!("{:2} {}", i + 1, format_args!("{}", p)));
}

/// Print the copedent table to the console
pub fn print_copedent() {
    let positions: Vec<Position> = Position::iter().collect();
    let string_count = 10;

    // Print header
    print!("{:>4}", "");
    for pos in &positions {
        if pos != &Position::Open {
            print!("{:>4}", position_string(pos));
        }
    }
    println!();

    // For each string (1 to 10)
    for string in 1..=string_count {
        print!("{string:>4}");
        for pos in &positions {
            if pos != &Position::Open {
                let copedent = copedent_change(*pos);
                let mut symbol = String::from("   ");
                for change in &copedent.copedent_change {
                    if change.string == string {
                        symbol = match change.semitone_change {
                            2 => " ++",
                            1 => "  +",
                            -1 => "  -",
                            -2 => " --",
                            _ => "   ",
                        }
                        .to_string();
                    }
                }
                print!("{symbol:>4}");
            }
        }
        println!();
    }
}

fn print_neck_positions(
    guitar: &Guitar,
    positions: &[NeckPositions],
    position_name: Option<&str>,
) -> Result<(), std::fmt::Error> {
    println!("{}", guitar.name);
    if let Some(name) = position_name {
        println!(" {name}");
    }
    for i in 0..12 {
        print!("{i:>2} ");
    }
    println!();

    for i in 0..guitar.tuning.len() {
        let mut row = String::new();
        for j in 0..12 {
            if let Some(pos) = positions.iter().find(|p| p.string == i && p.fret == j) {
                write!(row, "{:>3}", pos.note_name)?;
            } else {
                write!(row, "{:>3}", " --")?;
            }
        }
        println!("{row}");
    }

    Ok(())
}

/// Print the chord positions to the console
pub fn print_chord(guitar: &Guitar, position: &[Position], chord: &Chord) {
    let neck_positions = identify_notes_on_neck(guitar, position, &chord.notes());

    if let Err(e) = print_neck_positions(guitar, &neck_positions, Some(&position_name(position))) {
        eprintln!("Error printing neck positions: {e}");
    }
}

/// Print the chord positions on a pedal steel guitar to the console
pub fn print_chord_on_pedal_steel(guitar: &Guitar, position: &[Position], chord: &Chord) {
    let neck_positions = identify_notes_on_neck(guitar, position, &chord.notes());
    let frets = frets_with_all_chord_tones(&neck_positions, chord);

    if let Err(e) = print_neck_positions(guitar, &frets, Some(&position_name(position))) {
        eprintln!("Error printing neck positions: {e}");
    }
}

/// Print the scale positions to the console
pub fn print_scale(guitar: &Guitar, position: &[Position], scale: &Scale) {
    let neck_positions = identify_notes_on_neck(guitar, position, &scale.notes());

    if let Err(e) = print_neck_positions(guitar, &neck_positions, Some(&position_name(position))) {
        eprintln!("Error printing neck positions: {e}");
    }
}

/// Print the notes on the guitar neck for a given position and notes
pub fn print_notes_on_neck(guitar: &Guitar, position: &[Position], notes: &str) {
    let pitch_list = tuning(notes);
    let note_list: Vec<Note> = pitch_list.iter().map(|p| Note::new(*p, 0)).collect();
    let neck_positions = identify_notes_on_neck(guitar, position, &note_list);

    if let Err(e) = print_neck_positions(guitar, &neck_positions, Some(&position_name(position))) {
        eprintln!("Error printing neck positions: {e}");
    }
}
