use std::{
    io,
    vec,
};

// An enum contains it's type and key value
#[derive(Debug)]
enum RailsColumn {
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

impl RailsColumn {
    pub fn to_spec(&self) -> String {
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
#[derive(Debug)]
pub struct Table {
    name: String,
    columns: Vec<RailsColumn>,
}

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
                "t.integer" => {
                    RailsColumn::Integer(tokens[i + 1].trim_matches(matchers).to_string())
                }
                "t.bigint" => RailsColumn::Bigint(tokens[i + 1].trim_matches(matchers).to_string()),
                "t.float" => RailsColumn::Float(tokens[i + 1].trim_matches(matchers).to_string()),
                "t.decimal" => {
                    RailsColumn::Decimal(tokens[i + 1].trim_matches(matchers).to_string())
                }
                "t.numeric" => {
                    RailsColumn::Numeric(tokens[i + 1].trim_matches(matchers).to_string())
                }
                "t.datetime" => {
                    RailsColumn::Datetime(tokens[i + 1].trim_matches(matchers).to_string())
                }
                "t.time" => RailsColumn::Time(tokens[i + 1].trim_matches(matchers).to_string()),
                "t.date" => RailsColumn::Date(tokens[i + 1].trim_matches(matchers).to_string()),
                "t.binary" => RailsColumn::Binary(tokens[i + 1].trim_matches(matchers).to_string()),
                "t.boolean" => {
                    RailsColumn::Boolean(tokens[i + 1].trim_matches(matchers).to_string())
                }
                "end" => return Ok((table, tokens.split_off(i))),
                _ => continue,
            };
            table.columns.push(column);
        }
        Err(io::ErrorKind::NotFound)
    }
    pub fn to_spec(&self) -> String {
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
#[derive(Debug)]
pub struct Database(Vec<Table>);

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

    pub fn to_spec(self) -> String {
        self.0.iter().fold(String::new(), |specs, table| {
          [specs, table.to_spec()].join("\n\n")
        })
    }
}
