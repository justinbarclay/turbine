
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
      if specs.is_empty() {
        [specs, table.to_typescript()].join("")
      } else {
        [specs, table.to_typescript()].join("\n\n")
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
      format!("{}: {};", self.name, self.value_type.to_typescript())
    }
  }
}

#[cfg(test)]
pub mod test {
  use crate::{Database, typescript::ToTypeScript};
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
      Database::from(schema).to_typescript(),
      "type SampleSchema {
   a?: number;
   b?: string;
   c?: string;
   d?: number;
   e?: number;
   f?: number;
   g?: number;
   h?: number;
   i?: string;
   j?: string;
   k?: string;
   l?: string;
   m?: bool;
   n?: any;
   o?: any;
   created_at: string;
   updated_at: string;
}"
    )
  }
}
