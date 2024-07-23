use crate::{
    pager::Pager,
    statement::{ExecuteError, Statement, StatementError},
    table::Table,
};

pub struct Database {
    table: Table,
}

pub enum DatabaseError {
    CloseError,
}

pub enum HandleDBQueryStatusCode {
    Exit,
    Continue,
}

impl<'a> Database {
    pub fn try_new(filename: &str) -> std::io::Result<Self> {
        let pager = Pager::try_new(filename.into())?;
        let table = Table::new(pager);
        Ok(Self { table })
    }

    pub fn handle_query(&mut self, query: &str) -> Result<HandleDBQueryStatusCode, DatabaseError> {
        let query = query.trim();

        if query.is_empty() {
            return Ok(HandleDBQueryStatusCode::Continue);
        }

        match query {
            ".exit" => {
                self.close()?;
                return Ok(HandleDBQueryStatusCode::Exit);
            }
            value if value.starts_with(".") => {
                if let Err(_) = crate::meta::handlers::handle(value) {
                    println!("Unrecognised command '{}'", value)
                }
            }
            value => match Statement::new(value) {
                Ok(mut statement) => match statement.execute(&mut self.table) {
                    Ok(_) => println!("Executed."),
                    Err(ExecuteError::TableFull) => {
                        println!("Error: Table Full")
                    }
                    Err(ExecuteError::SerializationFail(s)) => println!("{}", s),
                },
                Err(e) => match e {
                    StatementError::SynthaxError(t) => {
                        println!("Syntax Error: {}", t)
                    }
                    StatementError::UnrecognisedStatement => {
                        println!("Unrecognized keyword at start of '{}'", value)
                    }
                    StatementError::ValidationError(s) => {
                        println!("Validation Error: {}", s)
                    }
                },
            },
        }

        Ok(HandleDBQueryStatusCode::Continue)
    }

    pub fn close(&mut self) -> Result<(), DatabaseError> {
        self.table
            .flush_pages()
            .map_err(|_| DatabaseError::CloseError)
    }
}
