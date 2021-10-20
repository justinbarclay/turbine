use super::ColumnData;

use super::Database;
use super::RailsColumn;
use super::Table;

pub trait ToGo {
  fn to_go(&self) -> String;
}

impl ToGo for Database {
  fn to_go(&self) -> String {
    self.0.iter().fold(String::new(), |specs, table| {
      if specs.is_empty() {
        [specs, table.to_go()].join("")
      } else {
        [specs, table.to_go()].join("\n\n")
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

impl ToGo for Table {
  fn to_go(&self) -> String {
    let spec = self.columns.iter().fold(String::new(), |spec, column| {
      [spec, "    ".to_owned() + &column.to_go()].join("\n")
    });

    let name = format_name(&self.name);
    format!("type {} struct {{{}\n}}", name, spec)
  }
}

impl ToGo for RailsColumn {
  fn to_go(&self) -> String {
    match self {
      RailsColumn::PrimaryKey => "*int64".to_string(),
      RailsColumn::String => "*string".to_string(),
      RailsColumn::Text => "*string".to_string(),
      RailsColumn::Integer => "*int64".to_string(),
      RailsColumn::Bigint => "*int128".to_string(),
      RailsColumn::Float => "*float64".to_string(),
      RailsColumn::Decimal => "*float64".to_string(),
      RailsColumn::Numeric => "*int64".to_string(),
      RailsColumn::Datetime => "*time.Time".to_string(),
      RailsColumn::Time => "*time.Time".to_string(),
      RailsColumn::Date => "*time.Time".to_string(),
      RailsColumn::Binary => "*[]uint8".to_string(),
      RailsColumn::Boolean => "*bool".to_string(),
      RailsColumn::HStore => "map[string]interface{}".to_string(),
      RailsColumn::JsonB => "map[string]interface{}".to_string(),
    }
  }
}

impl ToGo for ColumnData {
  fn to_go(&self) -> String {
    format!("{}: {},", self.name, self.value_type.to_go())
  }
}

#[cfg(test)]
pub mod test {
  use crate::{go::ToGo, Database};
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
      Database::from(schema).to_go(),
      "type SampleSchema struct {
    a: *int64,
    b: *string,
    c: *string,
    d: *int64,
    e: *int128,
    f: *float64,
    g: *float64,
    h: *int64,
    i: *time.Time,
    j: *time.Time,
    k: *time.Time,
    l: *[]uint8,
    m: *bool,
    n: map[string]interface{},
    o: map[string]interface{},
    created_at: *time.Time,
    updated_at: *time.Time,
}"
    )
  }
}
