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
      if specs.is_empty() {
        [specs, table.to_rust()].join("")
      } else {
        [specs, table.to_rust()].join("\n\n")
      }
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

#[cfg(test)]
pub mod test {
  #[cfg(test)]
  use std::{panic, str::FromStr, vec};

  use crate::{rust::ToRust, ColumnData, Database, RailsColumn, Table};
  #[test]
  fn can_convert_a_rails_schema_to_a_string_version_of_a_rust_struct() {
    let schema = "ActiveRecord::Schema.define(version: 20_210_916_202_951) do
  create_table \"sample_schema\", id: :serial, force: :cascade do |t|
    t.primary_key \"a\"
    t.string \"b\"
    t.text \"c\"
    t.integer \"d\"
    t.bigint \"e\"
    t.float \"f\"
    t.decimal \"g\"
    t.numeric \"h\"
    t.datetime \"i\"
    t.time \"j\"
    t.date \"k\"
    t.binary \"l\"
    t.boolean \"m\"
    t.hstore \"n\"
    t.jsonb \"o\"
    t.datetime \"created_at\", null: false
    t.datetime \"updated_at\", null: false
  end
end";
    assert_eq!(
      Database::from(schema).to_rust(),
      "struct SampleSchema {
   a: Option<usize>,
   b: Option<String>,
   c: Option<String>,
   d: Option<i64>,
   e: Option<i128>,
   f: Option<f64>,
   g: Option<f64>,
   h: Option<i64>,
   i: Option<String>,
   j: Option<String>,
   k: Option<String>,
   l: Option<Vec<u8>>,
   m: Option<bool>,
   n: Option<std::collections::HashMap<String,String>>,
   o: Option<std::collections::HashMap<String,String>>,
   created_at: String,
   updated_at: String,
}"
    )
  }
}
