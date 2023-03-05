mod diatonic;
mod scales;
use crate::diatonic::scale_finder;

fn main() {
	let note = 'C';
	let modes = ["major", "ionian", "dorian", "phrygian", "lydian", "mixolydian", "aeolian", "minor", "locrian"];
	println!("Step number:\t1\t2\t3\t4\t5\t6\t7\t1");
	for i in modes {
		if i == "major" || i == "minor" {
			println!("{i}: \t\t{}", scale_finder(note, ' ', i));
		} else {
			println!("{i}: \t{}", scale_finder(note, ' ', i));
		}
	}

	println!("C# dorian: \t{}", scale_finder('C', '#', "dorian"));

	println!("Test wrong root note swap");
	
	println!("F  dorian: \t{}", scale_finder('F', ' ', "dorian"));
	println!("E# dorian: \t{}", scale_finder('E', '#', "dorian"));
	println!("E  dorian: \t{}", scale_finder('E', ' ', "dorian"));
	println!("Fb dorian: \t{}", scale_finder('F', 'b', "dorian"));

	println!("C  dorian: \t{}", scale_finder('C', ' ', "dorian"));
	println!("H# dorian: \t{}", scale_finder('H', '#', "dorian"));
	println!("H  dorian: \t{}", scale_finder('H', ' ', "dorian"));
	println!("Cb dorian: \t{}", scale_finder('C', 'b', "dorian"));
}
