use clap::Parser;
use cstack_sqlite::repl::REPL;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "stackqlite.db")]
    filename: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let repl = REPL::new();
    repl.start(&args.filename)
}
