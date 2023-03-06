pub mod diatonic;
pub mod scales;
#[allow(unused)]
use crate::diatonic::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_char() {
        assert_eq!("".to_string(), scale_finder('Q', ' ', "minor"));
        assert_eq!("".to_string(), scale_finder('Q', 'b', "minor"));
        assert_eq!("".to_string(), scale_finder('C', 'Q', "minor"));
        assert_eq!("".to_string(), scale_finder('Q', 'Q', "minor"));
    }

    #[test]
    fn swap_wrong_root() {
        assert_eq!("FGAbHbCDEbF".to_string(), scale_finder('E', '#', "dorian"));
        assert_eq!("EFGAHCDE".to_string(), scale_finder('F', 'b', "phrygian"));
        assert_eq!(
            "CDEFGAHbC".to_string(),
            scale_finder('H', '#', "mixolydian")
        );
        assert_eq!(
            "HC#D#E#F#G#A#H".to_string(),
            scale_finder('C', 'b', "lydian")
        );
    }

    #[test]
    fn minor_no_acc() {
        assert_eq!("CDEbFGAbHbC".to_string(), scale_finder('c', ' ', "minor"));
        assert_eq!("DEFGAHbCD".to_string(), scale_finder('d', ' ', "minor"));
        assert_eq!("EF#GAHCDE".to_string(), scale_finder('E', ' ', "minor"));
        assert_eq!("FGAbHbCDbEbF".to_string(), scale_finder('F', ' ', "minor"));
        assert_eq!("GAHbCDEbFG".to_string(), scale_finder('G', ' ', "minor"));
        assert_eq!("AHCDEFGA".to_string(), scale_finder('A', ' ', "minor"));
        assert_eq!("HC#DEF#GAH".to_string(), scale_finder('H', ' ', "minor"));
        assert_eq!("CDEFGAHC".to_string(), scale_finder('C', ' ', "major"))
    }

    #[test]
    fn flat() {
        assert_eq!(
            "DbEbFGbAbHbCDb".to_string(),
            scale_finder('D', 'b', "major")
        )
    }
    #[test]
    fn sharp() {
        assert_eq!(
            "C#D#EF#G#A#HC#".to_string(),
            scale_finder('C', '#', "dorian")
        )
    }
}
