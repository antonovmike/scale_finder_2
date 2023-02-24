#[allow(unused)]
#[allow(dead_code)]

fn main() {
	// let major_ionian  = [2, 2, 1, 2, 2, 2, 1, 2];
	// let dorian        = [2, 1, 2, 2, 2, 1, 2, 2];
	let phrygian= [1, 2, 2, 2, 1, 2, 2, 2];
	// let lydian        = [2, 2, 2, 1, 2, 2, 2, 1];
	// let mixolydian    = [2, 2, 1, 2, 2, 2, 1, 2];
	// let minor_aeolian = [2, 1, 2, 2, 2, 1, 2, 2];
	// let locrian       = [1, 2, 2, 2, 1, 2, 2, 2];
    modes('F', phrygian);
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
		let phrygian = [1, 2, 2, 2, 1, 2, 2, 2];
		assert_eq!(("CDbEbFGAbHb".to_string()), modes('C', phrygian))
	}
    #[test]
    fn phrygian_d() {
		let phrygian = [1, 2, 2, 2, 1, 2, 2, 2];
		assert_eq!(("DEbFGAHbC".to_string()), modes('D', phrygian))
	}
    #[test]
    fn phrygian_e() {
		let phrygian = [1, 2, 2, 2, 1, 2, 2, 2];
		assert_eq!(("EFGAHCD".to_string()), modes('E', phrygian))
	}
    // #[test]
    // fn phrygian_f() {
	// 	let phrygian = [1, 2, 2, 2, 1, 2, 2, 2];
	// 	assert_eq!(("FGbAbHbCDbEb".to_string()), modes('F', phrygian))
	// }
    // #[test]
    // fn phrygian_g() {
	// 	let phrygian = [1, 2, 2, 2, 1, 2, 2, 2];
	// 	assert_eq!(("GAbHbCDEbF".to_string()), modes('G', phrygian))
	// }
    // #[test]
    // fn phrygian_a() {
	// 	let phrygian = [1, 2, 2, 2, 1, 2, 2, 2];
	// 	assert_eq!(("AHbCDEFG".to_string()), modes('A', phrygian))
	// }
    // #[test]
    // fn phrygian_h() {
	// 	let phrygian = [1, 2, 2, 2, 1, 2, 2, 2];
	// 	assert_eq!(("HCDEF#GA".to_string()), modes('A', phrygian))
	// }
}
