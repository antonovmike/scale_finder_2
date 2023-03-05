pub mod scales;
pub mod diatonic;
#[allow(unused)]
use crate::diatonic::*;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn wrong_char() {
		assert_eq!("".to_string(), scale_finder('Q', ' ', "minor"))
	}

	#[test]
	fn minor_no_acc() {
		assert_eq!("C\tD\tEb\tF\tG\tAb\tHb\tC\t".to_string(),    scale_finder('c', ' ', "minor"));
	 	assert_eq!("D\tE\tF\tG\tA\tHb\tC\tD\t".to_string(),      scale_finder('d', ' ', "minor"));
	 	assert_eq!("E\tF#\tG\tA\tH\tC\tD\tE\t".to_string(),      scale_finder('E', ' ', "minor"));
	 	assert_eq!("F\tG\tAb\tHb\tC\tDb\tEb\tF\t".to_string(),   scale_finder('F', ' ', "minor"));
	 	assert_eq!("G\tA\tHb\tC\tD\tEb\tF\tG\t".to_string(),     scale_finder('G', ' ', "minor"));
		assert_eq!("A\tH\tC\tD\tE\tF\tG\tA\t".to_string(),       scale_finder('A', ' ', "minor"));
		assert_eq!("H\tC#\tD\tE\tF#\tG\tA\tH\t".to_string(),     scale_finder('H', ' ', "minor"));
		assert_eq!("C\tD\tE\tF\tG\tA\tH\tC\t".to_string(),       scale_finder('C', ' ', "major"))
	}

	#[test]
	fn flat() {
		assert_eq!("Db\tEb\tF\tGb\tAb\tHb\tC\tDb\t".to_string(), scale_finder('D', 'b', "major"))
	}
	#[test]
	fn sharp() {
		assert_eq!("C#\tD#\tE\tF#\tG#\tA#\tH\tC#\t".to_string(),   scale_finder('C', '#', "dorian"))
	}
}