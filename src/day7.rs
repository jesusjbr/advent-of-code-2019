mod computer;
use computer::computer::Computer;
use itertools::Itertools;
use std::collections::HashMap;

//Collect a vector of i32 and generates the corresponding number
fn permutation_to_number(permutation: Vec<i32>) -> i32 {
    let mut number = 0;
    for i in 0..permutation.len() {
        number += (10 as i32).pow(4 - i as u32) * permutation[i];
    }
    number
}

//Returns the maximum thruster signal reached trying every setting sequence.
fn max_thruster_signal(int_code: Vec<i32>) -> i32 {
    let mut input: Vec<i32>;
    let mut output: i32;
    let mut setting: i32;
    let mut thruster: Computer;
    let mut results: HashMap<i32, i32> = HashMap::with_capacity(5);
    let permutations: Vec<_> = (0..=4).permutations(5).collect();
    for perm in permutations {
        output = 0;
        for thruster_number in 0..=4 {
            input = vec![perm[thruster_number], output];
            thruster = Computer::new(int_code.clone(), input);
            thruster.run();
            output = thruster.diagnostic_code;
        }
        setting = permutation_to_number(perm);
        results.insert(setting, output);
    }
    let max = results.iter().max_by_key(|(_, v)| *v).unwrap().1;
    *max
}

//Returns the maximum thruster signal reached trying every setting sequence.
fn feedback_loop_mode(int_code: Vec<i32>) -> i32 {
    let mut input: Vec<i32>;
    let mut output: i32;
    let mut setting: i32;
    let mut thruster;
    let mut finished;
    let mut results: HashMap<i32, i32> = HashMap::with_capacity(5);
    let mut amplifiers: HashMap<i32, Computer>;
    let permutations: Vec<_> = (5..=9).permutations(5).collect();
    for perm in permutations {
        output = 0;
        amplifiers = HashMap::with_capacity(5);
        finished = false;
        for thruster_number in 0..=4 {
            input = vec![perm[thruster_number], output];
            thruster = amplifiers
                .entry(thruster_number as i32)
                .or_insert(Computer::new(int_code.clone(), input));
            thruster.run_until_output();
            output = thruster.diagnostic_code;
        }
        setting = permutation_to_number(perm);
        results.insert(setting, output);
    }
    let max = results.iter().max_by_key(|(_, v)| *v).unwrap().1;
    *max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_part1() {
        let int_code = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(dbg!(max_thruster_signal(int_code)), 43210);
    }

    #[test]
    fn example2_part1() {
        let int_code = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(dbg!(max_thruster_signal(int_code)), 54321);
    }

    #[test]
    fn example3_part1() {
        let int_code = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(dbg!(max_thruster_signal(int_code)), 65210);
    }

    #[test]
    fn test_part1() {
        let int_code = vec![
            3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 30, 47, 64, 81, 98, 179, 260, 341, 422, 99999,
            3, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 1002, 9, 5, 9, 101, 4, 9, 9, 102, 2, 9, 9, 4, 9,
            99, 3, 9, 102, 3, 9, 9, 101, 2, 9, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9, 1001, 9, 5, 9,
            1002, 9, 3, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 1002, 9, 3, 9, 101, 2, 9, 9, 102, 5, 9,
            9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9,
            4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9,
            3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
            1001, 9, 2, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
            1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002,
            9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9,
            4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4,
            9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
            1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2,
            9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1,
            9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9,
            3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
            1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9,
            101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1,
            9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9,
            4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99,
        ];
        assert_eq!(dbg!(max_thruster_signal(int_code)), 45730);
    }
}
