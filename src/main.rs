use structopt::StructOpt;
use eddist::hamming::hamming_distance;
use eddist::word_matrix::create_matrix;
use eddist::levenshtein::levenshtein_distance;


#[derive(StructOpt)]
struct Cli {
  first_word: String,
  second_word: String
}


fn main() {
  let args = Cli::from_args();
  let first = args.first_word;
  let second = args.second_word;
  // let h_dist = hamming_distance(&first, &second);
  let levenshtein_matrix = create_matrix(&first, &second); 

  // println!("Hamming Distance between '{}' and '{}': {}", first, second, h_dist);
  println!("Edit distance matrix exampe:");
  for row in levenshtein_matrix.iter() {
      println!("{:?}", row);
  };
  println!("Matrix value (1,0): {}", levenshtein_matrix[1][0]);
  let ld = levenshtein_distance(&first, &second);
  println!("Lev Dist of {}, {}: {:?}", first, second, ld)


}
