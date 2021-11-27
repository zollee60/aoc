use std::fs;

pub fn read_input(file_name: &str) -> Vec<String> {
    let file_string = fs::read_to_string(file_name)
        .expect("Failure: Cannot read file");

    let input: Vec<String> = file_string
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();

    input
}

pub fn check_if_valid1(passport: &String) -> bool {
    passport.contains("byr") &&
    passport.contains("iyr") &&
    passport.contains("eyr") &&
    passport.contains("hgt") &&
    passport.contains("hcl") &&
    passport.contains("ecl") &&
    passport.contains("pid")
}

pub fn is_hex(c: char) -> bool {
    c == 'a' || c == 'b' || c == 'c' || c == 'd' || c == 'e' || c == 'f' 
}

pub fn check_if_valid2(passport: &String) -> bool {
    let tmp: Vec<&str> = passport.split_whitespace().collect();

    let byr_str = tmp.iter().find(|x| x.contains("byr")).unwrap();
    let byr_data: usize = byr_str.split(':').nth(1).unwrap().parse().unwrap();
    let byr_valid = byr_data > 1919 && byr_data < 2003;
    if !byr_valid { println!("byr is not valid: {}", byr_data); return false }

    let iyr_str = tmp.iter().find(|x| x.contains("iyr")).unwrap();
    let iyr_data: usize = iyr_str.split(':').nth(1).unwrap().parse().unwrap();
    let iyr_valid = iyr_data > 2009 && iyr_data < 2021;
    if !iyr_valid { println!("iyr is not valid: {}", iyr_data); return false }

    let eyr_str = tmp.iter().find(|x| x.contains("eyr")).unwrap();
    let eyr_data: usize = eyr_str.split(':').nth(1).unwrap().parse().unwrap();
    let eyr_valid = eyr_data > 2019 && eyr_data < 2031;
    if !eyr_valid { println!("eyr is not valid: {}", eyr_data); return false }

    let hgt_str = tmp.iter().find(|x| x.contains("hgt")).unwrap();
    let hgt_value = hgt_str.split(':').nth(1).unwrap();
    let hgt_data: usize =
    match hgt_value[..hgt_value.chars().count() - 2].parse::<usize>() {
        Ok(value) => value,
        Err(_err) => 0
    };
    let hgt_valid = (hgt_str.ends_with("in") && hgt_data > 58 && hgt_data < 77) || 
                    (hgt_str.ends_with("cm") && hgt_data > 149 && hgt_data < 194);
    if !hgt_valid { println!("hgt is not valid: {}", hgt_str); return false }

    let hcl_str = tmp.iter().find(|x| x.contains("hcl")).unwrap();
    let hcl_data = hcl_str.split(':').nth(1).unwrap();
    let hcl_valid = hcl_data.starts_with('#') && hcl_data.chars().count() == 7 && 
                    hcl_data[1..].chars().all(|x| x.is_numeric() || is_hex(x));
    if !hcl_valid { println!("hcl is not valid: {}", hcl_data); return false }

    let ecl_str = tmp.iter().find(|x| x.contains("ecl")).unwrap();
    let ecl_data = ecl_str.split(':').nth(1).unwrap();
    let ecl_valid = ecl_data == "amb" || ecl_data == "blu" || ecl_data == "brn" || ecl_data == "gry" ||
                    ecl_data == "hzl" || ecl_data == "oth" || ecl_data == "grn";
    if !ecl_valid { println!("ecl is not valid: {}", ecl_data); return false }

    let pid_str = tmp.iter().find(|x| x.contains("pid")).unwrap();
    let pid_data = pid_str.split(':').nth(1).unwrap();
    let pid_valid = pid_data.chars().count() == 9 && pid_data.chars().all(|x| x.is_numeric());
    if !pid_valid { println!("pid is not valid: {}", pid_data); return false }

    byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
    
}

pub fn count_valid(vec: &Vec<String>) -> usize {
    vec
    .iter()
    .filter(|x| check_if_valid1(&x) && check_if_valid2(&x))
    .count()
}