use super::Database;
use super::RailsColumn;
use super::Table;

trait ToSpec {
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
    let formatter = |key, type_decl| format!(":{} {}", &key, type_decl);
    match self {
      RailsColumn::PrimaryKey(key) => formatter(key, "int?"),
      RailsColumn::String(key) => formatter(key, "string?"),
      RailsColumn::Text(key) => formatter(key, "string?"),
      RailsColumn::Integer(key) => formatter(key, "int?"),
      RailsColumn::Bigint(key) => formatter(key, "int?"),
      RailsColumn::Float(key) => formatter(key, "float?"),
      RailsColumn::Decimal(key) => formatter(key, "float?"),
      RailsColumn::Numeric(key) => formatter(key, "int?"),
      RailsColumn::Datetime(key) => formatter(key, "string?"),
      RailsColumn::Time(key) => formatter(key, "string?"),
      RailsColumn::Date(key) => formatter(key, "string?"),
      RailsColumn::Binary(key) => formatter(key, "string?"),
      RailsColumn::Boolean(key) => formatter(key, "boolean?"),
    }
  }
}
