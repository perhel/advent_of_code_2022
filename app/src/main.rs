use input_loader::Load;
use models::Input;
use y_2022::{day_2_p_1, day_2_p_2};

fn main() {
    
    let input_f = Input::load("inputs/2022/day_2.txt");

    let score: u32 = day_2_p_1(&input_f);
    println!("{}", score);

    let score: u32 = day_2_p_2(&input_f);
    println!("{}", score);

    // let input: Vec<Vec<u32>> = input_f.to_str_vec("\\n")
    //     .into_iter()
    //     .map(|s| {
    //         s.split(char)
    //         .collect::<Vec<&str>>()
    //         .iter()
    //         .filter(|s| s.len() > 0)
    //         .map(|s| {
    //             FromStr::from_str(s.trim()).unwrap()
    //         })
    //         .collect::<Vec<u32>>()
    //     })
    //     .collect();

    // print!("Part one: ");
    // part_one(&input);
    // print!("Part two: ");
    // part_two(&input);
}

fn part_one(input: &Vec<Vec<u32>>) {
    let elfes: Vec<u32> = input.iter().map(|arr| arr.iter().sum()).collect();
    match elfes.iter().max() {
        Some(max) => println!("{:?}", max),
        None => println!("U f:ed up...")
    }
}

fn part_two(input: &Vec<Vec<u32>>) {
    let mut elfes: Vec<u32> = input.iter()
        .map(|arr| arr.iter().sum())
        .collect();

    elfes.sort_by(|a, b| b.cmp(a));

    match elfes.chunks(3)
    .nth(0) {
        Some(arr) => println!("{}", arr.iter().sum::<u32>()),
        None => println!("Somethings wrong...")
    };
}
