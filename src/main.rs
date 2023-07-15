mod cli;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::process;

use clap::Parser;
use cli::Cli;
use colored::*;
use diceware::{Passphraser, Preset};

fn main() {
  let cli = Cli::parse();
  let mut builder = Passphraser::new(cli.length);

  // Trying to load custom wordlist if set.
  if let Some(path) = cli.wordlist {
    if let Ok(wordlist) = read_wordlist(path.clone()) {
      builder.wordlist(&wordlist);
    } else {
      println!("Couldn't read the wordlist. Make sure the file exists.");
      process::exit(1);
    }
  }

  // Setting a preset for formatting.
  let mut preset = if let Some(preset) = cli.preset {
    Preset::from(&preset)
  } else {
    Preset::Default
  };

  if cli.capitalize {
    preset = Preset::Arbitrary {
      capitalize: cli.capitalize,
      delimiter: None,
    }
  }

  if cli.delimiter.is_some() {
    preset = Preset::Arbitrary {
      capitalize: cli.capitalize,
      delimiter: cli.delimiter,
    }
  }

  // Generate the passphrase.
  let mut passphrase = builder.preset(preset).generate();

  if passphrase.words().is_empty() {
    println!("Couldn't generate a passphrase with given parameters.");
    process::exit(1);
  } else {
    println!("{}", &passphrase.format().green().bold());

    if cli.entropy {
      let entropy = passphrase.entropy();

      let possibilities = format!("{}", entropy.possibilities).blue();
      let entropy = format!("{:.2} bits", entropy.entropy).blue();

      println!("\nPossibilities: {possibilities}");
      println!("Entropy: {entropy}");
      println!("\nMore about entropy at https://theworld.com/~reinhold/dicewarefaq.html#entropy");
    }
  }
}

/// Reads a wordlist with `<index> <word>` pairs and returns a [Result] with vector of lines.
fn read_wordlist<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  reader.lines().collect()
}
