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
}
