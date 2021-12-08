use std::fs;

pub fn read_input(file_name: &str) -> Vec<u32> {
    let file_string = fs::read_to_string(file_name)
        .expect("Failure: Cannot read file");

    let input: Vec<&str> = file_string
        .lines()
        .collect();

    input
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

pub fn count_increase(vec: &Vec<u32>) -> u32 {
    let mut inc_count = 0;
    let mut i = 1;
    let mut seed = vec[0];
    while i < vec.len() {
        if vec[i] > seed { inc_count = inc_count + 1; }
        seed = vec[i];
        i = i + 1;
    }

    inc_count    
}

pub fn count_3_window_inc(vec: &Vec<u32>) -> u32 {
    let mut inc_count = 0;
    let mut i = 2;

    let mut seed = vec[0] + vec[1] + vec[2];
    while i < vec.len() - 1 {
        if vec[i-1] + vec[i] + vec[i+1] > seed { inc_count = inc_count + 1; }
        seed = vec[i-1] + vec[i] + vec[i+1];
        i = i + 1;
    }

    inc_count    
}