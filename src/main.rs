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

	let c_steps: [u8; 8] = [2, 2, 1, 2, 2, 2, 1, 2];
	println!("Step number:\t1\t2\t3\t4\t5\t6\t7\t1");
	println!("MAJOR_IONIAN: \t{}", scale_finder('C', c_steps, "major"));
	println!("DORIAN: \t{}", scale_finder('C', c_steps, "dorian"));
	println!("PHRYGIAN: \t{}", scale_finder('C', c_steps, "phrygian"));
	println!("LYDIAN: \t{}", scale_finder('C', c_steps, "lydian"));
	println!("MIXOLYDIAN: \t{}", scale_finder('C', c_steps, "mixolydian"));
	println!("MINOR_AEOLIAN: \t{}", scale_finder('C', c_steps, "aeolian"));
	println!("LOCRIAN: \t{}", scale_finder('C', c_steps, "locrian"));
}

fn sequencer(note_name: char) -> Vec<char> {
// Split octave at the root note
	let split_here = OCTAVE.iter().position(|&r| r == note_name).unwrap();
	let (first, second) = OCTAVE.split_at(split_here);
// Unite both parts. Now note sequense starts with root note
	let mut a = first.to_vec();
	a.push(note_name);
	let mut b = second.to_vec();
	b.append(&mut a);
	return b
}

fn scale_finder(note_name: char, note_steps: [u8; 8], scale: &str) -> String {
// Replace note_steps with OCTAVE_STEPS and remove note_steps
// OCTAVE_STEPS[2].0 is a char
// OCTAVE_STEPS[2].1 is a semitone

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

	let note_sequence = sequencer(note_name);
	let mut empty_string = "".to_string();
	let mut index = 0;
	let mut shift_up = false;
	let mut shift_down = false;
	
	for i in note_steps {
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
		assert_eq!(vec!['E', 'F', 'G', 'A', 'H', 'C', 'D', 'E'], sequencer('E'));
		assert_eq!(vec!['A', 'H', 'C', 'D', 'E', 'F', 'G', 'A'], sequencer('A'))
	}

	#[test]
	fn c_minor() {
		let c_steps: [u8; 8] = [2, 2, 1, 2, 2, 2, 1, 2];
		assert_eq!("C\tD\tEb\tF\tG\tAb\tHb\tC\t".to_string(), scale_finder('C', c_steps, "minor"))
	}
	#[test]
	fn d_minor() {
	 	let d_steps: [u8; 8] = [2, 1, 2, 2, 2, 1, 2, 2];
	 	assert_eq!("D\tE\tF\tG\tA\tHb\tC\tD\t".to_string(), scale_finder('D', d_steps, "minor"))
	}
	#[test]
	fn e_minor() {
	 	let e_steps: [u8; 8] = [1, 2, 2, 2, 1, 2, 2, 2];
	 	assert_eq!("E\tF#\tG\tA\tH\tC\tD\tE\t".to_string(), scale_finder('E', e_steps, "minor"))
	}
	#[test]
	fn f_minor() {
	 	let f_steps: [u8; 8] = [2, 2, 2, 1, 2, 2, 1, 2];
	 	assert_eq!("F\tG\tAb\tHb\tC\tDb\tEb\tF\t".to_string(), scale_finder('F', f_steps, "minor"))
	}
	#[test]
	fn g_minor() {
	 	let g_steps: [u8; 8] = [2, 2, 1, 2, 2, 1, 2, 2];
	 	assert_eq!("G\tA\tHb\tC\tD\tEb\tF\tG\t".to_string(), scale_finder('G', g_steps, "minor"))
	}
	#[test]
	fn a_minor() {
		let a_steps: [u8; 8] = [2, 1, 2, 2, 1, 2, 2, 2];
		assert_eq!("A\tH\tC\tD\tE\tF\tG\tA\t".to_string(), scale_finder('A', a_steps, "minor"))
	}
	#[test]
	fn h_minor() {
		let h_steps: [u8; 8] = [1, 2, 2, 1, 2, 2, 2, 2];
		assert_eq!("H\tC#\tD\tE\tF#\tG\tA\tH\t".to_string(), scale_finder('H', h_steps, "minor"))
	}
	#[test]
	fn c_major() {
		let c_steps: [u8; 8] = [2, 2, 1, 2, 2, 2, 1, 2];
		assert_eq!("C\tD\tE\tF\tG\tA\tH\tC\t".to_string(), scale_finder('C', c_steps, "major"))
	}
}