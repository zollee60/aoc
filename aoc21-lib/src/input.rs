use std::fs;

pub fn read_input_string(file_name: &str) -> Vec<String> {
    let file_string = fs::read_to_string(file_name)
        .expect("Failure: Cannot read file");

    let input: Vec<String> = file_string
        .lines()
        .map(|x| x.to_string())
        .collect();

    input
}