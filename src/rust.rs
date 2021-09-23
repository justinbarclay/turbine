use crate::ColumnData;

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
    .split('_')
    .map(|n| n.chars().next().unwrap().to_uppercase().to_string() + &n[1..])
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

fn formatter (key: &str, type_decl: &str, nullable: bool) -> String{
  if nullable {
    format!("{}: Option<{}>,", &key, type_decl)
  } else {
    format!("{}: {},", &key, type_decl)
  }
}

impl ToRust for ColumnData {
  fn to_rust(&self) -> String {
    match self.value_type {
      RailsColumn::PrimaryKey => formatter(&self.name, "usize", self.nullable),
      RailsColumn::String => formatter(&self.name, "String", self.nullable),
      RailsColumn::Text => formatter(&self.name, "String", self.nullable),
      RailsColumn::Integer => formatter(&self.name, "i64", self.nullable),
      RailsColumn::Bigint => formatter(&self.name, "i128", self.nullable),
      RailsColumn::Float => formatter(&self.name, "f64", self.nullable),
      RailsColumn::Decimal => formatter(&self.name, "f64", self.nullable),
      RailsColumn::Numeric => formatter(&self.name, "i64", self.nullable),
      RailsColumn::Datetime => formatter(&self.name, "String", self.nullable),
      RailsColumn::Time => formatter(&self.name, "String", self.nullable),
      RailsColumn::Date => formatter(&self.name, "String", self.nullable),
      RailsColumn::Binary => formatter(&self.name, "Vec<u8>", self.nullable),
      RailsColumn::Boolean => formatter(&self.name, "bool", self.nullable),
      RailsColumn::HStore => formatter(&self.name, "std::collections::HashMap<String,String>", self.nullable),
      RailsColumn::JsonB => formatter(&self.name, "std::collections::HashMap<String,String>", self.nullable),
    }
  }
}
