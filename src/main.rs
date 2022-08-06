mod cli;

use std::process;

use clap::Parser;
use cli::DicewareCli;
use colored::*;

fn main() {
  let cli = DicewareCli::parse();

  let wordlist = {
    if let Some(path) = cli.wordlist {
      if let Ok(wordlist) = diceware::read_wordlist(path.clone()) {
        println!("Using the wordlist: {}\n", path);
        wordlist
      } else {
        println!("Couldn't read the wordlist. Make sure the file exists.");
        process::exit(1);
      }
    } else {
      diceware::read_wordlist_internal()
    }
  };

  let variants = wordlist.len();
  let rolls = diceware::roll_dice(cli.length, 5, 1, 6);
  let passphrase = diceware::passphrase(wordlist, rolls);

  if passphrase.is_empty() {
    println!("Couldn't generate a passphrase with given parameters.");
    process::exit(1);
  } else {
    println!("{}", passphrase.join(" ").green().bold());

    if cli.entropy {
      let possibilities = format!("{}", variants).blue();
      let entropy = format!("{:.2} bits", diceware::calc_entropy(variants, cli.length)).blue();

      println!("\nPossibilities: {possibilities}");
      println!("Entropy: {entropy}");
      println!("\nMore about entropy at https://theworld.com/~reinhold/dicewarefaq.html#entropy");
    }
  }
}
