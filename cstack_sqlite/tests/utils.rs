use rand::Rng;
use std::{
    fs::remove_file,
    io::Write,
    process::{Command, Stdio},
};

pub fn gen_random_filename() -> String {
    let rnd = rand::thread_rng().gen_range(10000..10000000);
    format!("{rnd}-stackqlite.db")
}

pub fn run_script_exec<T: ToString>(
    scripts: Vec<T>,
    filename: Option<String>,
    cleanup_db: bool,
) -> Vec<String> {
    let db_filename = filename.unwrap_or(gen_random_filename());

    let mut child = Command::new("cargo")
        .args([
            "run",
            "-p",
            "cstack_sqlite",
            "--",
            "--filename",
            db_filename.as_str(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = child.stdin.take() {
        for script in scripts {
            stdin
                .write_all(&[script.to_string().as_bytes(), b"\n"].concat())
                .unwrap();
        }
    }

    let output = child.wait_with_output().unwrap();
    let result = String::from_utf8(output.stdout).unwrap();

    // cleanup
    if cleanup_db {
        remove_file(db_filename).unwrap();
    }

    result.split("\n").map(|s| s.to_owned()).collect()
}

pub fn run_script_exec_with_defaults<T: ToString>(scripts: Vec<T>) -> Vec<String> {
    run_script_exec(scripts, None, true)
}

pub fn result_match<R: ToString, T: ToString>(result: Vec<R>, expected: Vec<T>) {
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(result[i].to_string(), e.to_string());
    }
}
