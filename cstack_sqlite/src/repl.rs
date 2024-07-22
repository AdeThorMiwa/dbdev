use crate::{
    statement::{ExecuteError, Statement, StatementError},
    table::Table,
};
use std::io::{stdin, stdout, ErrorKind, Write};

pub struct REPL {}

impl REPL {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) -> std::io::Result<()> {
        let mut table = Table::new();
        'repl: loop {
            self.prompt();
            if let Some(input) = self.read_input() {
                if let Err(_) = self.handle_input(&input, &mut table) {
                    break 'repl;
                }
            }
        }

        Ok(())
    }

    fn handle_input(&self, input: &str, table: &mut Table) -> std::io::Result<()> {
        match input.trim() {
            value if value.is_empty() => Ok(()),
            ".exit" => Err(std::io::Error::new(ErrorKind::UnexpectedEof, "Exit")),
            value if value.starts_with(".") => {
                if let Err(_) = crate::meta::handlers::handle(value) {
                    println!("Unrecognised command '{}'", value)
                }
                Ok(())
            }
            value => {
                let statement = Statement::new(value);
                if let Ok(mut statement) = statement {
                    match statement.execute(table) {
                        Ok(_) => println!("Executed."),
                        Err(ExecuteError::TableFull) => {
                            println!("Error: Table Full")
                        }
                        Err(ExecuteError::SerializationFail(s)) => println!("{}", s),
                    }
                } else if let Err(e) = statement {
                    match e {
                        StatementError::SynthaxError(t) => {
                            println!("Syntax Error: {}", t)
                        }
                        StatementError::UnrecognisedStatement => {
                            println!("Unrecognized keyword at start of '{}'", value)
                        }
                        StatementError::ValidationError(s) => {
                            println!("Validation Error: {}", s)
                        }
                    }
                }
                Ok(())
            }
        }
    }

    fn prompt(&self) {
        let mut writer = stdout();
        let _ = writer.write("csquarelite> ".as_bytes());
        let _ = writer.flush();
    }

    fn read_input(&self) -> Option<String> {
        let mut buf = String::new();
        let _ = stdin().read_line(&mut buf);
        Some(buf)
    }
}
