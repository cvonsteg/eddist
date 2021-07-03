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
    if str_1.len() != str_2.len() {
      panic!("Cannot calculate hamming distance on strings of different lengths");
    }
    let mut hamming_dist = 0;
    for (letter1, letter2) in str_1.chars().zip(str_2.chars()) {
      if letter1 != letter2 {
        hamming_dist += 1;
      }
    }
    hamming_dist
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_single_replacement() {
    // given
    let word1 = "cat";
    let word2 = "hat";
    // when
    let distance = hamming_distance(word1, word2);
    // then
    assert_eq!(distance, 1)
  }

  #[test]
  fn test_single_swap() {
    // given
    let str_1 = "cat";
    let str_2 = "cta";
    // when
    let distance = hamming_distance(str_1, str_2);
    // then
    assert_eq!(distance, 2);
  }

  #[test]
  fn test_flawn() {
    // Nice sanity check example from Wikipedia - Hamming should be 4
    // given
    let str_1 = "flaw";
    let str_2 = "lawn";
    // when
    let distance = hamming_distance(str_1, str_2);
    // then
    assert_eq!(distance, 4);
  }

  #[test]
  #[should_panic]
  fn test_hamming_panics_if_words_unequal() {
    // given
    let word1 = "abba";
    let word2 = "foo";
    // then
    hamming_distance(word1, word2);
  }

}