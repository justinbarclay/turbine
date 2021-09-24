pub mod rails_parser;
pub mod rust;
pub mod spec;
pub mod typescript;

#[derive(Debug, PartialEq)]
pub struct ColumnData {
  value_type: RailsColumn,
  name: String,
  nullable: bool,
}

// An enum contains it's type and key value
#[derive(Debug, PartialEq)]
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

impl PartialEq for Table {
  fn eq(&self, other: &Self) -> bool {
    if self.columns.len() != other.columns.len() {
      return false;
    }
    let eq_names = self.name == other.name;
    let contains_self = self.columns.iter().all(|column| other.columns.contains(column));
    let contains_other = other.columns.iter().all(|column| self.columns.contains(column));

    eq_names && contains_self && contains_other
  }
}

#[derive(Debug, PartialEq)]
pub struct Database(Vec<Table>);

mod tests {
  #![allow(unused_imports)]
  use crate::{ColumnData, Database, RailsColumn, Table};

  #[test]
  fn rails_columns_are_equal() {
    assert_eq!(RailsColumn::PrimaryKey, RailsColumn::PrimaryKey)
  }

  #[test]
  fn rails_columns_are_equal_but_not_when_you_look_at_them_sideways() {
    assert_ne!(RailsColumn::PrimaryKey, RailsColumn::Bigint)
  }

  #[test]
  fn yup_thats_the_same_column_data() {
    assert_eq!(
      ColumnData {
        name: "b".to_string(),
        value_type: RailsColumn::PrimaryKey,
        nullable: true
      },
      ColumnData {
        name: "b".to_string(),
        value_type: RailsColumn::PrimaryKey,
        nullable: true
      }
    )
  }

  #[test]
  fn thought_it_was_column_data_nope_chuck_testa() {
    assert_ne!(
      ColumnData {
        name: "b".to_string(),
        value_type: RailsColumn::PrimaryKey,
        nullable: true
      },
      ColumnData {
        name: "chuck_testa".to_string(),
        value_type: RailsColumn::PrimaryKey,
        nullable: true
      }
    )
  }
  #[test]
  fn you_better_believe_tables_can_equal() {
    assert_eq!(
      Table {
        name: "sample_schema".to_string(),
        columns: vec![ColumnData {
          name: "b".to_string(),
          value_type: RailsColumn::PrimaryKey,
          nullable: true
        }]
      },
      Table {
        name: "sample_schema".to_string(),
        columns: vec![ColumnData {
          name: "b".to_string(),
          value_type: RailsColumn::PrimaryKey,
          nullable: true
        }]
      }
    )
  }

  #[test]
  fn tables_are_the_most_equal_when_they_are_empty() {
    assert_eq!(
      Table {
        name: "sample_schema".to_string(),
        columns: vec![]
      },
      Table {
        name: "sample_schema".to_string(),
        columns: vec![]
      }
    )
  }

  #[test]
  fn tables_of_different_sizes_kinda_hate_each_other() {
    assert_ne!(
      Table {
        name: "sample_schema".to_string(),
        columns: vec![]
      },
      Table {
        name: "sample_schema".to_string(),
        columns: vec![ColumnData {
          name: "b".to_string(),
          value_type: RailsColumn::PrimaryKey,
          nullable: true
        }]
      }
    )
  }
  #[test]
  fn itd_be_a_shame_if_these_tables_dont_equal_not() {
    assert_ne!(
      Table {
        name: "my_table".to_string(),
        columns: vec![ColumnData {
          name: "b".to_string(),
          value_type: RailsColumn::PrimaryKey,
          nullable: true
        }]
      },
      Table {
        name: "also_my_table".to_string(),
        columns: vec![ColumnData {
          name: "b".to_string(),
          value_type: RailsColumn::PrimaryKey,
          nullable: true
        }]
      }
    )
  }

  #[test]
  fn databases_can_be_not_equal() {
    assert_ne!(
      Database(vec![Table {
        name: "sample_schema".to_string(),
        columns: vec![ColumnData {
          name: "b".to_string(),
          value_type: RailsColumn::PrimaryKey,
          nullable: true
        }]
      }]),
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
  fn databases_can_be_equal() {
    assert_eq!(
      Database(vec![Table {
        name: "sample_schema".to_string(),
        columns: vec![ColumnData {
          name: "a".to_string(),
          value_type: RailsColumn::PrimaryKey,
          nullable: true
        }]
      }]),
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
