use schema_parser::{rust::ToRust, spec::ToSpec, Database};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

use clap::{AppSettings, Clap};

enum OutputTypes {
  Spec,
  Rust,
}

impl FromStr for OutputTypes {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "rust" => Ok(OutputTypes::Rust),
      "spec" => Ok(OutputTypes::Spec),
      cant_parse => panic!("{} is not a valid output", cant_parse),
    }
  }
}

#[derive(Clap)]
#[clap(version = "0.1", author = "Justin Barclay <justincbarclay@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
  /// Sets a custom config file. Could have been an Option<T> with no default too
  #[clap(short, long, default_value = "db/schema.rb")]
  schema: String,

  #[clap(short, long, default_value = "spec")]
  output: OutputTypes,
}

fn main() {
  let opts = Opts::parse();
  // Open the path in read-only mode, returns `io::Result<File>`
  let path = Path::new(&opts.schema);
  let display = path.display();

  let mut file = match File::open(&path) {
    Err(why) => {
      eprintln!("couldn't open {}: {}", display, why);
      return;
    }
    Ok(file) => file,
  };

  let mut schema = String::new();
  if let Err(why) = file.read_to_string(&mut schema) {
    eprintln!("couldn't read {}: {}", display, why);
    return;
  }
  match opts.output{
    OutputTypes::Spec =>   println!("{}", Database::from(&schema).to_spec()),
    OutputTypes::Rust =>   println!("{}", Database::from(&schema).to_rust())
  }
}
