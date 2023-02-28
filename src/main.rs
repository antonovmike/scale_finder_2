mod scales;
use crate::scales::*;

#[allow(unused)]

fn main() {
    modes('F', PHRYGIAN);
}

fn modes(current_note: char, current_mode: [u8; 8]) -> String {
	let notes = [ 'C', 'D', 'E', 'F', 'G', 'A', 'H' ];
	let notes_tuples = [ ('C', 2), ('D', 2), ( 'E', 1), ('F', 2), ('G', 2), ('A', 2), ('H', 1)];

	let index = notes.iter().position(|&r| r == current_note).unwrap();
    let mut index_a = index;
	let mut index_mode = 0;
	let mut mode_sharps = "".to_string();
	let mut mode_flats  = "".to_string();

	for i in index_a..notes.len() {
		println!("{}. Distance = {}\tCurrent note is {} and it's lenght = {}", 
			index_mode + 1, current_mode[index_mode], notes_tuples[index_a].0, notes_tuples[index_a].1
		);
		mode_sharps.insert(mode_sharps.len(), notes_tuples[index_a].0);
		mode_flats.insert(mode_flats.len(), notes_tuples[index_a].0);
		if i > 0 {
			if notes_tuples[index_a].1 < current_mode[index_mode] {
				mode_flats.insert(mode_flats.len(), 'b');
				println!("This note {}/{} should be FLAT", i, notes_tuples[index_a].0)
			}
			if notes_tuples[index_a].1 > current_mode[index_mode] {
				mode_sharps.insert(mode_sharps.len(), '#');
				println!("This note {}/{} should be SHARP", i, notes_tuples[index_a].0)
			}
		}

		index_a += 1;
		index_mode += 1;
	}
	if index > 0 && index < 7 {
		for i in 0..index {
			println!("{}. DISTANCE = {}\tCurrent note is {} and it's lenght = {}", 
				index_mode + 1, current_mode[index_mode], notes_tuples[i].0, notes_tuples[index].1
			);
			mode_sharps.insert(mode_sharps.len(), notes_tuples[i].0);
			mode_flats.insert(mode_flats.len(), notes_tuples[i].0);
			if notes_tuples[i].1 < current_mode[index_mode] {
				mode_flats.insert(mode_flats.len(), 'b');
				println!("This note {}/{} should be FLAT", i, notes_tuples[i].0)
			}
			if notes_tuples[i].1 > current_mode[index_mode] {
				mode_sharps.insert(mode_sharps.len(), '#');
				println!("This note {}/{} should be SHARP", i, notes_tuples[i].0)
			}
		}
		index_a += 1;
		index_mode += 1;
	}
	
	let sharp_vec_of_chars: Vec<char> = mode_sharps.chars().collect();
    let flat_vec_of_chars:  Vec<char> = mode_flats.chars().collect();
	let mut sharp_vec_of_u8: Vec<u8> = vec![];
	let mut flat_vec_of_u8: Vec<u8> = vec![];
	for s in 0..sharp_vec_of_chars.len() {
		
	}
	
	println!("SHARPS {}", mode_sharps);
	println!("FLATS  {}", mode_flats);

	mode_flats
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phrygian_c() {
		assert_eq!(("CDbEbFGAbHb".to_string()), modes('C', PHRYGIAN))
	}
    #[test]
    fn phrygian_d() {
		assert_eq!(("DEbFGAHbC".to_string()), modes('D', PHRYGIAN))
	}
    #[test]
    fn phrygian_e() {
		assert_eq!(("EFGAHCD".to_string()), modes('E', PHRYGIAN))
	}
    // #[test]
    // fn phrygian_f() {
	// 	assert_eq!(("FGbAbHbCDbEb".to_string()), modes('F', phrygian))
	// }
    // #[test]
    // fn phrygian_g() {
	// 	assert_eq!(("GAbHbCDEbF".to_string()), modes('G', phrygian))
	// }
    // #[test]
    // fn phrygian_a() {
	// 	assert_eq!(("AHbCDEFG".to_string()), modes('A', phrygian))
	// }
    // #[test]
    // fn phrygian_h() {
	// 	assert_eq!(("HCDEF#GA".to_string()), modes('A', phrygian))
	// }
}
