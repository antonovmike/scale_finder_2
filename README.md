# scale_finder_2
New version written from scratch. The old one is [here](https://github.com/antonovmike/scale_finder).
I started this project at december 2021 after 1.5 month of studying Rust. Year later I made a brand new version. But the idea is the same: build any of 7 diatonic scales using sharps and flats.

TODO: Error handlers

Diatonic scales can be represented as a matrix. Each scale has 7 steps. Distance between steps can be 1 or 2 semitones. Each diatonic scale has only two 1 semitone steps and five 2 semitone steps. The sequence is always 2,2,1,2,2,2,1 and just shifted one step for each next scale. In the end of scale there is always an additional step - the 1 step.
```bash
Step\'s number    1  2  3  4  5  6  7  1

Major / Ionian  [2, 2, 1, 2, 2, 2, 1, 2]
Dorian          [2, 1, 2, 2, 2, 1, 2, 2]
Phrygian        [1, 2, 2, 2, 1, 2, 2, 2]
Lydian          [2, 2, 2, 1, 2, 2, 2, 1]
Mixolydian      [2, 2, 1, 2, 2, 2, 1, 2]
Minor / Aeolian [2, 1, 2, 2, 2, 1, 2, 2]
Locrian         [1, 2, 2, 2, 1, 2, 2, 2]
```
Each note has it's own length in semitones
```bash
C D E F G A H
2 2 1 2 2 2 1
```
If the length of the note is the same as the length of the step, then the note remains without change.

Example 1
```bash
Mixolydian = [2, 2, 1, 2, 2, 1, 2, 2]
G          = [2, 2, 1, 2, 2, 1, 2, 2]
Mixolydian G: G  A  H  C  D  E  F  G
```
Accidentals make note lower or higher. But the length of each note should be 1 or two. So if we will make C 1 semitone lower, we should shift D and E. Last one is 1 semitone, so the next note F can be not alternated. 

'b' - Flat makes note 1 semitone lower

'#' - Sharp makes note 1 semitone higher

Example 2
```bash
Phrygian = [1, 2, 2, 2, 1, 2, 2, 2]
F        = [2, 2, 2, 1, 2, 2, 1, 2]
Phrygian F: F  Gb Ab Hb C  Db Eb F
```
Example 3
```bash
Minor    = [2, 1, 2, 2, 1, 2, 2, 2]
H        = [1, 2, 2, 1, 2, 2, 2, 1]
Minor H:    H  C# D  E  F# G  A  H
```

The algorithm:

If MINOR 2 STEP 1 => next notes are sharp

Until MINOR 1 STEP 2 => next notes are regular

If MINOR 1 STEPS 2 => next notes are flat

Until MINOR 2 STEPS 1 => next notes are regular

**Wrong shrp/flat issue**

It is impossible to completely avoid using the "wrong" sharps or flats because there are scales that will have the wrong flats or sharps. Examples:

```bash
Major / Ionian Cb or H# and E#
Eb      F   Gb      Ab      Hb  Cb      Db      Eb
D#      E#  F#      G#      A#  H       C#      D#

Major / Ionian Cb or H#
Gb      Ab      Hb  Cb      Db      Eb      F   Gb
F#      G#      A#  H       C#      D#      E#  F#

Dorian Cb or E#
Ab      Hb  Cb      Db      Eb      F   Gb      Ab
G#      A#  H       C#      D#      E#  F#      G#

Phrygian Hb and Fb or E#
Hb  Cb      Db      Eb      F   Gb      Ab      Hb
A#  H       C#      D#      E#  F#      G#      A#
```
This very issue can not bo solved at all. Some scales just 
gonna be loops. It causes stack overflow