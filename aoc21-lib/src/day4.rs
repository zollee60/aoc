use std::fs;
#[derive(Clone)]
pub struct BingoNumber {
    pub num: u32,
    pub marked: bool
}

impl Default for BingoNumber {
    fn default () -> BingoNumber {
        BingoNumber{num: 0, marked: false}
    }
}

pub struct Bingo {
    pub numbers: Vec<u32>,
    pub tables: Vec<Vec<BingoNumber>> // Flattened
}

pub fn read_bingo(file_name: &str) -> Bingo {
    let file_string = fs::read_to_string(file_name)
        .expect("Failure: Cannot read file");

    let mut input: Vec<String> = file_string
        .lines()
        .map(|x| x.to_string())
        .collect();

    let chosen_num_string_unwrapped = input.remove(0);
    let chosen_num_strings: Vec<&str> = chosen_num_string_unwrapped.split(',').collect();
    let numbers = chosen_num_strings.iter().map(|nmr_str| nmr_str.trim().parse::<u32>().unwrap()).collect();

    let mut i = 0;

    let mut tables: Vec<BingoNumber> = Vec::new();

    //input.pop();

    while i < input.len() {
        if input[i] != "" {
            let number_strings: Vec<&str> = input[i].split(' ').collect();
            let numbers: Vec<BingoNumber> = number_strings.iter().filter(|str| **str != "").map(|nmr_str| BingoNumber { num: nmr_str.trim().parse::<u32>().unwrap(), marked: false } ).collect();
            tables.extend(numbers)
        }
        i += 1;
    }

    Bingo {
        numbers: numbers,
        tables: tables.chunks_exact(25).map(|chunk| chunk.to_vec()).collect()
    }
}

pub fn find_winner(bingo: &mut Bingo) -> u128 {
    draw_numbers(bingo)
}

fn draw_numbers(bingo: &mut Bingo) -> u128 {
    let mut i = 0;
    let mut result: u128 = 0;
    let mut final_bingo_found = false;
    let mut bingo_counter = 0;
    let num_of_tables = bingo.tables.len();
    while i < bingo.numbers.len() && !final_bingo_found {
        let called_number = bingo.numbers[i];
        for bingo_table in bingo.tables.iter_mut() {
            bingo_table.iter_mut().for_each(|bingo_number| {
                if bingo_number.num == called_number {
                    bingo_number.marked = true;
                }
            })
        }
        let mut j = 0;
        while j < bingo.tables.len() && !final_bingo_found {
            let is_bingo = check_bingo(&bingo.tables[j]);
            let mut current_bingo_table: Option<Vec<BingoNumber>> = None;
            if is_bingo {
                bingo_counter += 1;
                current_bingo_table = Some(bingo.tables[j].to_owned());
                 bingo.tables.remove(j);
            }
            if is_bingo && bingo_counter == num_of_tables && !Option::is_none(&current_bingo_table) {
                for num in current_bingo_table.unwrap() {
                    if !num.marked {
                        result += num.num as u128;
                    }
                }
                result *= called_number as u128;
                final_bingo_found = true;
            }
            j += 1;
        }
        i += 1;
    }
    result
}

fn check_bingo(slice: &[BingoNumber]) -> bool {
    for n in 0..5 {
        let bingo_row = slice[0+(n*5)].marked && slice[1+(n*5)].marked && slice[2+(n*5)].marked && slice[3+(n*5)].marked && slice[4+(n*5)].marked;
        let bingo_col = slice[0+n].marked && slice[1*5+n].marked && slice[2*5+n].marked && slice[3*5+n].marked && slice[4*5+n].marked;
        if bingo_row || bingo_col { return true }
    }
    false
}