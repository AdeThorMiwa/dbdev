use crate::{
    row::{Row, ROW_SIZE},
    table::Table,
};

pub enum Statement {
    Select,
    Insert { row: Row },
}

pub enum StatementError {
    UnrecognisedStatement,
}

pub enum ExecuteError {
    TableFull,
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
                    .expect("invalid id")
                    .parse::<u32>()
                    .expect("invalid id: non integer");

                let username = tokens.next().expect("invalid username");
                let email = tokens.next().expect("invalid email");
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

    fn execute_insert(row: &Row, table: &mut Table) -> Result<(), ExecuteError> {
        if table.num_rows >= table.max_rows() as u32 {
            return Err(ExecuteError::TableFull);
        }

        let row_slot = table.get_row_slot(table.num_rows);
        row.serialize(&mut row_slot[..ROW_SIZE]);
        table.increment_num_rows();

        Ok(())
    }

    fn execute_select(table: &mut Table) -> Result<(), ExecuteError> {
        let mut i = 0;
        while i < table.num_rows {
            let row_slot = table.get_row_slot(i);
            let row = Row::deserialize(&row_slot[..ROW_SIZE]);
            println!("{:?}", row);
            i += 1;
        }
        Ok(())
    }
}
