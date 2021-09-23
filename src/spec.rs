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
      [specs, table.to_spec()].join("\n\n")
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

fn formatter (key: &str, type_decl: &str, nullable: bool) -> String{
  if nullable {
    format!(":{} {}", &key, type_decl)
  } else {
    format!(":{} {}", &key, type_decl)
  }
}

impl ToSpec for ColumnData {
  fn to_spec(&self) -> String {
    match self.value_type {
      RailsColumn::PrimaryKey => formatter(&self.name, "int?", self.nullable),
      RailsColumn::String => formatter(&self.name, "string?", self.nullable),
      RailsColumn::Text => formatter(&self.name, "string?", self.nullable),
      RailsColumn::Integer => formatter(&self.name, "int?", self.nullable),
      RailsColumn::Bigint => formatter(&self.name, "int?", self.nullable),
      RailsColumn::Float => formatter(&self.name, "float?", self.nullable),
      RailsColumn::Decimal => formatter(&self.name, "float?", self.nullable),
      RailsColumn::Numeric => formatter(&self.name, "int?", self.nullable),
      RailsColumn::Datetime => formatter(&self.name, "string?", self.nullable),
      RailsColumn::Time => formatter(&self.name, "string?", self.nullable),
      RailsColumn::Date => formatter(&self.name, "string?", self.nullable),
      RailsColumn::Binary => formatter(&self.name, "string?", self.nullable),
      RailsColumn::Boolean => formatter(&self.name, "boolean?", self.nullable),
      RailsColumn::HStore => formatter(&self.name, "map?", self.nullable),
      RailsColumn::JsonB => formatter(&self.name, "map?", self.nullable),
    }
  }
}
