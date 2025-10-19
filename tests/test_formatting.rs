use std::fs;

use clogs::models::log_line::LogLine;

#[test]
fn test_single_clog() {
    let input = fs::read_to_string("tests/input/test.log").expect("Unable to read file");
    let result = input
        .lines()
        .map(|line| {
            serde_json::from_str::<LogLine>(&line)
                .unwrap()
                .to_single_clog()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", result);
    insta::assert_yaml_snapshot!(result);
}

#[test]
fn test_multi_clog() {
    let input = fs::read_to_string("tests/input/test.log").expect("Unable to read file");
    let result = input
        .lines()
        .map(|line| {
            serde_json::from_str::<LogLine>(&line)
                .unwrap()
                .to_multi_clog()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", result);
    insta::assert_yaml_snapshot!(result);
}
