use std::fs;

pub fn read_input(file_name: &str) -> Vec<String> {
    let file_string = fs::read_to_string(file_name)
        .expect("Failure: Cannot read file");

    let input: Vec<String> = file_string
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    input
}

pub fn calculate_row(row_spec: &str) -> usize {
    let mut high = 127;
    let mut low = 0;
    for c in row_spec.chars(){
        if c == 'F' { 
            high = (high + low + 1) / 2 - 1;
        }
        else if c == 'B' { 
            low = (high + low + 1) / 2;
        }
    }

    if row_spec.chars().last().unwrap() == 'F' { high } else { low }
}

pub fn calculate_col(col_spec: &str) -> usize {
    let mut high = 7;
    let mut low = 0;

    for c in col_spec.chars(){
        if c == 'R' { 
            low = (high + low + 1) / 2;
        }
        else if c == 'L' { 
            high = (high + low + 1) / 2 - 1;
        }
    }

    if col_spec.chars().last().unwrap() == 'L' { high } else { low }
}

pub fn calculate_id(row: usize, col: usize) -> usize {
    row * 8 + col
}

pub fn gen_id_vec(vec: &Vec<String>) -> Vec<usize> {
    let mut ids: Vec<usize> = Vec::new();

    for s in vec.iter() {
        let row = calculate_row(&s[..7]);
        let col = calculate_col(&s[7..10]);
        let id = calculate_id(row,col);
        //println!("{}",&id);
        ids.push(id);
    }

    ids
}

pub fn get_my_seat_id(vec: &Vec<usize>) -> usize {
    let mut i = 0;
    while i < vec.len()-1 && vec[i+1] - vec[i] != 2 {
        i += 1;
    }
    if i < vec.len()-1 { vec[i]+1 } else { 0 }
}

pub fn list_inputs(vec: &Vec<String>){
    for s in vec.iter() {
        println!("{}",&s);
    }
}