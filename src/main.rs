mod diatonic;
mod scales;
use crate::diatonic::*;

fn main() {
    let note = 'C';
    let modes = [
        "major",
        // "ionian",
        "dorian",
        "phrygian",
        "lydian",
        "mixolyd",
        // "aeolian",
        "minor",
        "locrian",
    ];
    println!("\tRoot note clean");
    for i in modes {
		    println!("{i}:   \t{}", scale_builder(note, ' ', i));
    }
    println!("\tRoot note sharp");
    for i in modes {
        println!("{i}:   \t{}", scale_builder(note, '#', i));
    }
    println!("\tRoot note flat");
    for i in modes {
        println!("{i}:   \t{}", scale_builder('d', 'b', i));
    }


    println!("C# major:   \t{}", scale_builder('C', '#', "major"));
    println!("C# lydian:   \t{}", scale_builder('C', '#', "lydian"));

    println!("\tTest wrong root note swap");

    println!("F  dorian: \t{}", scale_builder('F', ' ', "dorian"));
    println!("E# dorian: \t{}", scale_builder('E', '#', "dorian"));
    println!("E  phrygian: \t{}", scale_builder('E', ' ', "phrygian"));
    println!("Fb phrygian: \t{}", scale_builder('F', 'b', "phrygian"));

    println!("C  mixolydian: \t{}", scale_builder('C', ' ', "mixolydian"));
    println!("H# mixolydian: \t{}", scale_builder('H', '#', "mixolydian"));
    println!("H  lydian: \t{}", scale_builder('H', ' ', "lydian"));
    println!("Cb lydian: \t{}", scale_builder('C', 'b', "lydian"));

    println!("{}", octave());

    println!("{}", semitones("majoR"))
}
