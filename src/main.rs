use navigation::{find_basement, read_directions};
use presents::{calculate_gift_wrapping, calculate_ribbon};
use santa::Santa;
use util::{open_file, read_lines};

mod navigation;
mod presents;
mod santa;
mod util;

fn main() {
    day1();

    day2();

    day3();
}

fn day1() {
    println!("##### DAY 1 #####");

    let file_path_1 = "input1.txt";
    let directions = open_file(file_path_1);
    day1_1(&directions);
    day1_2(&directions);

    println!(" ");
}

fn day1_1(directions: &str) {
    let floor = read_directions(directions);
    println!("Santa ends on floor {floor}");
}

fn day1_2(directions: &str) {
    let basement = find_basement(directions);
    println!("Santa first enters the basement after {basement} steps");
}

fn day2() {
    println!("##### DAY 2 #####");
    let file_path_2 = "input2.txt";
    let presents = read_lines(file_path_2);

    day2_1(&presents);
    day2_2(&presents);
    println!(" ");
}

fn day2_1(presents: &Vec<String>) {
    let mut total_wrapping_paper = 0;
    for p in presents {
        let splits: Vec<&str> = p.split('x').collect();
        let w = splits[0].parse::<i32>().unwrap();
        let l = splits[1].parse::<i32>().unwrap();
        let h = splits[2].parse::<i32>().unwrap();
        total_wrapping_paper += calculate_gift_wrapping(w, l, h);
    }
    println!("{total_wrapping_paper} feet of wrapping paper used");
}

fn day2_2(presents: &Vec<String>) {
    let mut total_ribbon = 0;
    for p in presents {
        let splits: Vec<&str> = p.split('x').collect();
        let w = splits[0].parse::<i32>().unwrap();
        let l = splits[1].parse::<i32>().unwrap();
        let h = splits[2].parse::<i32>().unwrap();
        total_ribbon += calculate_ribbon(w, l, h);
    }
    println!("{total_ribbon} feet of ribbon used");
}

fn day3() {
    println!("##### DAY 3 #####");

    day3_1();
}

fn day3_1() {
    let mut santa = Santa::new();
    let directions = open_file("input3.txt");
    santa.move_santa(&directions);
    let visits = santa.visits();

    println!("Santa visits {visits} houses");
}