pub mod rails_parser;
pub mod rust;
pub mod spec;
pub mod typescript;

#[derive(Debug)]
pub struct ColumnData {
  value_type: RailsColumn,
  name: String,
  nullable: bool,
}

// An enum contains it's type and key value
#[derive(Debug)]
pub enum RailsColumn {
  PrimaryKey,
  String,
  Text,
  Integer,
  Bigint,
  Float,
  Decimal,
  Numeric,
  Datetime,
  Time,
  Date,
  Binary,
  HStore,
  JsonB,
  Boolean,
}

#[derive(Debug)]
pub struct Table {
  name: String,
  columns: Vec<ColumnData>,
}

#[derive(Debug)]
pub struct Database(Vec<Table>);
