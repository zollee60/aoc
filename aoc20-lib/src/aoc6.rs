use std::fs;
use std::collections::HashMap;

pub fn read_input(file_name: &str) -> Vec<String>{
    let file_string = fs::read_to_string(file_name)
        .expect("Failure: Cannot read file");

    let input: Vec<String> = file_string
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    input
}

pub fn get_distinct_letters2(line: &String, distinct: &mut HashMap<String, i32>){
    for c in line.chars(){
        print!("{}",c);
        if distinct.contains_key(&(&c).to_string()) {
            let newValue = distinct.get(&(&c).to_string()).unwrap() + 1;
            distinct.insert((&c).to_string(), newValue);
        } else {
            distinct.insert((&c).to_string(), 1);
        }
    }
    println!();
}

pub fn count_group_sums2(array: &Vec<String>){
    let mut distinct: HashMap<String, i32> = HashMap::new();
    let mut sum = 0;
    let mut lineCount = 0;

    for line in array.iter(){
        match line.len() {
            0 => {
                for (key, val) in distinct.iter(){
                    if val == &lineCount { sum = sum + 1 }
                }
                println!("The distinct is : {:?}; The sum is: {}", distinct, sum);
                distinct.clear();
                lineCount = 0;
            }
            _ => {
                lineCount = lineCount + 1;
                get_distinct_letters2(line, &mut distinct)
            }
        }
    }
    println!("The answer is: {}", sum);
}