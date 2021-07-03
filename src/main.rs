use structopt::StructOpt;
use eddist::hamming::hamming_distance;
use eddist::levenshtein::levenshtein_distance;


#[derive(StructOpt, Debug)]
#[structopt(name="eddist")]
enum Eddist{
  #[structopt(name="levenshtein")]
  Levenshtein {
    first_word: String,
    second_word: String,
    #[structopt(short="m", long="matrix", help="Show the final levenshtein distance matrix")]
    show_matrix: bool
  },
  #[structopt(name="hamming")]
  Hamming {
    first_word: String,
    second_word: String
  }
}



fn main() {
  let distance = match Eddist::from_args() {
    Eddist::Levenshtein{ first_word, second_word, show_matrix} => levenshtein_distance(first_word.as_str(), second_word.as_str(), show_matrix),
    Eddist::Hamming{ first_word, second_word } => hamming_distance(first_word.as_str(), second_word.as_str())
  };
  println!("{:?}", distance);
}
