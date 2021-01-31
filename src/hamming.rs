// Hamming distance is an edit distance metric which can be used to compare 2 EQUAL LENGTH strings.
// The Hamming distance metric measures the number of substitutions which would need to be made at
// each index of the strings to make them equal.


/// Calculates the hamming distance between 2 words
///
/// # Arguments
///
/// * `str_1` - String slice representing the first word to compare
/// * `str_2` - String slice representing the second word to compare
pub fn hamming_distance(str_1: &str, str_2: &str) -> i32 {
    // Hamming distance only works on equal-lengthed strings
    assert!(str_1.chars().count() == str_2.chars().count(), "Cannot calculate hamming distance on strings of different lengths");
    let mut hamming_dist = 0;
    for idx in 0..str_1.chars().count() {
      if str_1.chars().nth(idx) != str_2.chars().nth(idx) {
        hamming_dist += 1;
      }
    }
    hamming_dist
}
