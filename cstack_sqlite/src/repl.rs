use crate::db::{Database, DatabaseError, HandleDBQueryStatusCode};
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

pub struct REPL;

impl REPL {
    pub fn new() -> Self {
        Self
    }

    pub fn start(&self, db_filename: &str) -> std::io::Result<()> {
        let mut db = Database::try_new(db_filename)?;
        'repl: loop {
            self.prompt();
            if let Some(input) = self.read_input() {
                match db.handle_query(&input) {
                    Ok(HandleDBQueryStatusCode::Exit) => break 'repl,
                    Ok(HandleDBQueryStatusCode::Continue) => continue,
                    Err(DatabaseError::CloseError) => exit(1),
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
