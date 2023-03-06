mod diatonic;
mod scales;
use crate::diatonic::scale_finder;

fn main() {
    let note = 'C';
    let modes = [
        "major",
        "ionian",
        "dorian",
        "phrygian",
        "lydian",
        "mixolyd",
        "aeolian",
        "minor",
        "locrian",
    ];
    // println!("Step number:\t1\t2\t3\t4\t5\t6\t7\t1");
    for i in modes {
		println!("{i}:\t{}", scale_finder(note, ' ', i));
    }

    println!("C# dorian:\t{}", scale_finder('C', '#', "dorian"));

    println!("Test wrong root note swap");

    println!("F  dorian: \t{}", scale_finder('F', ' ', "dorian"));
    println!("E# dorian: \t{}", scale_finder('E', '#', "dorian"));
    println!("E  phrygian: \t{}", scale_finder('E', ' ', "phrygian"));
    println!("Fb phrygian: \t{}", scale_finder('F', 'b', "phrygian"));

    println!("C  mixolydian: \t{}", scale_finder('C', ' ', "mixolydian"));
    println!("H# mixolydian: \t{}", scale_finder('H', '#', "mixolydian"));
    println!("H  lydian: \t{}", scale_finder('H', ' ', "lydian"));
    println!("Cb lydian: \t{}", scale_finder('C', 'b', "lydian"));
}
