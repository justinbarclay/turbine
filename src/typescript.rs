use crate::rust::ToRust;
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

impl ToTypeScript for RailsColumn {
  fn to_typescript(&self) -> String {
    match self {
      RailsColumn::PrimaryKey => "number".to_string(),
      RailsColumn::String => "string".to_string(),
      RailsColumn::Text => "string".to_string(),
      RailsColumn::Integer => "number".to_string(),
      RailsColumn::Bigint => "number".to_string(),
      RailsColumn::Float => "number".to_string(),
      RailsColumn::Decimal => "number".to_string(),
      RailsColumn::Numeric => "number".to_string(),
      RailsColumn::Datetime => "string".to_string(),
      RailsColumn::Time => "string".to_string(),
      RailsColumn::Date => "string".to_string(),
      RailsColumn::Binary => "string".to_string(),
      RailsColumn::Boolean => "bool".to_string(),
      RailsColumn::JsonB => "any".to_string(),
      RailsColumn::HStore => "any".to_string(),
    }
  }
}

impl ToTypeScript for ColumnData {
  fn to_typescript(&self) -> String {
    if self.nullable {
      format!("{}?: {};", self.name, self.value_type.to_typescript())
    } else {
      format!("{}: {};", self.name, self.value_type.to_rust())
    }
  }
}
