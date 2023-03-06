pub mod diatonic;
pub mod scales;
#[allow(unused)]
use crate::diatonic::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_char() {
        assert_eq!("".to_string(), scale_builder('Q', ' ', "minor"));
        assert_eq!("".to_string(), scale_builder('Q', 'b', "minor"));
        assert_eq!("".to_string(), scale_builder('C', 'Q', "minor"));
        assert_eq!("".to_string(), scale_builder('Q', 'Q', "minor"));
    }

    #[test]
    fn swap_wrong_root() {
        assert_eq!("FGAbHbCDEbF".to_string(), scale_builder('E', '#', "dorian"));
        assert_eq!("EFGAHCDE".to_string(), scale_builder('F', 'b', "phrygian"));
        assert_eq!(
            "CDEFGAHbC".to_string(),
            scale_builder('H', '#', "mixolydian")
        );
        assert_eq!(
            "HC#D#E#F#G#A#H".to_string(),
            scale_builder('C', 'b', "lydian")
        );
    }

    #[test]
    fn minor_no_acc() {
        assert_eq!("CDEbFGAbHbC".to_string(), scale_builder('c', ' ', "minor"));
        assert_eq!("DEFGAHbCD".to_string(), scale_builder('d', ' ', "minor"));
        assert_eq!("EF#GAHCDE".to_string(), scale_builder('E', ' ', "minor"));
        assert_eq!("FGAbHbCDbEbF".to_string(), scale_builder('F', ' ', "minor"));
        assert_eq!("GAHbCDEbFG".to_string(), scale_builder('G', ' ', "minor"));
        assert_eq!("AHCDEFGA".to_string(), scale_builder('A', ' ', "minor"));
        assert_eq!("HC#DEF#GAH".to_string(), scale_builder('H', ' ', "minor"));
        assert_eq!("CDEFGAHC".to_string(), scale_builder('C', ' ', "major"))
    }

    #[test]
    fn flat() {
        assert_eq!(
            "DbEbFGbAbHbCDb".to_string(),
            scale_builder('D', 'b', "major")
        )
    }
    #[test]
    fn sharp() {
        assert_eq!(
            "C#D#EF#G#A#HC#".to_string(),
            scale_builder('C', '#', "dorian")
        )
    }
}
