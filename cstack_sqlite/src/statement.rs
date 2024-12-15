use crate::{
    btree::node::NodeError,
    cursor::Cursor,
    row::{Row, RowSerializationError},
    table::Table,
};

pub enum Statement {
    Select,
    Insert { row: Row },
}

pub enum StatementError {
    UnrecognisedStatement,
    SynthaxError(String),
    ValidationError(String),
}

pub enum ExecuteError {
    TableFull,
    SerializationFail(String),
}

impl Statement {
    pub fn new(token: &str) -> Result<Self, StatementError> {
        Self::parse_token_to_statement(token)
    }

    fn parse_token_to_statement(s: &str) -> Result<Self, StatementError> {
        Ok(match s {
            t if t.starts_with("insert") => {
                let mut tokens = t.split(" ");
                tokens.next();

                let id = tokens
                    .next()
                    .ok_or(StatementError::SynthaxError("invalid id".to_string()))?
                    .parse::<u32>()
                    .map_err(|_| {
                        StatementError::ValidationError(
                            "Integer value for 'id' cannot be negative".to_string(),
                        )
                    })?;

                let username = tokens
                    .next()
                    .ok_or(StatementError::SynthaxError("invalid username".to_string()))?;
                let email = tokens
                    .next()
                    .ok_or(StatementError::SynthaxError("invalid email".to_string()))?;
                let row = Row::new(id, username.to_string(), email.to_string());
                Statement::Insert { row }
            }
            t if t.starts_with("select") => Statement::Select,
            _ => return Err(StatementError::UnrecognisedStatement),
        })
    }

    pub fn execute(&mut self, table: &mut Table) -> Result<(), ExecuteError> {
        match self {
            Self::Insert { row } => Self::execute_insert(row, table),
            Self::Select => Self::execute_select(table),
        }
    }

    fn execute_insert(row: &mut Row, mut table: &mut Table) -> Result<(), ExecuteError> {
        let cursor = Cursor::end(&mut table);
        let mut page = table.pager.get_page_mut(cursor.page());

        page.insert(row.id, row, &mut table).map_err(|e| match e {
            NodeError::SerializationError(se) => match se {
                RowSerializationError::StringTooLong { field } => {
                    ExecuteError::SerializationFail(format!("String value for '{field}' too long."))
                }
            },
            NodeError::NodeFullInsertError => ExecuteError::TableFull,
        })?;

        Ok(())
    }

    fn execute_select(mut table: &mut Table) -> Result<(), ExecuteError> {
        let mut cursor = Cursor::start(&mut table);

        while !cursor.end_of_table() {
            let mut page = table.pager.get_page_mut(cursor.page());
            let value = page.get_cell_value(cursor.cell_num());
            let row = Row::deserialize(value);
            println!("{:?}", row);
            cursor.advance(&mut table);
        }
        Ok(())
    }
}
