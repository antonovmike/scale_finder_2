mod diatonic;
mod scales;
use crate::diatonic::scale_finder;

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
