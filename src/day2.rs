fn noun_and_verb(mut int_code: Vec<usize>, output_searched: usize) -> i32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            int_code[2] = verb;
            int_code[1] = noun;
            let mut computer = Computer::new(int_code.clone());
            let result = computer.run();
            if result == output_searched {
                return (noun * 100 + verb) as i32;
            }
        }
    }
    return 0;
}

pub struct Computer {
    int_code: Vec<usize>,
    current_position: usize,
    finished: bool,
}

impl Computer {
    fn new(int_code: Vec<usize>) -> Self {
        Self {
            int_code,
            current_position: 0,
            finished: false,
        }
    }

    fn execute_step(&mut self) {
        match self.int_code[self.current_position] {
            1 => {
                //Calculate the positions
                let result = self.int_code[self.current_position + 3];
                let operand1 = self.int_code[self.current_position + 1];
                let operand2 = self.int_code[self.current_position + 2];
                //Do the sum
                self.int_code[result] = self.int_code[operand1] + self.int_code[operand2];
                //Update current position
                self.current_position += 4;
            }
            2 => {
                //Calculate the positions
                let result = self.int_code[self.current_position + 3];
                let operand1 = self.int_code[self.current_position + 1];
                let operand2 = self.int_code[self.current_position + 2];
                //Do  the mul
                self.int_code[result] = self.int_code[operand1] * self.int_code[operand2];
                //Update current position
                self.current_position += 4;
            }
            99 => {
                self.finished = true;
            }
            _ => unreachable!(),
        }
    }

    fn run(&mut self) -> usize {
        while !self.finished {
            self.execute_step();
        }

        self.int_code[0]
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day1_part1() {
        let int_code = vec![
            1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 19, 6, 23, 2, 23, 6,
            27, 2, 6, 27, 31, 2, 13, 31, 35, 1, 9, 35, 39, 2, 10, 39, 43, 1, 6, 43, 47, 1, 13, 47,
            51, 2, 6, 51, 55, 2, 55, 6, 59, 1, 59, 5, 63, 2, 9, 63, 67, 1, 5, 67, 71, 2, 10, 71,
            75, 1, 6, 75, 79, 1, 79, 5, 83, 2, 83, 10, 87, 1, 9, 87, 91, 1, 5, 91, 95, 1, 95, 6,
            99, 2, 10, 99, 103, 1, 5, 103, 107, 1, 107, 6, 111, 1, 5, 111, 115, 2, 115, 6, 119, 1,
            119, 6, 123, 1, 123, 10, 127, 1, 127, 13, 131, 1, 131, 2, 135, 1, 135, 5, 0, 99, 2, 14,
            0, 0,
        ];
        let mut computer = Computer::new(int_code);
        let result = computer.run();
        assert_eq!(result, 3224742);
    }

    #[test]
    fn day1_part2() {
        let int_code = vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 19, 6, 23, 2, 23, 6,
            27, 2, 6, 27, 31, 2, 13, 31, 35, 1, 9, 35, 39, 2, 10, 39, 43, 1, 6, 43, 47, 1, 13, 47,
            51, 2, 6, 51, 55, 2, 55, 6, 59, 1, 59, 5, 63, 2, 9, 63, 67, 1, 5, 67, 71, 2, 10, 71,
            75, 1, 6, 75, 79, 1, 79, 5, 83, 2, 83, 10, 87, 1, 9, 87, 91, 1, 5, 91, 95, 1, 95, 6,
            99, 2, 10, 99, 103, 1, 5, 103, 107, 1, 107, 6, 111, 1, 5, 111, 115, 2, 115, 6, 119, 1,
            119, 6, 123, 1, 123, 10, 127, 1, 127, 13, 131, 1, 131, 2, 135, 1, 135, 5, 0, 99, 2, 14,
            0, 0,
        ];
        assert_eq!(noun_and_verb(int_code.clone(), 19690720), 7960);
    }
}
