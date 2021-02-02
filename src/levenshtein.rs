use std::cmp::min;
use super::word_matrix::create_matrix;


/// Calculate the Levenshtein distance between 2 words
///
/// Levenshtein distance quantifies single-character edit insertions, deletions and substitions, required to change one word into the other
///
/// # Arguments
///
/// * `word1` - First word to compare
/// * `word2` - Second word to compare
///
/// # Examples
///
/// ```
/// let first = "cat";
/// let second = "hat";
/// let ld = levenshtein_distance(&first, &second);
///
/// assert_eq!(1, ld)
/// ```
pub fn levenshtein_distance(word1: &str, word2: &str) -> i32 {
  let max_col = word1.chars().count();
  let max_row = word2.chars().count();
  let mut matrix = create_matrix(word1, word2);
  for (col, letter_first) in word1.chars().enumerate() {
    for (row, letter_second) in word2.chars().enumerate() {
      let cost = calculate_cost(&letter_first, &letter_second);
      matrix[row+1][col+1] = get_lev_value(&matrix, col, row, cost);
    }
  }
  matrix[max_row][max_col] 
}


/// Return a cost of 0 or 1 depending on whether the letters are identical or not respectively
///
/// # Arguments
///
/// * `letter1` - Character representing the letter being analysed form word1
/// * `letter2` - Character representing the letter being analysed from word2
fn calculate_cost(letter1: &char, letter2: &char) -> i32 {
  let mut cost = 1;
  if letter1 == letter2 {
    cost = 0 
  }
  cost
}


/// Get Levenshtein value to populate cell in the matrix, based on surrounding cells and cost
///
/// The value of a given cell in the Levenshtein matrix is defined as:
///
/// minimum of:
/// - value in the cell directly above + 1
/// - value in the cell directly to the left + 1
/// - Value in the cell up and left (diagonally above) + cost
///
/// # Arguments
///
/// * `matrix` - A vector of i32 vectors, representing a 2D matrix
/// * `col` - Index for which column is being used (NOTE: offset by 1)
/// * `row` - Index for which row is b eing used (NOTE: offset by 1)
/// * `cost` - A i32 cost value to penalise the cell diagonally above the current cell
fn get_lev_value(matrix: &Vec<Vec<i32>>, col: usize, row: usize, cost: i32) -> i32 {
  let above = matrix[row][col + 1] + 1;
  let left = matrix[row + 1][col] + 1;
  let diag = matrix[row][col] + cost;
  min(min(above, left), diag)
}

