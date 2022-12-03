#![allow(dead_code)]

mod day1;
mod day2;
mod day3;

fn main() {
    day_1();
    day_2();
    day_3();
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
