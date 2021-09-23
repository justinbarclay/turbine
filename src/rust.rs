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

impl ToRust for RailsColumn {
  fn to_rust(&self) -> String {
    match self {
      RailsColumn::PrimaryKey => "usize".to_string(),
      RailsColumn::String => "String".to_string(),
      RailsColumn::Text => "String".to_string(),
      RailsColumn::Integer => "i64".to_string(),
      RailsColumn::Bigint => "i128".to_string(),
      RailsColumn::Float => "f64".to_string(),
      RailsColumn::Decimal => "f64".to_string(),
      RailsColumn::Numeric => "i64".to_string(),
      RailsColumn::Datetime => "String".to_string(),
      RailsColumn::Time => "String".to_string(),
      RailsColumn::Date => "String".to_string(),
      RailsColumn::Binary => "Vec<u8>".to_string(),
      RailsColumn::Boolean => "bool".to_string(),
      RailsColumn::HStore => "std::collections::HashMap<String,String>".to_string(),
      RailsColumn::JsonB => "std::collections::HashMap<String,String>".to_string(),
    }
  }
}

impl ToRust for ColumnData {
  fn to_rust(&self) -> String {
    if self.nullable {
      format!("{}: Option<{}>,", self.name, self.value_type.to_rust())
    } else {
      format!("{}: {},", self.name, self.value_type.to_rust())
    }
  }
}
