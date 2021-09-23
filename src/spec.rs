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
    if self.nullable {
      format!(":{} {}", self.name, self.value_type.to_spec())
    } else {
      format!(":{} {}", self.name, self.value_type.to_spec())
    }
  }
}
