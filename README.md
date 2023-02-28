# scale_finder_2
New version written from scratch. The old one is [here](https://github.com/antonovmike/scale_finder).

Goal n1 is to find out how to implement some algorithms for calculating each of diatonic scales, fox example: E flat major, A minor.
Goal n2 convert wrong accidentals to correct ones for example C sharp major should be D flat major. Because we don't want to use weird cases like E sharp wich is F or Fb wich is E and C sharp witch is H and Hb wich is C.

Note H is also known as B, but I really don't like B flat - Bb. So I use H instead.

Diatonic scales can be represented as a matrix. Each scale has 7 steps. Distance between steps can be 1 or 2 semitones. Each diatonic scale has only two 1 semitone steps and five 2 semitone steps. The sequence is always 2,2,1,2,2,2,1 and just shifted one step for each next scale. In the end of scale there is always an additional step - the 1 step.
```bash
Step's number    1  2  3  4  5  6  7  1

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
