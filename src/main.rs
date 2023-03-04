mod scales;

use crate::scales::*;

fn main() {
	// ARRAY TO VEC
	// let array: [u8; 3] = [9, 8, 7];
	// let a = array.to_vec();
	// println!("{:?}\n{:?}", array, a);

	// VEC TO ARRAY
	// let vektor: Vec<u8> = vec![1,2,3,4,5];
	// let b: [u8; 5] = vektor.try_into().unwrap();
	// println!("{:?}", b);

	println!("Step number:\t1\t2\t3\t4\t5\t6\t7\t1");
	println!("MAJOR_IONIAN: \t{}", scale_finder('C', ' ', "major"));
	println!("DORIAN: \t{}", scale_finder('C', ' ', "dorian"));
	println!("PHRYGIAN: \t{}", scale_finder('C', ' ', "phrygian"));
	println!("LYDIAN: \t{}", scale_finder('C', ' ', "lydian"));
	println!("MIXOLYDIAN: \t{}", scale_finder('C', ' ', "mixolydian"));
	println!("MINOR_AEOLIAN: \t{}", scale_finder('C', ' ', "aeolian"));
	println!("LOCRIAN: \t{}", scale_finder('C', ' ', "locrian"));
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
		_ => (' ', 0)
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
	for i in note_sequence_tuple.clone() {
		note_semitones.push(i.1)
	}
	let mut note_sequence: Vec<char> = vec![];
	for i in note_sequence_tuple {
		note_sequence.push(i.0)
	}

	(note_semitones, note_sequence)
}

// Check if note_str is correct: CDEFGAH or B
// Check for accidentals: flat and sharp
#[allow(unused)]
fn note_and_acc(note: char, acc: char) -> (char, char) {
	let upper = note.to_uppercase().to_string().chars().next().expect("string is empty");
	let mut note_name: char = ' ';
	if "CDEFGAH".contains(upper) {
		note_name = upper
	} else if upper == 'B' {
		note_name = 'H'
	} else {
		note_name = ' '
	}
	
	let mut any_acc = ' ';
	if acc == 'b' || acc == 'B' { any_acc = 'b'  }
	if acc == '#' { any_acc = '#' }

	(note_name, any_acc)
}

#[allow(unused)]
fn scale_finder(note: char, acc: char, scale: &str) -> String {
	let (note_name, any_acc) = note_and_acc(note, acc);
	if note_name == ' ' {return "".to_string()}

	let scale_name = &scale[..];
	let current_scale = match scale_name {
		"major" => MAJOR_IONIAN,
		"ionian" => MAJOR_IONIAN,
		"dorian" => DORIAN,
		"phrygian" => PHRYGIAN,
		"lydian" => LYDIAN,
		"mixolydian" => MIXOLYDIAN,
		"minor" => MINOR_AEOLIAN,
		"aeolian" => MINOR_AEOLIAN,
		"locrian" => LOCRIAN,
		&_ => [0,0,0,0,0,0,0,0,]
	};

	let (note_semitones, note_sequence) = sequencer(note_name);

	let mut empty_string = "".to_string();
	let mut index = 0;
	let mut shift_up = false;
	let mut shift_down = false;
	
	for i in note_semitones {
		if i == current_scale[index] {
			if !shift_down && !shift_up {
				empty_string = format!("{}{}\t", empty_string, note_sequence[index]);
			}
			if shift_down {
				empty_string = format!("{}{}{}\t", empty_string, note_sequence[index], 'b');
			}
			if shift_up {
				empty_string = format!("{}{}{}\t", empty_string, note_sequence[index], '#');
			}
		}
		else if i < current_scale[index] && !shift_up && !shift_down {
			empty_string = format!("{}{}\t", empty_string, note_sequence[index]);
			shift_up = true
		}
		else if i > current_scale[index] && !shift_up && !shift_down {
			empty_string = format!("{}{}\t", empty_string, note_sequence[index]);
			shift_down = true
		}
		else if i < current_scale[index] && shift_up && !shift_down {
			empty_string = format!("{}{}{}\t", empty_string, note_sequence[index], '#');
		}
		else if i > current_scale[index] && shift_up && !shift_down {
			empty_string = format!("{}{}{}\t", empty_string, note_sequence[index], '#');
			shift_up = false
		}
		
		else if i > current_scale[index] && shift_down && !shift_up {
			empty_string = format!("{}{}{}\t", empty_string, note_sequence[index], 'b');
		}
		else if i < current_scale[index] && shift_down && !shift_up {
			empty_string = format!("{}{}{}\t", empty_string, note_sequence[index], 'b');
			shift_down = false
		}	
		index += 1
	}
	empty_string
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sequence() {
		let e_u8 = vec![1, 2, 2, 2, 1, 2, 2, 1];
		let e_char = vec!['E', 'F', 'G', 'A', 'H', 'C', 'D', 'E'];
		let a_u8 = vec![2, 1, 2, 2, 1, 2, 2, 2];
		let a_char = vec!['A', 'H', 'C', 'D', 'E', 'F', 'G', 'A'];
		assert_eq!((e_u8, e_char), sequencer('E'));
		assert_eq!((a_u8, a_char), sequencer('A'))
	}

	#[test]
	fn wrong_char() {
		assert_eq!("".to_string(), scale_finder('Q', ' ', "minor"))
	}

	#[test]
	fn minor_no_acc() {
		assert_eq!("C\tD\tEb\tF\tG\tAb\tHb\tC\t".to_string(),  scale_finder('c', ' ', "minor"));
	 	assert_eq!("D\tE\tF\tG\tA\tHb\tC\tD\t".to_string(),    scale_finder('d', ' ', "minor"));
	 	assert_eq!("E\tF#\tG\tA\tH\tC\tD\tE\t".to_string(),    scale_finder('E', ' ', "minor"));
	 	assert_eq!("F\tG\tAb\tHb\tC\tDb\tEb\tF\t".to_string(), scale_finder('F', ' ', "minor"));
	 	assert_eq!("G\tA\tHb\tC\tD\tEb\tF\tG\t".to_string(),   scale_finder('G', ' ', "minor"));
		assert_eq!("A\tH\tC\tD\tE\tF\tG\tA\t".to_string(),     scale_finder('A', ' ', "minor"));
		assert_eq!("H\tC#\tD\tE\tF#\tG\tA\tH\t".to_string(),   scale_finder('H', ' ', "minor"));
		assert_eq!("C\tD\tE\tF\tG\tA\tH\tC\t".to_string(),     scale_finder('C', ' ', "major"))
	}
}