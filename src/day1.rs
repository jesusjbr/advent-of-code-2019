use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn simple_fuel_requirements() -> i32 {
    let file = File::open("./input/day1.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|mass| simple_fuel(mass.unwrap().parse::<i32>().unwrap()))
        .sum()
}

fn complex_fuel_requirements() -> i32 {
    let file = File::open("./input/day1.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|mass| fuel_needed(mass.unwrap().parse::<i32>().unwrap()))
        .sum()
}

fn simple_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_needed(mass: i32) -> i32 {
    let mut total: i32 = 0;
    let mut fuel: i32;
    fuel = mass / 3 - 2;
    while fuel > 0 {
        total += fuel;
        fuel = fuel / 3 - 2;
    }
    total
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day1_part1() {
        assert_eq!(simple_fuel(12), 2);
        assert_eq!(simple_fuel(14), 2);
        assert_eq!(simple_fuel(1969), 654);
        assert_eq!(simple_fuel(100756), 33583);
        assert_eq!(simple_fuel_requirements(), 3126794);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(fuel_needed(14), 2);
        assert_eq!(fuel_needed(1969), 966);
        assert_eq!(fuel_needed(100756), 50346);
        assert_eq!(complex_fuel_requirements(), 4687331);
    }
}
