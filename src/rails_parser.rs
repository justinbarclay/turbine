use std::{str::FromStr, vec};

use super::{ColumnData, Database, RailsColumn, Table};

#[derive(Debug)]
pub struct RailsParseError(String);

impl FromStr for RailsColumn {
  type Err = RailsParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "t.primary_key" => Ok(RailsColumn::PrimaryKey),
      "t.string" => Ok(RailsColumn::String),
      "t.text" => Ok(RailsColumn::Text),
      "t.integer" => Ok(RailsColumn::Integer),
      "t.bigint" => Ok(RailsColumn::Bigint),
      "t.float" => Ok(RailsColumn::Float),
      "t.decimal" => Ok(RailsColumn::Decimal),
      "t.numeric" => Ok(RailsColumn::Numeric),
      "t.datetime" => Ok(RailsColumn::Datetime),
      "t.time" => Ok(RailsColumn::Time),
      "t.date" => Ok(RailsColumn::Date),
      "t.binary" => Ok(RailsColumn::Binary),
      "t.boolean" => Ok(RailsColumn::Boolean),
      "t.hstore" => Ok(RailsColumn::HStore),
      "t.jsonb" => Ok(RailsColumn::HStore),
      token => Err(RailsParseError(format!("Unable to parse token: {}", token))),
    }
  }
}

impl Table {
  fn from_tokens(mut tokens: Vec<&str>) -> (Option<Self>, Vec<&str>) {
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
            table.name = tokens[i + 1].trim_matches(matchers).to_string();
            continue;
          }

          // If not nothing we're looking at can be a valid column name
          _ => continue,
        }
      }

      // Ok so we're inside a table declaration
      let column = match (tokens[i], RailsColumn::from_str(tokens[i])) {
        (_, Ok(value_type)) => ColumnData {
          value_type,
          name: tokens[i + 1].trim_matches(matchers).to_string(),
          nullable: !(tokens[i + 2] == "null:" && tokens[i + 3] == "false"),
        },
        ("end", Err(_)) => return (Some(table), tokens.split_off(i + 1)),

        ("create_table", Err(_)) => panic!(
          "{}",
          "Warning Invalid Rails Schema:\n Found create_table inside create_table block"
        ),
        (_, Err(_)) => continue,
      };

      table.columns.push(column);
    }

    if inside_table_expression {
      panic!(
        "{}",
        "Parsing Error: \n create_table block does not have matching end"
      )
    }
    // We can only get here when there are no create_table tokens and
    // we've reached the last token in our list.
    (None, tokens.split_off(1))
  }
}

impl Database {
  pub fn from(schema: &str) -> Self {
    let mut tokens: Vec<&str> = schema.split_whitespace().collect();
    let mut database: Database = Database(vec![]);
    loop {
      let (table, remaining_tokens) = Table::from_tokens(tokens);
      if let Some(table) = table {
        database.0.push(table);
      }
      if remaining_tokens.is_empty() {
        break;
      } else {
        tokens = remaining_tokens;
      }
    }

    database
  }
}

#[cfg(test)]
mod tests {
  use std::{panic, str::FromStr, vec};

  use crate::{ColumnData, Database, RailsColumn, Table};
  #[test]
  fn rails_columns_respond_to_from_str() {
    assert_eq!(
      RailsColumn::from_str("t.integer").unwrap(),
      RailsColumn::Integer
    )
  }

  #[test]
  fn rails_columns_throws_an_error_when_it_doesnt_recognize_the_token() {
    assert!(RailsColumn::from_str("unrecognized").is_err())
  }
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

  #[test]
  fn it_parses_multiple_tables() {
    let table = "create_table \"table_1\", id: :serial, force: :cascade do |t|
    t.primary_key \"a\"
  end
  create_table \"table_2\", id: :serial, force: :cascade do |t|
    t.primary_key \"b\"
  end";
    assert_eq!(
      Database::from(table),
      Database(vec![
        Table {
          name: "table_1".to_string(),
          columns: vec![ColumnData {
            name: "a".to_string(),
            value_type: RailsColumn::PrimaryKey,
            nullable: true
          }]
        },
        Table {
          name: "table_2".to_string(),
          columns: vec![ColumnData {
            name: "b".to_string(),
            value_type: RailsColumn::PrimaryKey,
            nullable: true
          }]
        }
      ])
    );
  }

  #[test]
  fn it_panics_when_it_encounters_two_create_table_before_an_end() {
    let table = "create_table \"sample_schema\", id: :serial, force: :cascade do |t|
    t.primary_key \"a\"
    create_table
  end";

    let result = panic::catch_unwind(|| Database::from(table));
    assert!(result.is_err());
    if let Err(panic) = result {
      match panic.downcast::<String>() {
        Ok(message) => assert_eq!(
          message.as_ref(),
          "Warning Invalid Rails Schema:\n Found create_table inside create_table block"
        ),
        Err(_) => assert!(false),
      }
    }
  }

  #[test]
  fn it_can_represent_every_column_type() {
    let table = "ActiveRecord::Schema.define(version: 20_210_916_202_951) do
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
      Database::from(table),
      Database(vec![Table {
        name: "sample_schema".to_string(),
        columns: vec![
          ColumnData {
            value_type: RailsColumn::PrimaryKey,
            name: "a".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::String,
            name: "b".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Text,
            name: "c".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Integer,
            name: "d".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Bigint,
            name: "e".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Float,
            name: "f".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Decimal,
            name: "g".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Numeric,
            name: "h".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Datetime,
            name: "i".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Time,
            name: "j".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Date,
            name: "k".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Binary,
            name: "l".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Boolean,
            name: "m".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::HStore,
            name: "n".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::HStore,
            name: "o".to_string(),
            nullable: true
          },
          ColumnData {
            value_type: RailsColumn::Datetime,
            name: "created_at".to_string(),
            nullable: false
          },
          ColumnData {
            value_type: RailsColumn::Datetime,
            name: "updated_at".to_string(),
            nullable: false
          }
        ]
      }])
    );
  }
}
