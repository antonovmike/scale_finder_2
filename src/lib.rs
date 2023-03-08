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
        assert_eq!("FGAbHbCDEbF".to_string(),    scale_builder('E', '#', "dorian"));
        assert_eq!("EFGAHCDE".to_string(),       scale_builder('F', 'b', "phrygian"));
        assert_eq!("CDEFGAHbC".to_string(),      scale_builder('H', '#', "mixolydian"));
        assert_eq!("HC#D#E#F#G#A#H".to_string(), scale_builder('C', 'b', "lydian"));
    }

// CLEAN ROOT NOTES

    #[test]
    fn c() {
        assert_eq!("CDEFGAHC".to_string(),      scale_builder('C', ' ', "Major"));
        assert_eq!("CDEbFGAHbC".to_string(),    scale_builder('C', ' ', "DORIAN"));
        assert_eq!("CDbEbFGAbHbC".to_string(),  scale_builder('C', ' ', "Phrygian"));
        assert_eq!("CDEF#GAHC".to_string(),     scale_builder('C', ' ', "LYDIAN")); 
        assert_eq!("CDEFGAHbC".to_string(),     scale_builder('C', ' ', "Mixolydian"));
        assert_eq!("CDEbFGAbHbC".to_string(),   scale_builder('c', ' ', "Minor"));
        assert_eq!("CDbEbFGbAbHbC".to_string(), scale_builder('C', ' ', "Locrian"));
    }

    #[test]
    fn d() {
        assert_eq!("DEF#GAHC#D".to_string(),    scale_builder('D', ' ', "Major"));
        assert_eq!("DEFGAHCD".to_string(),      scale_builder('D', ' ', "Dorian"));
        assert_eq!("DEbFGAHbCD".to_string(),    scale_builder('D', ' ', "Phrygian"));
        assert_eq!("DEF#G#AHC#D".to_string(),   scale_builder('D', ' ', "LYDIAN")); 
        assert_eq!("DEF#GAHCD".to_string(),     scale_builder('D', ' ', "Mixolydian"));
        assert_eq!("DEFGAHbCD".to_string(),     scale_builder('D', ' ', "Minor"));
        assert_eq!("DEbFGAbHbCD".to_string(),   scale_builder('D', ' ', "Locrian"));
    }

    #[test]
    fn e() {
        assert_eq!("EF#G#AHC#D#E".to_string(),    scale_builder('E', ' ', "Major"));
        assert_eq!("EF#GAHC#DE".to_string(),      scale_builder('E', ' ', "Dorian"));
        assert_eq!("EFGAHCDE".to_string(),        scale_builder('E', ' ', "Phrygian"));
        assert_eq!("EF#G#A#HC#D#E".to_string(),   scale_builder('E', ' ', "LYDIAN")); 
        assert_eq!("EF#G#AHC#DE".to_string(),     scale_builder('E', ' ', "Mixolydian"));
        assert_eq!("EF#GAHCDE".to_string(),       scale_builder('E', ' ', "Minor"));
        assert_eq!("EFGAHbCDE".to_string(),       scale_builder('E', ' ', "Locrian"));
    }

    #[test]
    fn f() {
        assert_eq!("FGAHbCDEF".to_string(),       scale_builder('F', ' ', "Major"));
        assert_eq!("FGAbHbCDEbF".to_string(),     scale_builder('F', ' ', "Dorian"));
        assert_eq!("FGbAbHbCDbEbF".to_string(),   scale_builder('F', ' ', "Phrygian"));
        assert_eq!("FGAHCDEF".to_string(),        scale_builder('F', ' ', "LYDIAN")); 
        assert_eq!("FGAHbCDEbF".to_string(),      scale_builder('F', ' ', "Mixolydian"));
        assert_eq!("FGAbHbCDbEbF".to_string(),    scale_builder('F', ' ', "Minor"));
        assert_eq!("FGbAbHbCbDbEbF".to_string(),  scale_builder('F', ' ', "Locrian"));
    }

    #[test]
    fn g() {
        assert_eq!("GAHCDEF#G".to_string(),       scale_builder('G', ' ', "Major"));
        assert_eq!("GAHbCDEFG".to_string(),       scale_builder('G', ' ', "Dorian"));
        assert_eq!("GAbHbCDEbFG".to_string(),     scale_builder('G', ' ', "Phrygian"));
        assert_eq!("GAHC#DEF#G".to_string(),      scale_builder('G', ' ', "LYDIAN")); 
        assert_eq!("GAHCDEFG".to_string(),        scale_builder('G', ' ', "Mixolydian"));
        assert_eq!("GAHbCDEbFG".to_string(),      scale_builder('G', ' ', "Minor"));
        assert_eq!("GAbHbCDbEbFG".to_string(),    scale_builder('G', ' ', "Locrian"));
    }

    #[test]
    fn a() {
        assert_eq!("AHC#DEF#G#A".to_string(),     scale_builder('A', ' ', "Major"));
        assert_eq!("AHCDEF#GA".to_string(),       scale_builder('A', ' ', "Dorian"));
        assert_eq!("AHbCDEFGA".to_string(),       scale_builder('A', ' ', "Phrygian"));
        assert_eq!("AHC#D#EF#G#A".to_string(),    scale_builder('A', ' ', "LYDIAN")); 
        assert_eq!("AHC#DEF#GA".to_string(),      scale_builder('A', ' ', "Mixolydian"));
        assert_eq!("AHCDEFGA".to_string(),        scale_builder('A', ' ', "Minor"));
        assert_eq!("AHbCDEbFGA".to_string(),      scale_builder('A', ' ', "Locrian"));
    }

    #[test]
    fn h() {
        assert_eq!("HC#D#EF#G#A#H".to_string(),    scale_builder('H', ' ', "Major"));
        assert_eq!("HC#DEF#G#AH".to_string(),      scale_builder('H', ' ', "Dorian"));
        assert_eq!("HCDEF#GAH".to_string(),        scale_builder('H', ' ', "Phrygian"));
        assert_eq!("HC#D#E#F#G#A#H".to_string(),   scale_builder('H', ' ', "LYDIAN")); 
        assert_eq!("HC#D#EF#G#AH".to_string(),     scale_builder('H', ' ', "Mixolydian"));
        assert_eq!("HC#DEF#GAH".to_string(),       scale_builder('H', ' ', "Minor"));
        assert_eq!("HCDEFGAH".to_string(),         scale_builder('H', ' ', "Locrian"));
    }

// FLAT AND SHARP ROOT NOTES

    #[test]
    fn d_flat_c_sharp() {
        assert_eq!("DbEbFGbAbHbCDb".to_string(),  scale_builder('D', 'b', "Major"));
        assert_eq!("C#D#EF#G#A#HC#".to_string(),  scale_builder('C', '#', "Dorian"));
        assert_eq!("C#DEF#G#AHC#".to_string(),    scale_builder('C', '#', "Phrygian"));
        assert_eq!("DbEbFGAbHbCDb".to_string(),   scale_builder('D', 'b', "LYDIAN")); 
        assert_eq!("C#D#E#F#G#A#HC#".to_string(), scale_builder('C', '#', "Mixolydian"));
        assert_eq!("C#D#EF#G#AHC#".to_string(),   scale_builder('C', '#', "Minor"));
        assert_eq!("C#DEF#GAHC#".to_string(),     scale_builder('C', '#', "Locrian"));
    }

    #[test]
    fn e_flat_d_sharp() {
        assert_eq!("EbFGAbHbCDEb".to_string(),    scale_builder('E', 'b', "Major"));
        assert_eq!("EbFGbAbHbCDbEb".to_string(),  scale_builder('E', 'b', "Dorian"));
        assert_eq!("D#EF#G#A#HC#D#".to_string(),  scale_builder('D', '#', "Phrygian"));
        assert_eq!("EbFGAHbCDEb".to_string(),     scale_builder('E', 'b', "LYDIAN")); 
        assert_eq!("EbFGAbHbCDbEb".to_string(),   scale_builder('E', 'b', "Mixolydian"));
        assert_eq!("EbFGbAbHbCbDbEb".to_string(), scale_builder('E', 'b', "Minor"));
        assert_eq!("D#EF#G#AHC#D#".to_string(),   scale_builder('D', '#', "Locrian"));
    }

    #[test]
    fn g_flat_f_sharp () {
        assert_eq!("GbAbHbCbDbEbFGb".to_string(), scale_builder('G', 'b', "Major"));
        assert_eq!("F#G#AHC#D#EF#".to_string(),   scale_builder('F', '#', "Dorian"));
        assert_eq!("F#GAHC#DEF#".to_string(),     scale_builder('F', '#', "Phrygian"));
        assert_eq!("GbAbHbCDbEbFGb".to_string(),  scale_builder('G', 'b', "LYDIAN")); 
        assert_eq!("GbAbHbCbDbEbFbGb".to_string(),scale_builder('G', 'b', "Mixolydian"));
        assert_eq!("F#G#AHC#DEF#".to_string(),    scale_builder('F', '#', "Minor"));
        assert_eq!("F#GAHCDEF#".to_string(),      scale_builder('F', '#', "Locrian"));
    }

    #[test]
    fn a_flat_g_sharp() {
        assert_eq!("AbHbCDbEbFGAb".to_string(),   scale_builder('A', 'b', "Major"));
        assert_eq!("AbHbCbDbEbFGbAb".to_string(), scale_builder('A', 'b', "Dorian"));
        assert_eq!("G#AHC#D#EF#G#".to_string(),   scale_builder('G', '#', "Phrygian"));
        assert_eq!("AbHbCDEbFGAb".to_string(),    scale_builder('A', 'b', "LYDIAN")); 
        assert_eq!("AbHbCDbEbFGbAb".to_string(),  scale_builder('A', 'b', "Mixolydian"));
        assert_eq!("G#A#HC#D#EF#G#".to_string(),  scale_builder('G', '#', "Minor"));
        assert_eq!("G#AHC#DEF#G#".to_string(),    scale_builder('G', '#', "Locrian"));
    }

    #[test]
    fn h_flat_a_sharp() {
        assert_eq!("HbCDEbFGAHb".to_string(),     scale_builder('H', 'b', "Major"));
        assert_eq!("HbCDbEbFGAbHb".to_string(),   scale_builder('H', 'b', "Dorian"));
        assert_eq!("A#HC#D#E#F#G#A#".to_string(), scale_builder('A', '#', "Phrygian"));
        assert_eq!("HbCDEFGAHb".to_string(),      scale_builder('H', 'b', "LYDIAN")); 
        assert_eq!("HbCDEbFGAbHb".to_string(),    scale_builder('H', 'b', "Mixolydian"));
        assert_eq!("HbCDbEbFGbAbHb".to_string(),  scale_builder('H', 'b', "Minor"));
        assert_eq!("A#HC#D#EF#G#A#".to_string(),  scale_builder('A', '#', "Locrian"));
    }
}
