use schema_parser::{rust::ToRust, spec::ToSpec, typescript::ToTypeScript, Database};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;

use clap::{AppSettings, Clap};

enum FormatTypes {
  Spec,
  Rust,
  TypeScript,
}

impl FromStr for FormatTypes {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "rust" => Ok(FormatTypes::Rust),
      "spec" => Ok(FormatTypes::Spec),
      "typescript" => Ok(FormatTypes::TypeScript),
      cant_parse => panic!("{} is not a valid output", cant_parse),
    }
  }
}

impl std::fmt::Display for FormatTypes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let output = match self {
      FormatTypes::Spec => "spec",
      FormatTypes::Rust => "rust",
      FormatTypes::TypeScript => "typescript",
    };
    write!(f, "{}", output)
  }
}
#[derive(Clap)]
#[clap(version = "0.1", author = "Justin Barclay <justincbarclay@gmail.com>")]
#[clap(description = "turbine is a toy cli for converting a schema file into type declarations")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
  /// Specifies the location of the Rails schema file
  #[clap(short, long, default_value = "db/schema.rb")]
  schema: String,

  /// Specifies type definition format to convert the schema file into.
  #[clap(short, long, default_value = "spec", possible_values = &["spec", "rust", "typescript"])]
  format: FormatTypes,

  /// The name of the file to output the results into
  #[clap(short, long)]
  output: Option<String>,
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
  let spec = match opts.format {
    FormatTypes::Spec => Database::from(&schema).to_spec(),
    FormatTypes::Rust => Database::from(&schema).to_rust(),
    FormatTypes::TypeScript => Database::from(&schema).to_typescript(),
  };

  match opts.output {
    Some(name) => {
      let output_path = Path::new(&name);
      let output_display = output_path.display();
      let mut file = match File::open(&path) {
        Err(why) => {
          eprintln!("couldn't open {}: {}", output_display, why);
          return;
        }
        Ok(file) => file,
      };

      match write!(file, "{}", spec) {
        Err(why) => {
          eprintln!("couldn't open {}: {}", output_display, why);
          return;
        }
        Ok(_) => (),
      }
    }
    None => println!("{}", spec),
  }
}
