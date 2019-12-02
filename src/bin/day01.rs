use std::io::{self, BufRead};

fn fuel_calc(x: i64) -> i64 {
    x / 3 - 2
}

fn recursive_fuel_calc(x: i64) -> i64 {
    let fuel = fuel_calc(x);
    if fuel > 0 {
        fuel + recursive_fuel_calc(fuel)
    } else {
        0
    }
}

fn main() {
    let stdin = io::stdin();

    let result: i64 = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        //.map(fuel_calc)
        .map(recursive_fuel_calc)
        .sum();

    println!("{}", result);
}
