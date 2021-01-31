/// Creates a sparse matrix for word indexing
///
/// # Arguments
///
/// * `word1` - A string slice holding the first word
/// * `word2` - A string slice hodling the second word
pub fn create_matrix(word1: &str, word2: &str) -> Vec<Vec<i32>> {
    let mut matrix = Vec::new();
  // Populate first row
  let first_row = len_as_i32_vec(&word1);
  matrix.push(first_row);
  // Populate the rest of the rows
  let cols: Vec<usize> = (1..=word2.chars().count()).collect();
  for idx in cols {
    matrix.push(zero_padded_array(word1.chars().count(), &idx));
  }
  matrix
}

/// Creates a vector of i32 values from 0 to the length of a passed word
///
/// # Arguments
///
/// * `word` - A string slice 
fn len_as_i32_vec(word: &str) -> Vec<i32> {
  let length = word.chars().count();
  let mut row = Vec::new();
  for idx in 0..=length {
    row.push(idx as i32)
  }
  row
}

/// Creates a right-padded array of 0s with a defined first value
///
/// # Arguments
///
/// * `size` - integer denoting size of the array
/// * `first_val` - Value to insert at the first index
fn zero_padded_array(size: usize, first_val: &usize) -> Vec<i32> {
  let mut zero_vec: Vec<i32> = Vec::with_capacity(size as usize);
  zero_vec.push(*first_val as i32);
  for _ in 0..size {
    zero_vec.push(0);
  }
  return  zero_vec;
}
