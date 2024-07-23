use utils::{gen_random_filename, result_match, run_script_exec, run_script_exec_with_defaults};
mod utils;

// TODO: dry out test cases with a macro
// TODO: add non-nullable test for columns

#[test]
fn inserts_and_retrieves_row() {
    let scripts = vec!["insert 1 user1 person1@example.com", "select", ".exit"];
    let results = run_script_exec_with_defaults(scripts);
    result_match(
        results,
        vec![
            "csquarelite> Executed.",
            "csquarelite> Row { id: 1, username: \"user1\", email: \"person1@example.com\" }",
            "Executed.",
            "csquarelite> ",
        ],
    );
}

#[test]
fn print_error_message_when_table_is_full() {
    let mut scripts = vec![];
    for i in 0..1401 {
        scripts.push(format!("insert {i} user{i} person{i}@example.com"));
    }
    scripts.push(".exit".to_owned());
    let results = run_script_exec_with_defaults(scripts);

    assert_eq!(results[results.len() - 2], "csquarelite> Error: Table Full")
}

#[test]
fn allows_inserting_strings_that_are_max_length() {
    let username = "a".repeat(32);
    let email = "a".repeat(255);
    let scripts = vec![
        format!("insert 1 {username} {email}"),
        "select".to_string(),
        ".exit".to_string(),
    ];
    let results = run_script_exec_with_defaults(scripts);

    result_match(
        results,
        vec![
            "csquarelite> Executed.",
            format!("csquarelite> Row {{ id: 1, username: \"{username}\", email: \"{email}\" }}")
                .as_str(),
            "Executed.",
            "csquarelite> ",
        ],
    );
}

#[test]
fn prints_error_message_if_strings_are_too_long() {
    let username = "a".repeat(33);
    let email = "a".repeat(256);
    let scripts = vec![
        format!("insert 1 {username} {email}"),
        "select".to_string(),
        ".exit".to_string(),
    ];

    let results = run_script_exec_with_defaults(scripts);

    result_match(
        results,
        vec![
            "csquarelite> String value for 'username' too long.",
            "csquarelite> Executed.",
            "csquarelite> ",
        ],
    );
}

#[test]
fn prints_an_error_message_if_id_is_negative() {
    let scripts = vec!["insert -1 cstack foo@bar.com", "select", ".exit"];
    let results = run_script_exec_with_defaults(scripts);

    result_match(
        results,
        vec![
            "csquarelite> Validation Error: Integer value for 'id' cannot be negative",
            "csquarelite> Executed.",
            "csquarelite> ",
        ],
    );
}

#[test]
fn keeps_data_after_closing_connection() {
    let db_filename = gen_random_filename();
    let scripts = vec!["insert 1 user1 person1@example.com", ".exit"];
    let results = run_script_exec(scripts, Some(db_filename.to_owned()), false);
    result_match(results, vec!["csquarelite> Executed.", "csquarelite> "]);

    let scripts = vec!["select", ".exit"];
    let results = run_script_exec(scripts, Some(db_filename), true);
    result_match(
        results,
        vec![
            "csquarelite> Row { id: 1, username: \"user1\", email: \"person1@example.com\" }",
            "Executed.",
            "csquarelite> ",
        ],
    );
}
