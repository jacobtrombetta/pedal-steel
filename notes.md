# Pedal Steel

## Tunings
Pedal steel guitar
E9th: F#, D#, G#, E, B, G#, F#, E, D, B
      9   7   3   R  5  3   9   R  7b 5

8 string lap steel guitar tunings
E13th (Don Helms): G# E C# B G# E C# A
E13th (Little Roy Wiggins): G# E C# B G# E D B
A6: E C# A F# E C# A F#
    5 3  R 6  5 3  R 6
C6: E C  A G  E C  A G
    3 R  6 5  3 R  6  5
C6 (Cindy Cashdollar): G E C A G E C A

# Lap steel

## From Chris Scruggs classes
Noel Boggs, Herb Remmington on Bob Wills -> A6th
A6th -> melodies
E13th -> strum, bar slants, chordal solo (tough neck to solo on)

### Western Swing 1
Trick 1 -> Speedy West bar hit and increase tone knob
Trick 2 -> Speedy West bar and palm slap
Volume knob
Chimes
C6th tuning
P1 -> Hold bar with fingers down behind it (hand placement)
P2 -> Pick hand plam blocking (practice pick, slide up, mute, repeat)
P3 -> Bar practice, only have bar over strings being played, slant forward and backward
P4 -> Alternate picking (thumb, one) Ex1
P5 -> Ex2 blocking, bar, practice

### Finding the minor
Every vi has a built in relative minor
E -> C#m
I    vi

E13th 6th in 3rd string
to play major don't play 3rd string
to play minor don't play 4th string

I  IV  V
vi ii iii

C  F  G
Am Dm Em

9th chords & 7th can be used as lead ins or blusy voicing

minor - to find minor go up 3 frets & ignore 4th string
9th - down 2 frets and ignore 4th string

## Development
```
cargo clippy -- -D warnings -W clippy::pedantic -W clippy::nursery
cargo fmt
```

### Helpful links for development
https://github.com/ozankasikci/rust-music-theory/
https://docs.rs/rust-music-theory/0.3.0/rust_music_theory/chord/struct.Chord.html
