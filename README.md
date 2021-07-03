# Eddist 

Library of edit distance tools, implemented in Rust.

Edit distance tools are used to quantify the similarity between 2 strings, primarily by way of counting the number of insertions, removals or replacements (or transpositions), required to make 2 strings identical.

## Aim

To implement the most commonly used edit distance algorithms in Rust, including the likes of Hamming, Levenshtein, Damerau-Levenshtein distance metrics (and more).

## Installation

For Unix/MacOS systems, clone the repository and then run the `install.sh` script

```bash
git clone https://github.com/cvonsteg/eddist.git
cd eddist
./install.sh
```

## Usage

Currently implemented algorithms are `Hamming` and `Levenshtein` distances.  To use these tools simply run the cli after installing:

```bash
eddist hamming <first-word> <second-word>
eddist levenshtein <first-word> <second-word>
```

For the `Levenshtein` tool, there is an optional flag to show the Levenshtein matrix

```bash
eddist levenshtein <first-word> <second-word> -m
# or
eddist levenshtein <first-word> <second-word> --matrix
```

## Notes

- Levenshtein algo [here](https://people.cs.pitt.edu/~kirk/cs1501/Pruhs/Spring2006/assignments/editdistance/Levenshtein%20Distance.htm)
- Similar implementation in rust to check out [here](https://matklad.github.io/2017/03/12/min-of-three.html)

