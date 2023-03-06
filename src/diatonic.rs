use crate::scales::*;

#[allow(unused)]
pub fn octave() -> String {
    "C  D  E F  G  A  H C\n2  2  1 2  2  2  1 2".to_string()
}

#[allow(unused)]
pub fn semitones(scale: &str) -> String {
    let current_scale = match &scale.to_lowercase()[..] {
        "major" => MAJOR_IONIAN,
        "ionian" => MAJOR_IONIAN,
        "dorian" => DORIAN,
        "phrygian" => PHRYGIAN,
        "lydian" => LYDIAN,
        "mixolydian" => MIXOLYDIAN,
        "minor" => MINOR_AEOLIAN,
        "aeolian" => MINOR_AEOLIAN,
        "locrian" => LOCRIAN,
        &_ => [0, 0, 0, 0, 0, 0, 0, 0],
    };
    let mut answer = "".to_string();
    for i in current_scale {
        if i == 1 {
            answer.push('1');
            answer.push(' ')
        } else {
            answer.push('2');
            answer.push(' ');
            answer.push(' ')
        };
    }
    format!("{}: {}", scale.to_uppercase(), answer)
}

#[allow(unused)]
pub fn scale_builder(note: char, acc: char, scale: &str) -> String {
    let (mut note_name, any_acc) = note_and_acc(note, acc);
    if note_name == 'X' { return "".to_string() }
    if any_acc   == 'X' { return "".to_string() }

    let mut swap = false;
    if note == 'E' && any_acc == '#' {
        swap = true;
        note_name = 'F'
    }
    if note == 'F' && any_acc == 'b' {
        swap = true;
        note_name = 'E'
    }
    if note == 'H' && any_acc == '#' {
        swap = true;
        note_name = 'C'
    }

    if note == 'C' && any_acc == 'b' {
        swap = true;
        note_name = 'H'
    }

    let current_scale = match scale {
        "major"      => MAJOR_IONIAN,
        "ionian"     => MAJOR_IONIAN,
        "dorian"     => DORIAN,
        "phrygian"   => PHRYGIAN,
        "lydian"     => LYDIAN,
        "mixolydian" => MIXOLYDIAN,
        "minor"      => MINOR_AEOLIAN,
        "aeolian"    => MINOR_AEOLIAN,
        "locrian"    => LOCRIAN,
        &_ => [0, 0, 0, 0, 0, 0, 0, 0],
    };

    let (note_semitones, note_sequence) = sequencer(note_name);

    let mut empty_string = "".to_string();
    let mut index = 0;
    let mut shift_up = false;
    let mut shift_down = false;

    if any_acc == 'b' && !swap { shift_down = true } 
	if any_acc == '#' && !swap { shift_up   = true }

    for i in note_semitones {
        if i == current_scale[index] {
            if !shift_down && !shift_up {
                empty_string = format!("{}{}", empty_string, note_sequence[index]);
            }
            if shift_down && !shift_up {
                empty_string = format!("{}{}{}", empty_string, note_sequence[index], 'b');
            }
            if shift_up && !shift_down {
                empty_string = format!("{}{}{}", empty_string, note_sequence[index], '#');
            }
        } else if i < current_scale[index] && !shift_up && !shift_down {
            empty_string = format!("{}{}", empty_string, note_sequence[index]);
            shift_up = true
        } else if i > current_scale[index] && !shift_up && !shift_down {
            empty_string = format!("{}{}", empty_string, note_sequence[index]);
            shift_down = true
        } else if i < current_scale[index] && shift_up && !shift_down {
            empty_string = format!("{}{}{}", empty_string, note_sequence[index], '#');
        } else if i > current_scale[index] && shift_up && !shift_down {
            empty_string = format!("{}{}{}", empty_string, note_sequence[index], '#');
            shift_up = false
        } else if i > current_scale[index] && shift_down && !shift_up {
            empty_string = format!("{}{}{}", empty_string, note_sequence[index], 'b');
        } else if i < current_scale[index] && shift_down && !shift_up {
            empty_string = format!("{}{}{}", empty_string, note_sequence[index], 'b');
            shift_down = false
        }
        index += 1
    }

    // let wrong_root = match note {
    //     'C' => ('C', 2),
    //     'D' => ('D', 2),
    //     'G' => ('G', 2),
    //     'A' => ('A', 2),
    //     _ => ('X', 0)
    // };
    // find this note's index in OCTAVE_STEPS
    // find next note after the current
    // run scale_builder with flat root note
    // if empty_string.contains("E#") {
    //     let nn = OCTAVE_STEPS.binary_search(&wrong_root).unwrap();
    //     let n = if nn == 6 { OCTAVE_STEPS[1].0 } else { OCTAVE_STEPS[nn+1].0 };
    //     scale_builder(n, 'b', scale)
    // } else if empty_string.contains("Fb") {
        // return "ERROR Fb".to_string()
    // } else if empty_string.contains("H#") {
    //     let nn = OCTAVE_STEPS.binary_search(&wrong_root).unwrap();
    //     let n = if nn == 6 { OCTAVE_STEPS[1].0 } else { OCTAVE_STEPS[nn+1].0 };
    //     scale_builder(n, 'b', scale)
    // } else if empty_string.contains("Cb") {
    //     return "ERROR Cb".to_string()
    // } else {return empty_string}

    return empty_string
}

fn sequencer(note_name: char) -> (Vec<u8>, Vec<char>) {
    let note_step: (char, u8) = match note_name {
        'C' => ('C', 2),
        'D' => ('D', 2),
        'E' => ('E', 1),
        'F' => ('F', 2),
        'G' => ('G', 2),
        'A' => ('A', 2),
        'H' => ('H', 1),
        'B' => ('H', 1),
        _ => (' ', 0),
    };
    // Split octave at the root note
    let split_here = OCTAVE_STEPS.iter().position(|&r| r == note_step).unwrap();
    let (first, second) = OCTAVE_STEPS.split_at(split_here);
    // Unite both parts. Now note sequense starts with root note
    let mut a = first.to_vec();
    a.push(note_step);
    let mut b = second.to_vec();
    b.append(&mut a);

    let note_sequence_tuple = b;
    let mut note_semitones: Vec<u8> = vec![];
    for i in note_sequence_tuple.clone() { note_semitones.push(i.1) }

    let mut note_sequence: Vec<char> = vec![];
    for i in note_sequence_tuple { note_sequence.push(i.0) }

    (note_semitones, note_sequence)
}

// Check if note_str is correct: CDEFGAH or B
// Check for accidentals: flat and sharp
#[allow(unused)]
fn note_and_acc(note: char, acc: char) -> (char, char) {
    let note_upper = note
        .to_uppercase()
        .to_string()
        .chars()
        .next()
        .expect("string is empty");
    let mut note_name: char = ' ';

    if "CDEFGAH".contains(note_upper) { note_name = note_upper } 
	else if note_upper == 'B' { note_name = 'H' } 
	else { note_name = 'X' }

    let mut any_acc = 'X';
    if acc == 'b' || acc == 'B' { any_acc = 'b' }
    if acc == '#' { any_acc = '#' }
    if acc == ' ' { any_acc = ' ' }

    (note_name, any_acc)
}
