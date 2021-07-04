// Brush up on lifetimes for word-matrix struct - https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html?highlight=lifetimes#lifetime-elision

/// Creates a sparse matrix for word indexing
///
/// # Arguments
///
/// * `word1` - A string slice holding the first word
/// * `word2` - A string slice hodling the second word
pub fn create_matrix(word1: &str, word2: &str) -> Vec<Vec<i32>> {
  let mut matrix = Vec::new();
  // Populate first row
  let first_row: Vec<i32> = (0..=(word1.len()) as i32).collect();
  matrix.push(first_row);
  // Populate the rest of the rows
  for idx in 1..=word2.len() {
    matrix.push(zero_padded_array(&(word1.len() + 1), &idx));
  }
  matrix
}


/// Creates a right-padded array of 0s with a defined first value
///
/// # Arguments
///
/// * `size` - usize denoting size of the array
/// * `first_val` - Value to insert at the first index
fn zero_padded_array(size: &usize, first_val: &usize) -> Vec<i32> {
  let mut zero_vec: Vec<i32> = vec![0; *size];
  zero_vec[0] = *first_val as i32;
  zero_vec
}


#[cfg(test)]
mod tests {
  use std::usize;

use super::*;

  #[test]
  fn test_array_len() {
    // given
    let size = 5 as usize;
    let first_val = 0 as usize;
    // when
    let res = zero_padded_array(&size, &first_val);
    // then
    assert_eq!(res.len(), size);
  }

  #[test]
  fn test_first_value_set() {
    // given
    let size = 5 as usize;
    let first_val = 4 as usize;
    // when
    let res = zero_padded_array(&size, &first_val);
    // then
    assert_eq!(res[0], first_val as i32);
  }
}