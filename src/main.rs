//! pedal-steel — CLI for inspecting pedal‑steel necks, tunings, and copedents.
//!
//! Small command line tool that:
//! - lists available scales and chords
//! - prints tunings (comma separated note list)
//! - prints copedent charts (pedal/lever semitone changes)
//! - shows scales or chord positions on a neck for a given tuning
//!
//! Usage examples:
//!   cargo run -- list scales
//!   cargo run -- tuning --notes "F#, D#, G#, E, B, G#, F#, E, D, B"
//!   cargo run -- scale --tuning-name "E9" --tuning "F#, D#, G#, E, B, G#, F#, E, D, B" --scale "E major"
//!   cargo run -- chord --tuning-name "E9" --tuning "F#, D#, G#, E, B, G#, F#, E, D, B" --chord "E major"
//!
//! The CLI (clap) is defined here; functionality is implemented in the
//! library modules: copedent, display, guitar, and tunings.

use clap::{Parser, Subcommand};
use pedal_steel::{
    copedent::{Position, possible_positions},
    display::{print_chord, print_chord_on_pedal_steel, print_copedent, print_scale, print_tuning},
    guitar::Guitar,
};
use rust_music_theory::{chord::Chord, scale::Scale};

// Constant is taken from rust-music-theory crate
// https://github.com/ozankasikci/rust-music-theory/blob/src/bin/rustmt.rs
const AVAILABLE_SCALES: [&str; 14] = [
    "Major|Ionian",
    "Minor|Aeolian",
    "Dorian",
    "Phrygian",
    "Lydian",
    "Mixolydian",
    "Locrian",
    "Harmonic Minor",
    "Melodic Minor",
    "Pentatonic Major",
    "Pentatonic Minor",
    "Blues",
    "Chromatic",
    "Whole Tone",
];

// Constant is taken from rust-music-theory crate
// https://github.com/ozankasikci/rust-music-theory/blob/src/bin/rustmt.rs
const AVAILABLE_CHORDS: [&str; 22] = [
    "Major Triad",
    "Minor Triad",
    "Suspended2 Triad",
    "Suspended4 Triad",
    "Augmented Triad",
    "Diminished Triad",
    "Major Seventh",
    "Minor Seventh",
    "Augmented Seventh",
    "Augmented Major Seventh",
    "Diminished Seventh",
    "Half Diminished Seventh",
    "Minor Major Seventh",
    "Dominant Seventh",
    "Dominant Ninth",
    "Major Ninth",
    "Dominant Eleventh",
    "Major Eleventh",
    "Minor Eleventh",
    "Dominant Thirteenth",
    "Major Thirteenth",
    "Minor Thirteenth",
];

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available items
    List {
        #[arg(value_enum)]
        what: ListWhat,
    },

    /// Print a tuning (comma separated notes, e.g. "F#, D#, G#, E, B, G#, F#, E, D, B")
    Tuning {
        #[arg(long)]
        notes: String,
    },

    /// Print copedent chart (uses internal copedent definitions)
    Copedent,

    /// Show a scale on neck for given tuning
    Scale {
        #[arg(long)]
        tuning_name: String,
        #[arg(long)]
        tuning: String,
        #[arg(long)]
        scale: String,
    },

    /// Show chord positions for given tuning
    Chord {
        #[arg(long)]
        tuning_name: String,
        #[arg(long)]
        tuning: String,
        #[arg(long)]
        chord: String,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum ListWhat {
    Scales,
    Chords,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { what } => match what {
            ListWhat::Scales => {
                for s in AVAILABLE_SCALES {
                    println!("{}", s);
                }
            }
            ListWhat::Chords => {
                for c in AVAILABLE_CHORDS {
                    println!("{}", c);
                }
            }
        },

        Commands::Tuning { notes } => {
            let guitar = Guitar::new("cli", &notes);
            print_tuning(&guitar.tuning);
        }

        Commands::Copedent => {
            print_copedent();
        }

        Commands::Scale {
            tuning_name,
            tuning,
            scale,
        } => {
            let guitar = Guitar::new(&tuning_name, &tuning);
            match Scale::from_regex(&scale) {
                Ok(scale_obj) => {
                    print_scale(&guitar, &[Position::Open], &scale_obj);
                }
                Err(_) => eprintln!("Invalid scale: {}", scale),
            }
        }

        Commands::Chord {
            tuning_name,
            tuning,
            chord,
        } => {
            let guitar = Guitar::new(&tuning_name, &tuning);
            match Chord::from_regex(&chord) {
                Ok(chord_obj) => {
                    // print chord positions (uses possible_positions)
                    let positions = possible_positions();
                    for position in positions {
                        print_chord(&guitar, &position, &chord_obj);
                        print_chord_on_pedal_steel(&guitar, &position, &chord_obj);
                    }
                }
                Err(_) => eprintln!("Invalid chord: {}", chord),
            }
        }
    }
}
