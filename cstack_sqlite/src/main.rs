use cstack_sqlite::repl::REPL;

fn main() -> std::io::Result<()> {
    let repl = REPL::new();
    repl.start()
}
