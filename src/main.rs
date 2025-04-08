use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use turbine::{rust::ToRust, spec::ToSpec, typescript::ToTypeScript, go::ToGo, Database};

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum FormatTypes {
  Spec,
  Rust,
  TypeScript,
  Go,
}

#[derive(Parser, Debug)]
#[command(version = "0.2", author = "Justin Barclay <justincbarclay@gmail.com>")]
#[command(
  name = "turbine",
  about = "🌬️ a simple tool to bootstrap type declarations 🌬️"
)]
struct Opts {
  /// Specifies the location of the Rails schema file
  schema: String,

  /// Specifies type definition format to convert the schema file into.
  #[arg(short, long, value_enum, default_value_t = FormatTypes::Spec)]
  format: FormatTypes,

  /// Where to save the output. If no name is specified it defaults to stdout
  #[arg(short, long)]
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
    FormatTypes::Go => Database::from(&schema).to_go(),
    FormatTypes::TypeScript => Database::from(&schema).to_typescript(),
  };

  match opts.output {
    Some(name) => {
      let output_path = Path::new(&name);
      let output_display = output_path.display();
      let mut file = match File::create(&output_path) { // Use File::create to write
        Err(why) => {
          eprintln!("couldn't create {}: {}", output_display, why); // Updated error message
          return;
        }
        Ok(file) => file,
      };

      if let Err(why) = write!(file, "{}", spec) {
        eprintln!("couldn't write to {}: {}", output_display, why); // Updated error message
      }
    }
    None => println!("{}", spec),
  }
}
