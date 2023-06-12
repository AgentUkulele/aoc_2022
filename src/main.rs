#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    // day_1();
    // day_2();
    // day_3();
    // day_4();
    day_5();
}

fn day_1() {
    let day1_1 = day1::day1_1();
    println!("{day1_1}");

    let day1_2 = day1::day1_2();
    println!("{day1_2}");
}

fn day_2() {
    let day2_1 = day2::day2_1();
    println!("{day2_1}");

    let day2_2 = day2::day2_2();
    println!("{day2_2}");
}

fn day_3() {
    let day3_1 = day3::day3_1();
    println!("{day3_1}");

    let day3_2 = day3::day3_2();
    println!("{day3_2}");
}

fn day_4() {
    let day4_1 = day4::day4_1();
    println!("{day4_1}");

    let day4_2 = day4::day4_2();
    println!("{day4_2}");
}

fn day_5() {
    let day5_1 = day5::day5_1();
    println!("{day5_1}");
}
