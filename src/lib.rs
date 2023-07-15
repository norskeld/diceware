use rand::Rng;

static EFF_WORDLIST: &str = include_str!("../data/eff_long_wordlist.txt");

/// Represents a pair of an index, and a word associated with that index.
pub(crate) type Pair = (usize, String);

/// Formatting presets.
#[derive(Clone, Debug, Default)]
pub enum Preset {
  /// Format using `PascalCase` style.
  PascalCase,
  /// Format using `kebab-case` style.
  KebabCase,
  /// Format using `snake_case` style.
  SnakeCase,
  /// Format using provided parameters.
  Arbitrary {
    /// Whether to capitalize a word or not.
    capitalize: bool,
    /// Delimiter to use when joining words.
    delimiter: Option<String>,
  },
  /// Format using default parameters.
  #[default]
  Default,
}

impl Preset {
  /// Creates a [Preset] from given string (excepting [Preset::Arbitrary]).
  pub fn from(preset_name: &str) -> Self {
    match preset_name {
      | "pascal" => Self::PascalCase,
      | "kebab" => Self::KebabCase,
      | "snake" => Self::SnakeCase,
      | _ => Self::Default,
    }
  }
}

/// Non-consuming builder that allows to easily configure things up and generate a [Passphrase].
///
/// # Examples
///
/// You can use method chaining:
///
/// ```ignore
/// let mut builder = Passphraser::new(6)
///   .wordlist(&wordlist)
///   .length(10);
///   .preset(Preset::PascalCase)
/// ```
///
/// Or call them separately:
///
/// ```ignore
/// let mut builder = Passphraser::new(6);
///
/// builder.wordlist(&wordlist);
/// builder.length(10);
/// builder.preset(Preset::PascalCase);
///
/// let passphrase = builder.generate();
/// ```
#[derive(Debug)]
pub struct Passphraser {
  /// Number of words to generate.
  length: usize,
  /// Wordlist to pick words from.
  wordlist: Vec<String>,
  /// Formatting preset to use. Default is [Preset::Default].
  preset: Preset,
}

impl Passphraser {
  /// Create builder with specified number of words to generate.
  pub fn new(length: usize) -> Self {
    Self {
      length,
      wordlist: builtin_wordlist(),
      preset: Preset::Default,
    }
  }

  /// Set the number of words to generate.
  pub fn length(&mut self, length: usize) -> &mut Self {
    self.length = length;
    self
  }

  /// Set the wordlist to pick words from.
  pub fn wordlist<'a>(&'a mut self, list: &'a [String]) -> &'a mut Self {
    self.wordlist = list.to_vec();
    self
  }

  /// Set the formatting preset.
  pub fn preset(&mut self, preset: Preset) -> &mut Self {
    self.preset = preset;
    self
  }

  /// Roll dice, generate passphrase words, calculate entropy and return a [Passphrase].
  pub fn generate(&self) -> Passphrase {
    let rolls = roll_dice(self.length, 5, 1, 6);
    let words = passphrase(&self.wordlist, rolls);

    let entropy = Entropy::new(self.wordlist.len(), self.length);

    Passphrase {
      words,
      preset: self.preset.clone(),
      entropy,
    }
  }
}

/// Contains information about entropy.
#[derive(Debug)]
pub struct Entropy {
  /// How much unique words (possibilites) contains the wordlist.
  pub possibilities: usize,
  /// Calculated entropy of the passphrase.
  pub entropy: f32,
}

impl Entropy {
  pub fn new(possibilities: usize, phrase_length: usize) -> Self {
    Entropy {
      possibilities,
      entropy: calc_entropy(possibilities, phrase_length),
    }
  }
}

/// Contains generated passphrase words, formatting preset and calculated entropy.
#[derive(Debug)]
pub struct Passphrase {
  preset: Preset,
  entropy: Entropy,
  words: Vec<String>,
}

impl Passphrase {
  const DELIM_DEFAULT: &'static str = " ";
  const DELIM_KEBABCASE: &'static str = "-";
  const DELIM_PASCALCASE: &'static str = "";
  const DELIM_SNAKECASE: &'static str = "_";

  /// Returns generated passphrase words.
  pub fn words(&self) -> &Vec<String> {
    &self.words
  }

  /// Returns calculated passphrase [Entropy].
  pub fn entropy(&self) -> &Entropy {
    &self.entropy
  }

  /// Formats passphrase using the passphrase's preset.
  pub fn format(&self) -> String {
    self.format_with(&self.preset)
  }

  /// Formats passphrase using the given preset.
  pub fn format_with(&self, preset: &Preset) -> String {
    match &preset {
      | Preset::PascalCase => self.format_using(Self::DELIM_PASCALCASE, true),
      | Preset::KebabCase => self.format_using(Self::DELIM_KEBABCASE, false),
      | Preset::SnakeCase => self.format_using(Self::DELIM_SNAKECASE, false),
      | Preset::Arbitrary {
        capitalize,
        delimiter,
      } => {
        let default = Self::DELIM_DEFAULT.to_string();
        let delimiter = delimiter.clone().unwrap_or(default);

        self.format_using(&delimiter, *capitalize)
      },
      | Preset::Default => self.format_using(Self::DELIM_DEFAULT, false),
    }
  }

  /// Joins words using specified delimiter and optionally capitalizes them.
  fn format_using(&self, delimiter: &str, capitalize: bool) -> String {
    let words = if capitalize {
      self
        .words
        .iter()
        .map(|word| to_capitalized(word))
        .collect::<Vec<_>>()
    } else {
      self.words.clone()
    };

    words.join(delimiter)
  }
}

/// Rolls a dice, producing a vector of numbers for each run.
pub fn roll_dice(runs: usize, rolls: usize, start: usize, end: usize) -> Vec<Vec<usize>> {
  let mut rng = rand::thread_rng();

  (1..=runs)
    .map(|_| (1..=rolls).map(|_| rng.gen_range(start..end)).collect())
    .collect()
}

/// Given a wordlist and dice rolls, generates a Diceware passphrase as a [Vec] of words.
pub fn passphrase(lines: &[String], dice_rolls: Vec<Vec<usize>>) -> Vec<String> {
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

/// Reads a built-in EFF long wordlist and returns a vector of lines.
pub fn builtin_wordlist() -> Vec<String> {
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

/// Capitalizes the first char of given string.
pub(crate) fn to_capitalized(s: &str) -> String {
  let mut chars = s.chars();

  match chars.next() {
    | None => String::new(),
    | Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
  }
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
