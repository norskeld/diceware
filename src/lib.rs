use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

use rand::Rng;

static EFF_WORDLIST: &str = include_str!("../data/eff_long_wordlist.txt");

/// Represents a pair of an index, and a word associated with that index.
pub(crate) type Pair = (usize, String);

/// Given a wordlist and rolls, generates a Diceware passphraseas as a [Vec] of words.
pub fn passphrase(lines: Vec<String>, dice_rolls: Vec<Vec<usize>>) -> Vec<String> {
  let words = dice_rolls.iter().fold(Vec::new(), |acc, roll| {
    let rolled_index = to_index(roll.to_vec());

    let rolled_word = lines.iter().find_map(|line| {
      let components = to_components(line);
      let pair = to_pair(components);

      match pair {
        | Some((index, word)) if rolled_index == index => Some(word),
        | _ => None,
      }
    });

    if let Some(word) = rolled_word {
      [acc, vec![word]].concat()
    } else {
      acc
    }
  });

  words
}

/// Rolls a dice, producing a vector of numbers for each run.
pub fn roll_dice(runs: usize, rolls: usize, start: usize, end: usize) -> Vec<Vec<usize>> {
  let mut rng = rand::thread_rng();

  (1..=runs)
    .map(|_| (1..=rolls).map(|_| rng.gen_range(start..end)).collect())
    .collect()
}

/// Reads a wordlist with `<index> <word>` pairs and returns a [Result] with vector of lines.
pub fn read_wordlist<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  reader.lines().collect()
}

/// Reads a built-in EFF long wordlist and returns a vector of lines.
pub fn read_wordlist_internal() -> Vec<String> {
  EFF_WORDLIST.lines().map(str::to_string).collect()
}

/// Given a length (the number of possibilities, e.g. for the EFF long list it is 7776
/// possibilities) of a wordlist and phrase length in words, calculates entropy of the phrase.
pub fn calc_entropy(possibilities: usize, phrase_length: usize) -> f32 {
  (possibilities as f32).log2() * (phrase_length as f32)
}

/// Splits a given line into a vector of components.
pub(crate) fn to_components(line: &str) -> Vec<&str> {
  line.split_ascii_whitespace().collect()
}

/// Unpacks a given vector of line components in the form of `[index, word]` to a [Pair] struct.
pub(crate) fn to_pair(components: Vec<&str>) -> Option<Pair> {
  let mut components = components.iter();

  if let (Some(index), Some(word)) = (components.next(), components.next()) {
    match index.parse::<usize>() {
      | Ok(index) => Some((index, word.to_string())),
      | Err(_) => None,
    }
  } else {
    None
  }
}

/// Reduces a vector of rolled numbers to a single number which then will be used as an index in a
/// Diceware wordlist.
pub(crate) fn to_index(ns: Vec<usize>) -> usize {
  ns.iter().fold(0, |acc, n| acc * 10 + n)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn test_roll_dice() {
    roll_dice(6, 5, 0, 0);
    roll_dice(6, 0, 0, 0);
  }

  #[test]
  fn test_to_index() {
    assert_eq!(to_index(vec![1, 1, 1]), 111);
    assert_eq!(to_index(vec![5, 2, 3, 1, 6]), 52316);
  }
}
