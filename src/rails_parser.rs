use std::{io, vec};

use super::{Database, RailsColumn, Table};

impl Table {
  fn from_tokens(mut tokens: Vec<&str>) -> Result<(Self, Vec<&str>), io::ErrorKind> {
    let mut table = Table {
      name: "".to_string(),
      columns: vec![],
    };
    let matchers: &[_] = &['"', ','];
    let mut inside_table_expression = false;

    for i in 0..tokens.len() {
      // Check to see if we're in a table expression
      if !inside_table_expression {
        match tokens[i] {
          // If not and the next token is the table name
          "create_table" => {
            inside_table_expression = true;
            table.name = tokens[i + 1].trim_matches(matchers).to_string()
          }

          // If not nothing we're looking at can be a valid column name
          _ => continue,
        }
      }

      // Ok so we're inside a table declaration
      let column = match tokens[i] {
        "t.primary_key" => {
          RailsColumn::PrimaryKey(tokens[i + 1].trim_matches(matchers).to_string())
        }
        "t.string" => RailsColumn::String(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.text" => RailsColumn::Text(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.integer" => RailsColumn::Integer(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.bigint" => RailsColumn::Bigint(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.float" => RailsColumn::Float(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.decimal" => RailsColumn::Decimal(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.numeric" => RailsColumn::Numeric(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.datetime" => RailsColumn::Datetime(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.time" => RailsColumn::Time(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.date" => RailsColumn::Date(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.binary" => RailsColumn::Binary(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.boolean" => RailsColumn::Boolean(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.hstore" => RailsColumn::HStore(tokens[i + 1].trim_matches(matchers).to_string()),
        "t.jsonb" => RailsColumn::HStore(tokens[i + 1].trim_matches(matchers).to_string()),
        "end" => return Ok((table, tokens.split_off(i))),
        _ => continue,
      };
      table.columns.push(column);
    }
    Err(io::ErrorKind::NotFound)
  }
}

impl Database {
  pub fn from(schema: &str) -> Self {
    let mut tokens: Vec<&str> = schema.split_whitespace().collect();
    let mut database: Database = Database(vec![]);
    loop {
      if let Ok((table, remaining_tokens)) = Table::from_tokens(tokens) {
        database.0.push(table);
        if remaining_tokens.is_empty() {
          break;
        } else {
          tokens = remaining_tokens;
        }
      } else {
        return database;
      };
    }

    database
  }
}
