use super::ColumnData;
use super::Database;
use super::RailsColumn;
use super::Table;

pub trait ToSpec {
  fn to_spec(&self) -> String;
}

impl ToSpec for Database {
  fn to_spec(&self) -> String {
    self.0.iter().fold(String::new(), |specs, table| {
      if specs.is_empty() {
        [specs, table.to_spec()].join("")
      } else {
        [specs, table.to_spec()].join("\n\n")
      }
    })
  }
}

impl ToSpec for Table {
  fn to_spec(&self) -> String {
    let spec = self.columns.iter().fold(String::new(), |spec, column| {
      if spec.is_empty() {
        spec + &column.to_spec()
      } else {
        [spec, "   ".to_owned() + &column.to_spec()].join("\n")
      }
    });
    // Remove last new line
    format!("(spec/def {}\n  {{{}}})", self.name, spec)
  }
}

impl ToSpec for RailsColumn {
  fn to_spec(&self) -> String {
    match self {
      RailsColumn::PrimaryKey => "int?".to_string(),
      RailsColumn::String => "string?".to_string(),
      RailsColumn::Text => "string?".to_string(),
      RailsColumn::Integer => "int?".to_string(),
      RailsColumn::Bigint => "int?".to_string(),
      RailsColumn::Float => "float?".to_string(),
      RailsColumn::Decimal => "float?".to_string(),
      RailsColumn::Numeric => "int?".to_string(),
      RailsColumn::Datetime => "string?".to_string(),
      RailsColumn::Time => "string?".to_string(),
      RailsColumn::Date => "string?".to_string(),
      RailsColumn::Binary => "string?".to_string(),
      RailsColumn::Boolean => "boolean?".to_string(),
      RailsColumn::HStore => "map?".to_string(),
      RailsColumn::JsonB => "map?".to_string(),
    }
  }
}
impl ToSpec for ColumnData {
  fn to_spec(&self) -> String {
    format!(":{} {}", self.name, self.value_type.to_spec())
  }
}

#[cfg(test)]
pub mod test {
  use crate::{spec::ToSpec, Database};

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
      Database::from(schema).to_spec(),
      "(spec/def sample_schema
  {:a int?
   :b string?
   :c string?
   :d int?
   :e int?
   :f float?
   :g float?
   :h int?
   :i string?
   :j string?
   :k string?
   :l string?
   :m boolean?
   :n map?
   :o map?
   :created_at string?
   :updated_at string?})",
    )
  }
}
