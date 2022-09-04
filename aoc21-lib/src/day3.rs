pub fn calc_power_consumption(diagnostics: &Vec<String>) -> (u32, u32) {
    let mut i = 0;
    let mut most_common: String = String::from("");
    let mut least_common: String = String::from("");
    while i < diagnostics[0].len() {
        let mut one = 0;
        let mut zero = 0;
        for j in 0..diagnostics.len() {
            if diagnostics[j].chars().nth(i).unwrap() == '1' { one = one + 1; }
            else { zero = zero + 1; }
        }
        if one > zero {
            most_common.push_str("1");
            least_common.push_str("0");
        } else {
            most_common.push_str("0");
            least_common.push_str("1");
        }
        i = i + 1;
    }

    let gamma_bin = u32::from_str_radix(&most_common, 2).unwrap();
    let epsilon_bin = u32::from_str_radix(&least_common, 2).unwrap();

    (gamma_bin, epsilon_bin)
}

fn get_common_bits_in_positon<'x>(diagnostics: &Vec<String>, pos: usize) -> (&'x str, &'x str) {
    let mut one = 0;
    let mut zero = 0;
    for j in 0..diagnostics.len() {
        if diagnostics[j].chars().nth(pos).unwrap() == '1' { one = one + 1; }
        else { zero = zero + 1; }
    }

    let mut result: (&'x str, &'x str);

    if one >= zero { result = ("1", "0"); }
    else { result = ("0", "1"); }

    result
}

pub fn calc_life_support_rating(diagnostics: &mut Vec<String>) -> u32 {
    let mut i = 0;
    let mut oxygen_clone = diagnostics.to_owned();
    let mut co2_clone = diagnostics.to_owned();
    println!("Length of 1 number: {}", diagnostics[0].chars().count());
    while i < diagnostics[0].chars().count() && oxygen_clone.len() > 1 {
        //println!("Value of I: {}", i);
        //println!("Length of Oxygen: {}", oxygen_clone.len());
        //println!("Length of CO2: {}", co2_clone.len());
        let (o_most, o_least) = get_common_bits_in_positon(&oxygen_clone, i);
        oxygen_clone = oxygen_clone
                        .into_iter()
                        .filter(|num| num.chars().nth(i).unwrap() == o_most.chars().nth(0).unwrap())
                        .collect::<Vec<String>>();
        i = i + 1;
    }
    i = 0;
    while i < diagnostics[0].chars().count() && co2_clone.len() > 1 {
        //println!("Value of I: {}", i);
        //println!("Length of Oxygen: {}", oxygen_clone.len());
        //println!("Length of CO2: {}", co2_clone.len());

        let (c_most, c_least) = get_common_bits_in_positon(&co2_clone, i);
        co2_clone = co2_clone
                        .into_iter()
                        .filter(|num| num.chars().nth(i).unwrap() == c_least.chars().nth(0).unwrap())
                        .collect::<Vec<String>>();
        i = i + 1;
    }

    //println!("Final length of Oxygen: {}", oxygen_clone.len());
    //println!("Final length of CO2: {}", co2_clone.len());

    let oxygen = u32::from_str_radix(&oxygen_clone[0], 2).unwrap();
    let co2 = u32::from_str_radix(&co2_clone[0], 2).unwrap();

    oxygen * co2
}