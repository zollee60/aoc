use std::fs;

pub fn read_input(file_name: &str) -> Vec<String> {
    let file_string = fs::read_to_string(file_name)
        .expect("Failure: Cannot read file");

    let input: Vec<String> = file_string
        .lines()
        .map(|x| x.to_string())
        .collect();

    input
}

pub fn count_trees(vec: &Vec<String>, right: usize, down: usize) -> u32 {
    let mut x = 0;
    let mut level = 1;
    let mut count = 0;
    for line in vec.iter().step_by(down) {
        if level > 1 {
            x += right;
            if x >= line.chars().count() { x = x - line.chars().count() }
            if line.chars().nth(x).unwrap() == '#' { count += 1; }
        }
        level += 1;
    }

    count
}