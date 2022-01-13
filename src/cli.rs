use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about = "Generates strong Diceware passphrases.", long_about = None)]
pub struct DicewareCli {
  /// How much words you want to generate.
  #[clap(short, long, default_value_t = 6, display_order = 1)]
  pub length: usize,

  /// Path to a custom wordlist (the wordlist must contain 1111-7776 lines).
  #[clap(short, long, display_order = 2)]
  pub wordlist: Option<String>,

  /// Whether to show entropy of the passphrase.
  #[clap(short, long, display_order = 3)]
  pub entropy: bool,
}
