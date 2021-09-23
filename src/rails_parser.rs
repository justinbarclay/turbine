use std::{io, vec};

use super::{ColumnData, Database, RailsColumn, Table};

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
        "t.primary_key" => ColumnData {
          value_type: RailsColumn::PrimaryKey,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.string" => ColumnData {
          value_type: RailsColumn::String,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.text" => ColumnData {
          value_type: RailsColumn::Text,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.integer" => ColumnData {
          value_type: RailsColumn::Integer,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.bigint" => ColumnData {
          value_type: RailsColumn::Bigint,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.float" => ColumnData {
          value_type: RailsColumn::Float,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.decimal" => ColumnData {
          value_type: RailsColumn::Decimal,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.numeric" => ColumnData {
          value_type: RailsColumn::Numeric,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.datetime" => ColumnData {
          value_type: RailsColumn::Datetime,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.time" => ColumnData {
          value_type: RailsColumn::Time,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.date" => ColumnData {
          value_type: RailsColumn::Date,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.binary" => ColumnData {
          value_type: RailsColumn::Binary,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.boolean" => ColumnData {
          value_type: RailsColumn::Boolean,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.hstore" => ColumnData {
          value_type: RailsColumn::HStore,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        "t.jsonb" => ColumnData {
          value_type: RailsColumn::HStore,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
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

#[cfg(test)]
mod tests {
  use std::vec;

  use crate::{ColumnData, Database, RailsColumn, Table};

  #[test]
  fn it_parses_a_simple_rails_table_definition() {
    let table = "create_table \"sample_schema\", id: :serial, force: :cascade do |t|
    t.primary_key \"a\"
  end";
    assert_eq!(
      Database::from(table),
      Database(vec![Table {
        name: "sample_schema".to_string(),
        columns: vec![ColumnData {
          name: "a".to_string(),
          value_type: RailsColumn::PrimaryKey,
          nullable: true
        }]
      }])
    );
  }
}
