# Eddist 

Library of edit distance tools, implemented in Rust.

Edit distance tools are used to quantify the similarity between 2 strings, primarily by way of counting the number of insertions, removals or replacements (or transpositions), required to make 2 strings identical.

## Aim

To implement the most commonly used edit distance algorithms in Rust, including the likes of Hamming, Levenshtein, Damerau-Levenshtein distance metrics (and more).


## TODO

- Convert word-matrix into a struct including hashmaps to efficiently look-up letters
- Levenshtein algo from [here](https://people.cs.pitt.edu/~kirk/cs1501/Pruhs/Spring2006/assignments/editdistance/Levenshtein%20Distance.htm)
