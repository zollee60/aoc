use aoc20_lib;

fn main() {

    // Puzzle 1
    let input: Vec<u32> = aoc20_lib::aoc1::read_input("input.txt");
    let (p1, p2) = aoc20_lib::aoc1::find_pair(&input);
    let (n1, n2, n3) = aoc20_lib::aoc1::find_three(&input);

    println!("-----Pairs-----\nP1: {}; P2: {}\nAnswer: {}\n---------------", p1, p2, p1*p2);
    println!("-----Three-----\nP1: {}; P2: {}, P3: {}\nAnswer: {}\n---------------", n1, n2, n3, n1*n2*n3);

    // Puzzle 2
    let input: Vec<String> = aoc20_lib::aoc2::read_input("input.txt");
    let c1 = aoc20_lib::aoc2::count_valid(&input, aoc20_lib::aoc2::check_if_valid);
    let c2 = aoc20_lib::aoc2::count_valid(&input, aoc20_lib::aoc2::check_if_valid2);

    println!("Number of valid inputs first: {}", c1);
    println!("Number of valid inputs second: {}", c2);

    // Puzzle 3
    let input: Vec<String> = aoc20_lib::aoc3::read_input("input.txt");
    let v = vec![(1,1),(3,1),(5,1),(7,1),(1,2)];
    let mut res = 1;
    for tup in v.iter() {
        res *= aoc20_lib::aoc3::count_trees(&input, tup.0, tup.1);
    }

    println!("Result: {}", res);

    // Puzzle 4
    let input: Vec<String> = aoc20_lib::aoc4::read_input("input.txt");
    let c = aoc20_lib::aoc4::count_valid(&input);
    println!("Valid passports: {}", c);

    // Puzzle 5
    let input: Vec<String> = aoc20_lib::aoc5::read_input("input.txt");
    //list_inputs(&input);
    let mut ids = aoc20_lib::aoc5::gen_id_vec(&input);
    ids.sort();
    let my_id = aoc20_lib::aoc5::get_my_seat_id(&ids);
    println!("My id: {}", my_id);

    // Puzzle 6
    let input = aoc20_lib::aoc6::read_input("input.txt");
    aoc20_lib::aoc6::count_group_sums2(&input);
}
