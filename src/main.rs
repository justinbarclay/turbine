use schema_parser::Database;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Open the path in read-only mode, returns `io::Result<File>`
    let path = Path::new("/home/justin/dev/tidal/application-inventory/db/schema.rb");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut schema = String::new();
    match file.read_to_string(&mut schema) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, schema),
    }
    println!("{}", Database::from(&schema).to_spec())
}
