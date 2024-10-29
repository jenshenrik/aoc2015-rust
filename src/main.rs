use adventcoin::mine;
use list::{better_is_nice, is_nice};
use navigation::{find_basement, read_directions};
use presents::{calculate_gift_wrapping, calculate_ribbon, unwrap_present};
use santa::{collaborate, get_unique_visits_combined, Santa};
use util::{open_file, read_lines};

mod adventcoin;
mod list;
mod navigation;
mod presents;
mod santa;
mod util;

fn main() {
    day1();

    day2();

    day3();

    day4();

    day5();
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
    println!("1: {floor}");
}

fn day1_2(directions: &str) {
    let basement = find_basement(directions);
    println!("2: {basement}");
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
        let (w, l, h) = unwrap_present(p);
        total_wrapping_paper += calculate_gift_wrapping(w, l, h);
    }
    println!("1: {total_wrapping_paper}");
}

fn day2_2(presents: &Vec<String>) {
    let mut total_ribbon = 0;
    for p in presents {
        let (w, l, h) = unwrap_present(p);
        total_ribbon += calculate_ribbon(w, l, h);
    }
    println!("2: {total_ribbon}");
}

fn day3() {
    let directions = open_file("input3.txt");

    println!("##### DAY 3 #####");

    day3_1(&directions);
    day3_2(&directions);

    println!(" ");
}

fn day3_1(directions: &str) {
    let mut santa = Santa::new();
    santa.move_santa(directions);
    let visits = santa.visits();

    println!("1: {visits}");
}

fn day3_2(directions: &str) {
    let mut santa = Santa::new();
    let mut robo_santa = Santa::new();

    collaborate(&mut santa, &mut robo_santa, directions);
    let unique_visits = get_unique_visits_combined(&santa, &robo_santa);
    println!("2: {unique_visits}");
}

fn day4() {
    println!("##### DAY 4 #####");

    day4_1();
    day4_2();

    println!(" ");
}

fn day4_1() {
    let coin = mine("iwrupvqb", "00000");

    println!("1: {coin}");
}

fn day4_2() {
    let coin = mine("iwrupvqb", "000000");

    println!("2: {coin}");
}

fn day5() {
    let list = read_lines("input5.txt");

    println!("##### DAY 5 #####");

    day5_1(&list);
    day5_2(&list);

    println!(" ");
}

fn day5_1(list: &Vec<String>) {
    let mut count = 0;

    for line in list {
        if is_nice(line) {
            count += 1;
        }
    }

    println!("1. {count}");
}

fn day5_2(list: &Vec<String>) {
    let mut count = 0;

    for line in list {
        if better_is_nice(line) {
            count += 1;
        }
    }

    println!("2. {count}");
}
