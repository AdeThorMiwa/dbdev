use crate::{statement::Statement, table::Table};
use std::io::{stdin, stdout, Write};

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
                match input.trim() {
                    value if value.is_empty() => continue,
                    ".exit" => {
                        break 'repl;
                    }
                    value if value.starts_with(".") => {
                        if let Err(_) = crate::meta::handlers::handle(value) {
                            println!("Unrecognised command '{}'", value)
                        }
                        continue;
                    }
                    value => {
                        if let Ok(mut statement) = Statement::new(value) {
                            if let Ok(_) = statement.execute(&mut table) {
                                println!("Executed.")
                            } else {
                                println!("command execution failed")
                            }
                        } else {
                            println!("Unrecognized keyword at start of '{}'", value)
                        }
                    }
                }
            }
        }

        Ok(())
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
