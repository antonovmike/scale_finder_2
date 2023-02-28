# scale_finder_2
New version written from scratch
```bash
                 1  2  3  4  5  6  7  1

Major / Ionian  [2, 2, 1, 2, 2, 2, 1, 2]
Dorian          [2, 1, 2, 2, 2, 1, 2, 2]
Phrygian        [1, 2, 2, 2, 1, 2, 2, 2]
Lydian          [2, 2, 2, 1, 2, 2, 2, 1]
Mixolydian      [2, 2, 1, 2, 2, 2, 1, 2]
Minor / Aeolian [2, 1, 2, 2, 2, 1, 2, 2]
Locrian         [1, 2, 2, 2, 1, 2, 2, 2]
```
Each note length in semitones
```bash
C D E F G A H
2 2 1 2 2 2 1
```
Example 1
```bash
Mixolydian = [2, 2, 1, 2, 2, 1, 2]
G          = [2, 2, 1, 2, 2, 1, 2]
Mixolydian G: 2, 2, 1, 2, 2, 1, 2
```
Accidentals make note lower or higher. But the length of each note should be 1 or two. So if we will make C 1 semitone lower, we should shift D and E. Last one is 1 semitone, so the next note F can be not alterated. 

'b' - Flat makes note 1 semitone lower

'#' - Sharp makes note 1 semitone higher

Example 2
```bash
Phrygian = [1, 2, 2, 2, 1, 2, 2]
F        = [2, 2, 2, 1, 2, 2, 1]
Phrygian F: F  Gb Ab Hb C  Db Eb
```
Example 3
```bash
Minor    = [2, 1, 2, 2, 1, 2, 2]
H        = [1, 2, 2, 1, 2, 2, 2]
Minor H:    H  C# D  E  F# G  A
```
