use aoc21_lib;
use aoc21_lib::day2::*;

fn main() {
    println!("Running Advent of Code 2021!");
    let vec: Vec<u32> = aoc21_lib::day1::read_input("./target/debug/day1.txt");
    //let stringVec: Vec<String> = vec.iter().map(|num| num.to_string()).collect();
    //let string: String = stringVec.join("\n");
    //println!("{}", string);
    println!("Day 1: Increase count = {}", aoc21_lib::day1::count_increase(&vec));
    println!("Day 1: Three-window increase count = {}", aoc21_lib::day1::count_3_window_inc(&vec));

    let mut subamrine = aoc21_lib::day2::SubMarine {
        depth: 0,
        horizontal: 0,
        aim: 0
    };

    let plan: Vec<String> = aoc21_lib::input::read_input_string("./target/debug/day2.txt");

    subamrine.execute_plan(&plan);

    println!("Day 2: Product of depth & horizaontal = {}", subamrine.depth * subamrine.horizontal);

    subamrine.reset();

    subamrine.execute_plan_v2(&plan);

    println!("Day 2: Product of depth & horizaontal (correct) = {}", subamrine.depth * subamrine.horizontal);

    let mut diagnostics: Vec<String> = aoc21_lib::input::read_input_string("./target/debug/day3.txt");

    let (gamma_bin, epsilon_bin) = aoc21_lib::day3::calc_power_consumption(&diagnostics);

    println!("Day 3: Power consumption = {}", gamma_bin * epsilon_bin);

    let life_support_rating = aoc21_lib::day3::calc_life_support_rating(&mut diagnostics);

    println!("Day 3: Life support rating = {}", life_support_rating);
}
