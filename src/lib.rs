pub mod rails_parser;
// An enum contains it's type and key value
#[derive(Debug)]
pub enum RailsColumn {
  PrimaryKey(String),
  String(String),
  Text(String),
  Integer(String),
  Bigint(String),
  Float(String),
  Decimal(String),
  Numeric(String),
  Datetime(String),
  Time(String),
  Date(String),
  Binary(String),
  Boolean(String),
}

#[derive(Debug)]
pub struct Table {
  name: String,
  columns: Vec<RailsColumn>,
}

#[derive(Debug)]
pub struct Database(Vec<Table>);
