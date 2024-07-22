use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn run_script_exec<T: ToString>(scripts: Vec<T>) -> Vec<String> {
    let mut child = Command::new("cargo")
        .args(["run", "-p", "cstack_sqlite"])
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
    result.split("\n").map(|s| s.to_owned()).collect()
}

pub fn result_match<R: ToString, T: ToString>(result: Vec<R>, expected: Vec<T>) {
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(result[i].to_string(), e.to_string());
    }
}
