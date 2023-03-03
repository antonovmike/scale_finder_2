mod scales;
use crate::scales::*;

#[allow(unused)]
const NOTES: [char; 8] = ['C', 'D', 'E', 'F', 'G', 'A', 'H', 'C'];
#[allow(unused)]
const STEPS: [u8; 8] = [2, 2, 1, 2, 2, 2, 1, 2];

fn main() {
	let a = scale_finder(NOTES, STEPS, MINOR_AEOLIAN);
    println!("{a}");
    println!("Minor: C -- D - Eb -- F -- G - Ab - Hb -- C");
}

fn scale_finder(note_sequence: [char; 8], note_steps: [u8; 8], scale: [u8; 8]) -> String {
	let mut empty_string = "".to_string();
	let mut index = 0;
	let mut shift_up = false;
	let mut shift_down = false;
	
	for i in note_steps {
		if i == scale[index] {
			if !shift_down && !shift_up {
				empty_string = format!("{}{}", empty_string, note_sequence[index]);
			}
			if shift_down {
				empty_string = format!("{}{}{}", empty_string, note_sequence[index], 'b');
			}
			if shift_up {
				empty_string = format!("{}{}{}", empty_string, note_sequence[index], '#');
			}
		}
		else if i < scale[index] && !shift_up && !shift_down {
			empty_string = format!("{}{}", empty_string, note_sequence[index]);
			shift_up = true
		}
		else if i > scale[index] && !shift_up && !shift_down {
			empty_string = format!("{}{}", empty_string, note_sequence[index]);
			shift_down = true
		}
		else if i < scale[index] && shift_up && !shift_down {
			empty_string = format!("{}{}{}", empty_string, note_sequence[index], '#');
		}
		else if i > scale[index] && shift_up && !shift_down {
			empty_string = format!("{}{}{}", empty_string, note_sequence[index], '#');
			shift_up = false
		}
		
		else if i > scale[index] && shift_down && !shift_up {
			empty_string = format!("{}{}{}", empty_string, note_sequence[index], 'b');
		}
		else if i < scale[index] && shift_down && !shift_up {
			empty_string = format!("{}{}{}", empty_string, note_sequence[index], 'b');
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
	fn c_minor() {
		assert_eq!("CDEbFGAbHbC".to_string(), scale_finder(NOTES, STEPS, MINOR_AEOLIAN))
	}
	#[test]
	fn d_minor() {
	 let d: [char; 8] = ['D', 'E', 'F', 'G', 'A', 'H', 'C', 'D'];
	 let d_steps: [u8; 8] = [2, 1, 2, 2, 2, 1, 2, 2];
	 assert_eq!("DEFGAHbCD".to_string(), scale_finder(d, d_steps, MINOR_AEOLIAN))
	}
	#[test]
	fn e_minor() {
	 let e: [char; 8] = ['E', 'F', 'G', 'A', 'H', 'C', 'D', 'E'];
	 let e_steps: [u8; 8] = [1, 2, 2, 2, 1, 2, 2, 2];
	 assert_eq!("EF#GAHCDE".to_string(), scale_finder(e, e_steps, MINOR_AEOLIAN))
	}
	#[test]
	fn f_minor() {
	 let f: [char; 8] = ['F', 'G', 'A', 'H', 'C', 'D', 'E', 'F'];
	 let f_steps: [u8; 8] = [2, 2, 2, 1, 2, 2, 1, 2];
	 assert_eq!("FGAbHbCDbEbF".to_string(), scale_finder(f, f_steps, MINOR_AEOLIAN))
	}
	#[test]
	fn g_minor() {
	 let g: [char; 8] = ['G', 'A', 'H', 'C', 'D', 'E', 'F', 'G'];
	 let g_steps: [u8; 8] = [2, 2, 1, 2, 2, 1, 2, 2];
	 assert_eq!("GAHbCDEbFG".to_string(), scale_finder(g, g_steps, MINOR_AEOLIAN))
	}
	#[test]
	fn a_minor() {
		let a: [char; 8] = ['A', 'H', 'C', 'D', 'E', 'F', 'G', 'A'];
		let a_steps: [u8; 8] = [2, 1, 2, 2, 1, 2, 2, 2];
		assert_eq!("AHCDEFGA".to_string(), scale_finder(a, a_steps, MINOR_AEOLIAN))
	}
	#[test]
	fn h_minor() {
		let h: [char; 8] = ['H', 'C', 'D', 'E', 'F', 'G', 'A', 'H'];
		let h_steps: [u8; 8] = [1, 2, 2, 1, 2, 2, 2, 2];
		assert_eq!("HC#DEF#GAH".to_string(), scale_finder(h, h_steps, MINOR_AEOLIAN))
	}
	#[test]
	fn c_major() {
		let c_maj: [char; 8] = ['C', 'D', 'E', 'F', 'G', 'A', 'H', 'C'];
		let c_steps: [u8; 8] = [2, 2, 1, 2, 2, 2, 1, 2];
		assert_eq!("CDEFGAHC".to_string(), scale_finder(c_maj, c_steps, MAJOR_IONIAN))
	}
}