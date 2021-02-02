use  super::word_matrix::create_matrix;


// for i in n_cols
// for j in n_rows
// update mechanism


pub fn levenshtein_distance(word1: &str, word2: &str) -> i32 {
  let max_col = word1.chars().count();
  let max_row = word2.chars().count();
  let mut matrix = create_matrix(word1, word2);
  for (col, letter_first) in word1.chars().enumerate() {
    for (row, letter_second) in word2.chars().enumerate() {
      let cost = calculate_cost(&letter_first, &letter_second);
      matrix[row+1][col+1] = get_value(&matrix, col, row, cost);
    }
  }
  matrix[max_row][max_col] 
}


fn calculate_cost(letter1: &char, letter2: &char) -> i32 {
  let mut cost = 1;
  if letter1 == letter2 {
    cost = 0 
  }
  cost
}


fn get_value(matrix: &Vec<Vec<i32>>, col: usize, row: usize, cost: i32) -> i32 {
  let above = matrix[row][col + 1] + 1;
  let left = matrix[row + 1][col] + 1;
  let diag = matrix[row][col] + cost;
  min2(min2(above, left), diag)
}


fn min2(x: i32, y: i32) -> i32 {
  if x < y { x } else { y }
}
