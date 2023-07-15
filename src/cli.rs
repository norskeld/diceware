use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about = "Generates strong Diceware passphrases.", long_about = None)]
pub struct DicewareCli {
  /// How much words to generate.
  #[clap(short, long, default_value_t = 6, display_order = 1)]
  pub length: usize,

  /// Path to a custom wordlist.
  #[clap(short, long, display_order = 2)]
  pub wordlist: Option<String>,

  /// Show entropy of the passphrase.
  #[clap(short, long, display_order = 3)]
  pub entropy: bool,

  /// Delimiter to use for joining words.
  #[clap(short, long, display_order = 4)]
  pub delimiter: Option<String>,
}
