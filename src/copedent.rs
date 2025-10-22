//! Copedent — pedal/lever definitions and helpers for a 10‑string pedal steel.
//!
//! This module models the copedent (pedal and lever changes) for a pedal-steel
//! neck: which strings are raised or lowered and by how many semitones.

use strum_macros::EnumIter;

const NUMBER_OF_STRINGS: usize = 10;

/// Represents a change in the copedent for a specific string
#[derive(Debug)]
pub struct CopedentChange {
    /// The string number
    pub string: u8,
    /// The semitone change (+/-) as an integer
    pub semitone_change: i8,
}

/// Represents a change in the copedent for a specific position
#[derive(Debug)]
pub struct Copedent {
    /// The list of copedent changes for the position
    pub copedent_change: Vec<CopedentChange>,
}

/// Represents the overall pedal and lever changes for a set of positions
pub struct PedalAndLevers {
    /// The list of copedent changes for the set of positions
    pub copedent_change: Vec<u8>,
}

/// Represents a pedal or lever position on a pedal steel guitar
#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Open,
    A,
    B,
    C,
    D,
    Lkl,
    Lkv,
    Lkr,
    Rkl,
    Rkr,
}

/// Get a list of possible pedal and lever combos for a pedal steel guitar
pub fn possible_positions() -> Vec<Vec<Position>> {
    vec![
        vec![Position::Open],
        vec![Position::A],
        vec![Position::B],
        vec![Position::C],
        vec![Position::Lkl],
        vec![Position::Lkv],
        vec![Position::Lkr],
        vec![Position::Rkl],
        vec![Position::Rkr],
        vec![Position::A, Position::B],
        vec![Position::B, Position::C],
        vec![Position::A, Position::Lkl],
        vec![Position::B, Position::Lkr],
        vec![Position::Lkv],
        vec![Position::Rkl],
        vec![Position::Rkr],
    ]
}

/// Get the string representation of a position
pub const fn position_string(position: &Position) -> &str {
    match position {
        Position::Open => "Open",
        Position::A => "A",
        Position::B => "B",
        Position::C => "C",
        Position::D => "D",
        Position::Lkl => "LKL",
        Position::Lkv => "LKV",
        Position::Lkr => "LKR",
        Position::Rkl => "RKL",
        Position::Rkr => "RKR",
    }
}

/// Get the copedent changes for a specific position
#[allow(clippy::too_many_lines)]
pub fn copedent_change(position: Position) -> Copedent {
    match position {
        Position::Open => Copedent {
            copedent_change: vec![],
        },
        Position::A => Copedent {
            copedent_change: vec![
                CopedentChange {
                    string: 10,
                    semitone_change: 2,
                },
                CopedentChange {
                    string: 5,
                    semitone_change: 2,
                },
            ],
        },
        Position::B => Copedent {
            copedent_change: vec![
                CopedentChange {
                    string: 6,
                    semitone_change: 1,
                },
                CopedentChange {
                    string: 3,
                    semitone_change: 1,
                },
            ],
        },
        Position::C => Copedent {
            copedent_change: vec![
                CopedentChange {
                    string: 5,
                    semitone_change: 2,
                },
                CopedentChange {
                    string: 4,
                    semitone_change: 2,
                },
            ],
        },
        Position::D => Copedent {
            copedent_change: vec![CopedentChange {
                string: 1,
                semitone_change: 2,
            }],
        },
        Position::Lkl => Copedent {
            copedent_change: vec![
                CopedentChange {
                    string: 8,
                    semitone_change: 1,
                },
                CopedentChange {
                    string: 4,
                    semitone_change: 1,
                },
            ],
        },
        Position::Lkv => Copedent {
            copedent_change: vec![CopedentChange {
                string: 5,
                semitone_change: -1,
            }],
        },
        Position::Lkr => Copedent {
            copedent_change: vec![
                CopedentChange {
                    string: 8,
                    semitone_change: -1,
                },
                CopedentChange {
                    string: 4,
                    semitone_change: -1,
                },
            ],
        },
        Position::Rkl => Copedent {
            copedent_change: vec![
                CopedentChange {
                    string: 1,
                    semitone_change: -1,
                },
                CopedentChange {
                    string: 6,
                    semitone_change: -2,
                },
            ],
        },
        Position::Rkr => Copedent {
            copedent_change: vec![
                CopedentChange {
                    string: 9,
                    semitone_change: -1,
                },
                CopedentChange {
                    string: 2,
                    semitone_change: -1,
                },
            ],
        },
    }
}

/// Calculate the overall pedal and lever changes for a set of positions
pub fn pedal_and_levers(positions: &[Position]) -> PedalAndLevers {
    let mut copedent_offset = [0_u8; NUMBER_OF_STRINGS];

    // For each position, get the copedent changes and add them to the offset
    for position in positions {
        let copedent = copedent_change(*position);

        // Define the copedent offset
        for change in &copedent.copedent_change {
            copedent_offset[(change.string - 1) as usize] +=
                change.semitone_change.rem_euclid(12) as u8;
        }
    }

    PedalAndLevers {
        copedent_change: copedent_offset.to_vec(),
    }
}

/// Generate a name for a set of positions
pub fn position_name(positions: &[Position]) -> String {
    let mut name = String::new();
    for position in positions {
        if !name.is_empty() {
            name.push_str(" & ");
        }
        name.push_str(position_string(position));
    }
    name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copedent_change() {
        let result = copedent_change(Position::A);
        assert_eq!(result.copedent_change.len(), 2);
        assert_eq!(result.copedent_change[0].string, 10);
        assert_eq!(result.copedent_change[0].semitone_change, 2);
        assert_eq!(result.copedent_change[1].string, 5);
        assert_eq!(result.copedent_change[1].semitone_change, 2);
    }

    #[test]
    fn test_pedal_and_levers_open() {
        let result = pedal_and_levers(&[Position::Open]);
        assert_eq!(result.copedent_change, vec![0; NUMBER_OF_STRINGS]);
    }

    #[test]
    fn test_pedal_and_levers_a() {
        let result = pedal_and_levers(&[Position::A]);
        assert_eq!(result.copedent_change, vec![0, 0, 0, 0, 2, 0, 0, 0, 0, 2]);
    }

    #[test]
    fn test_pedal_and_levers_a_b() {
        let result = pedal_and_levers(&[Position::A, Position::B]);
        assert_eq!(result.copedent_change, vec![0, 0, 1, 0, 2, 1, 0, 0, 0, 2]);
    }

    #[test]
    fn test_position_name() {
        let name = position_name(&[Position::A, Position::B, Position::Lkr]);
        assert_eq!(name, "A & B & LKR");
    }
}
