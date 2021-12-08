pub struct BingoNumber {
    pub num: u32,
    pub marked: bool
}

pub struct Bingo {
    pub numbers: Vec<u32>,
    pub table: [[BingoNumber;5];5]
}

pub fn read_bingo(file_name: &str) -> Bingo {
    let file_string = fs::read_to_string(file_name)
        .expect("Failure: Cannot read file");

    let input: Vec<String> = file_string
        .lines()
        .map(|x| x.to_string())
        .collect();
}