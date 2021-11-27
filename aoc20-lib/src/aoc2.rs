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

pub fn check_if_valid(line: &String) -> bool {
    let tmp: Vec<&str> = line.split(' ').collect();
    let policy: Vec<&str> = tmp[0].split('-').collect();
    let min: u32 = policy.iter().nth(0).unwrap().parse().unwrap();
    let max: u32 = policy.iter().nth(1).unwrap().parse().unwrap();
    let ch = tmp[1].chars().nth(0).unwrap();
    let pw = tmp[2];

    pw.matches(ch).count() >= min as usize && pw.matches(ch).count() <= max as usize
}

pub fn check_if_valid2(line: &String) -> bool {
    let tmp: Vec<&str> = line.split(' ').collect();
    let policy: Vec<&str> = tmp[0].split('-').collect();
    let pos1: usize = policy.iter().nth(0).unwrap().parse().unwrap();
    let pos2: usize = policy.iter().nth(1).unwrap().parse().unwrap();
    let ch = tmp[1].chars().nth(0).unwrap() as u8;
    let pw = tmp[2];
    let stat1 = pw.as_bytes()[pos1-1] == ch;
    let stat2 = pw.as_bytes()[pos2-1] == ch;

    stat1 ^ stat2
}

pub fn count_valid(vec: &Vec<String>, f: fn(&String) -> bool) -> usize{
    vec
    .iter()
    .filter(|x| f(x))
    .count()
}