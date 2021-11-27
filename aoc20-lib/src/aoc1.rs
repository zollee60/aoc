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

pub fn find_pair(vec: &Vec<u32>) -> (u32, u32) {
    let (mut p1, mut p2, mut i, mut j) = (0,0,0,0);

    while i < vec.len() && p1+p2 != 2020{
        p1 = vec[i];
        j = i + 1; 
        while j < vec.len() && p1+p2 != 2020{
            p2 = vec[j];
            j += 1;
        }
        i += 1;
    }

    (p1,p2)
}

pub fn find_three(vec: &Vec<u32>) -> (u64, u64, u64) {
    let (mut i, mut j, mut k);
    let mut p1: u64 = 0;
    let mut p2: u64 = 0;
    let mut p3: u64 = 0;
    i = 0;
    while i < vec.len() && p1+p2+p3 != 2020{
        p1 = vec[i].into();
        j = i + 1;
        while j < vec.len() && p1+p2+p3 != 2020{
            p2 = vec[j].into();
            k = j + 1;
            while k < vec.len() && p1+p2+p3 != 2020{
                p3 = vec[k].into();
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    (p1,p2,p3)
}