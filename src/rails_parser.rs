use std::{io, vec};

use super::{ColumnData, Database, RailsColumn, Table};

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
        "end" => return (Some(table), tokens.split_off(i + 1)),

        "create_table" => panic!(
          "{}",
          "Warning Invalid Rails Schema:\n Found create_table inside create_table block"
        ),
        _ => continue,
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
  use std::{panic, vec};

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
      Database(vec![
        Table {
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
        }
      ])
    );
  }
}
