use crate::{
    row::{Row, RowSerializationError, ROW_SIZE},
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
                        StatementError::ValidationError("Integer value for 'id' cannot be negative".to_string())
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

    fn execute_insert(row: &mut Row, table: &mut Table) -> Result<(), ExecuteError> {
        if table.get_row_len() >= table.max_rows()  {
            return Err(ExecuteError::TableFull);
        }

        let row_slot = table.get_row_slot(table.get_row_len());
        row.serialize(&mut row_slot[..ROW_SIZE])
            .map_err(|e| match e {
                RowSerializationError::StringTooLong { field } => {
                    ExecuteError::SerializationFail(format!("String value for '{field}' too long."))
                }
            })?;
        table.increment_num_rows();

        Ok(())
    }

    fn execute_select(table: &mut Table) -> Result<(), ExecuteError> {
        let mut i = 0;
        while i < table.get_row_len() {
            let row_slot = table.get_row_slot(i);
            let row = Row::deserialize(&row_slot[..ROW_SIZE]);
            println!("{:?}", row);
            i += 1;
        }
        Ok(())
    }
}
