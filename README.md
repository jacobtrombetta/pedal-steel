# Pedal Steel

A command line tool for inspecting pedal steel guitar necks, tunings, and copedents. Visualize scales and chords across different pedal and lever positions on a 10-string pedal steel guitar.

## Features

- **List available scales and chords** - View all supported scale and chord types
- **Parse and print tunings** - Display tunings from comma-separated note lists
- **Print copedent charts** - Visualize pedal/lever semitone changes in a table format
- **Show scale positions** - See where scale notes fall across the neck for any tuning
- **Show chord positions** - Find chord voicings and complete chord tones at specific frets
- **Flexible position support** - Analyze Open position plus pedals (A, B, C, D) and levers (LKL, LKV, LKR, RKL, RKR)

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed (1.70.0 or later recommended).

Clone this repository and build:

```bash
git clone https://github.com/jacobtrombetta/pedal-steel.git
cd pedal-steel
cargo build --release
```

The compiled binary will be in `target/release/pedal-steel`.

## Quick Start

### List Available Scales and Chords

```bash
cargo run -- list scales
cargo run -- list chords
```

### Print a Tuning

Display the notes of a tuning:

```bash
cargo run -- tuning --notes "F#, D#, G#, E, B, G#, F#, E, D, B"
```

### View Copedent Chart

Print the copedent table showing which strings are affected by each pedal and lever:

```bash
cargo run -- copedent
```

### Visualize a Scale

Show where scale notes appear on the neck:

```bash
cargo run -- scale \
  --tuning-name "E9" \
  --tuning "F#, D#, G#, E, B, G#, F#, E, D, B" \
  --scale "E major"
```

**Output:**
```
E9
 Open
 0  1  2  3  4  5  6  7  8  9 10 11 
 F# -- G#  A --  B -- C# -- D#  E --
 D#  E -- F# -- G#  A --  B -- C# --
 G#  A --  B -- C# -- D#  E -- F# --
  E -- F# -- G#  A --  B -- C# -- D#
  B -- C# -- D#  E -- F# -- G#  A --
 G#  A --  B -- C# -- D#  E -- F# --
 F# -- G#  A --  B -- C# -- D#  E --
  E -- F# -- G#  A --  B -- C# -- D#
 -- D#  E -- F# -- G#  A --  B -- C#
  B -- C# -- D#  E -- F# -- G#  A --
```

### Find Chord Positions

Discover where chord voicings exist across different pedal/lever combinations:

```bash
cargo run -- chord \
  --tuning-name "E9" \
  --tuning "F#, D#, G#, E, B, G#, F#, E, D, B" \
  --chord "E major"
```

## Usage Examples

**Common E9 tuning:**
```bash
cargo run -- scale --tuning-name "E9" --tuning "F#, D#, G#, E, B, G#, F#, E, D, B" --scale "E major"
```

**C6 tuning:**
```bash
cargo run -- scale --tuning-name "C6" --tuning "E, C, A, G, E, C" --scale "C major"
```

**Blues scale:**
```bash
cargo run -- scale --tuning-name "E9" --tuning "F#, D#, G#, E, B, G#, F#, E, D, B" --scale "E blues"
```

## Supported Scales

- Major/Ionian
- Minor/Aeolian
- Dorian, Phrygian, Lydian, Mixolydian, Locrian
- Harmonic Minor, Melodic Minor
- Pentatonic Major, Pentatonic Minor
- Blues, Chromatic, Whole Tone

## Supported Chords

Triads: Major, Minor, Suspended2, Suspended4, Augmented, Diminished

Seventh chords: Major 7th, Minor 7th, Augmented 7th, Augmented Major 7th, Diminished 7th, Half Diminished 7th, Minor Major 7th, Dominant 7th

Extended chords: Dominant/Major/Minor 9th, 11th, and 13th

## Project Structure

```
pedal-steel/
├── src/
│   ├── main.rs       # CLI interface and command parsing
│   ├── lib.rs        # Library module exports
│   ├── copedent.rs   # Pedal and lever change definitions
│   ├── guitar.rs     # Guitar/neck representation and note identification
│   ├── tunings.rs    # Tuning string parsing
│   └── display.rs    # Pretty-printing for CLI output
├── Cargo.toml        # Rust dependencies
└── README.md         # This file
```

## Development

### Code Formatting

Format code with rustfmt:

```bash
cargo fmt
```

### Linting

Run Clippy for linting:

```bash
cargo clippy -- -D warnings
```

### Testing

Run the test suite:

```bash
cargo test
```

## Current Limitations and Future Work

### Known Limitations

- **Fixed copedent:** The copedent is hardcoded for a specific E9 configuration. Different players use different copedents.
- **10-string only:** Currently assumes a 10-string pedal steel guitar.
- **12 frets:** Display is limited to the first 12 frets.
- **Position combinations:** Only shows predefined position combinations, not all possible pedal/lever combinations.
- **No audio playback:** This is a visual tool only; it doesn't generate sound.

### Potential Enhancements

- [ ] **Custom copedent configuration:** Allow users to define their own pedal and lever changes via config file, CLI flags, or UI
- [ ] **Flexible string count:** Support copedants for non-10-string configurations
- [ ] **Extended fret range:** Display option for more than 12 frets
- [ ] **Interactive mode:** UI for exploring positions interactively, possibly with WASM
- [ ] **Export functionality:** Generate printable PDFs or images of neck diagrams
- [ ] **Preset tunings:** Library of common pedal steel tunings (E9, C6, Extended E9, etc.)
- [ ] **Bar position suggestions:** Suggest optimal bar positions for chord transitions
- [ ] **Fretboard color schemes:** Different visualization styles for better readability

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

This project is dual-licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.

## Acknowledgments

- Built with [rust-music-theory](https://github.com/ozankasikci/rust-music-theory) for music theory primitives
- Scale and chord lists adapted from rust-music-theory examples
- Inspired by the rich tradition of pedal steel guitar playing
