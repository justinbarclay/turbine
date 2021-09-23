use crate::ColumnData;

use super::Database;
use super::RailsColumn;
use super::Table;

pub trait ToTypeScript {
  fn to_typescript(&self) -> String;
}

impl ToTypeScript for Database {
  fn to_typescript(&self) -> String {
    self.0.iter().fold(String::new(), |specs, table| {
      [specs, table.to_typescript()].join("\n\n")
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

impl ToTypeScript for Table {
  fn to_typescript(&self) -> String {
    let spec = self.columns.iter().fold(String::new(), |spec, column| {
      [spec, "   ".to_owned() + &column.to_typescript()].join("\n")
    });

    let name = format_name(&self.name);
    format!("type {} {{{}\n}}", name, spec)
  }
}

fn formatter (key: &str, type_decl: &str, nullable: bool) -> String{
  if nullable {
    format!("{}?: {};", &key, type_decl)
  } else {
    format!("{}: {};", &key, type_decl)
  }
}

impl ToTypeScript for ColumnData {
  fn to_typescript(&self) -> String {
    match self.value_type {
      RailsColumn::PrimaryKey => formatter(&self.name, "number", self.nullable),
      RailsColumn::String => formatter(&self.name, "string", self.nullable),
      RailsColumn::Text => formatter(&self.name, "string", self.nullable),
      RailsColumn::Integer => formatter(&self.name, "number", self.nullable),
      RailsColumn::Bigint => formatter(&self.name, "number", self.nullable),
      RailsColumn::Float => formatter(&self.name, "number", self.nullable),
      RailsColumn::Decimal => formatter(&self.name, "number", self.nullable),
      RailsColumn::Numeric => formatter(&self.name, "number", self.nullable),
      RailsColumn::Datetime => formatter(&self.name, "string", self.nullable),
      RailsColumn::Time => formatter(&self.name, "string", self.nullable),
      RailsColumn::Date => formatter(&self.name, "string", self.nullable),
      RailsColumn::Binary => formatter(&self.name, "string", self.nullable), // Assuming byte64 buffer
      RailsColumn::Boolean => formatter(&self.name, "bool", self.nullable),
      RailsColumn::JsonB => formatter(&self.name, "any", self.nullable),
      RailsColumn::HStore => formatter(&self.name, "any", self.nullable),
    }
  }
}
