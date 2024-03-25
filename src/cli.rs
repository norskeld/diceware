use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about = "Generates strong Diceware passphrases.", long_about = None)]
pub struct Cli {
  /// How much words to generate.
  #[arg(short, long, default_value_t = 6)]
  pub length: usize,

  /// Path to a custom wordlist.
  #[arg(short, long)]
  pub wordlist: Option<String>,

  /// Show entropy of the passphrase.
  #[arg(short, long)]
  pub entropy: bool,

  /// Capitalize words.
  #[arg(short, long)]
  pub capitalize: bool,

  /// Delimiter to use for joining words.
  #[arg(short, long)]
  pub delimiter: Option<String>,

  /// Formatting preset to use.
  #[arg(short, long, value_parser = ["pascal", "kebab", "snake"])]
  pub preset: Option<String>,
}
