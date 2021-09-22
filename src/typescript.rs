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
    let formatter = |key, type_decl| format!("{}: {};", &key, type_decl);
    match self {
      RailsColumn::PrimaryKey(key) => formatter(key, "number"),
      RailsColumn::String(key) => formatter(key, "string"),
      RailsColumn::Text(key) => formatter(key, "string"),
      RailsColumn::Integer(key) => formatter(key, "number"),
      RailsColumn::Bigint(key) => formatter(key, "number"),
      RailsColumn::Float(key) => formatter(key, "number"),
      RailsColumn::Decimal(key) => formatter(key, "number"),
      RailsColumn::Numeric(key) => formatter(key, "number"),
      RailsColumn::Datetime(key) => formatter(key, "string"),
      RailsColumn::Time(key) => formatter(key, "string"),
      RailsColumn::Date(key) => formatter(key, "string"),
      RailsColumn::Binary(key) => formatter(key, "string"), // Assuming byte64 buffer
      RailsColumn::Boolean(key) => formatter(key, "bool"),
      RailsColumn::JsonB(key) => formatter(key, "any"),
      RailsColumn::HStore(key) => formatter(key, "any"),
    }
  }
}
