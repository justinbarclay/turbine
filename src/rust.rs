use super::Database;
use super::RailsColumn;
use super::Table;

pub trait ToRust {
  fn to_rust(&self) -> String;
}

impl ToRust for Database {
  fn to_rust(&self) -> String {
    self.0.iter().fold(String::new(), |specs, table| {
      [specs, table.to_rust()].join("\n\n")
    })
  }
}
fn format_name(name: &str) -> String {
  name
    .split("_")
    .map(|n| n.chars().nth(0).unwrap().to_uppercase().to_string() + &n[1..])
    .collect::<Vec<String>>()
    .join("")
}

impl ToRust for Table {
  fn to_rust(&self) -> String {
    let spec = self.columns.iter().fold(String::new(), |spec, column| {
      [spec, "   ".to_owned() + &column.to_rust()].join("\n")
    });

    let name = format_name(&self.name);
    format!("struct {} {{{}\n}}", name, spec)
  }
}

impl ToRust for RailsColumn {
  fn to_rust(&self) -> String {
    let formatter = |key, type_decl| format!("{}: {},", &key, type_decl);
    match self {
      RailsColumn::PrimaryKey(key) => formatter(key, "usize"),
      RailsColumn::String(key) => formatter(key, "String"),
      RailsColumn::Text(key) => formatter(key, "String"),
      RailsColumn::Integer(key) => formatter(key, "i64"),
      RailsColumn::Bigint(key) => formatter(key, "i128"),
      RailsColumn::Float(key) => formatter(key, "f64"),
      RailsColumn::Decimal(key) => formatter(key, "f64"),
      RailsColumn::Numeric(key) => formatter(key, "i64"),
      RailsColumn::Datetime(key) => formatter(key, "String"),
      RailsColumn::Time(key) => formatter(key, "String"),
      RailsColumn::Date(key) => formatter(key, "String"),
      RailsColumn::Binary(key) => formatter(key, "Vec<u8>"),
      RailsColumn::Boolean(key) => formatter(key, "bool"),
      RailsColumn::HStore(key) => formatter(key, "std::collections::HashMap<String,String>"),
      RailsColumn::JsonB(key) => formatter(key, "std::collections::HashMap<String,String>"),
    }
  }
}
