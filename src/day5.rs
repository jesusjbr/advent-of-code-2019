enum Operation {
    Add,
    Multiply,
    Save,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Operation {
    fn from_u32(value: i32) -> Operation {
        match value {
            1 => Operation::Add,
            2 => Operation::Multiply,
            3 => Operation::Save,
            4 => Operation::Output,
            5 => Operation::JumpIfTrue,
            6 => Operation::JumpIfFalse,
            7 => Operation::LessThan,
            8 => Operation::Equals,
            99 => Operation::Halt,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from_u32(value: i32) -> ParameterMode {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

struct Instruction {
    operation: Operation,
    first_mode: ParameterMode,
    second_mode: ParameterMode,
    third_mode: ParameterMode,
}

impl Instruction {
    fn new(value: i32) -> Self {
        let mut inst = value;
        let operation = value % 100;
        inst = (inst - operation) / 100;
        let first_mode = if inst % 10 > 0 { 1 } else { 0 };
        let second_mode = if inst % 100 > 1 { 1 } else { 0 };
        let third_mode = if inst % 1000 > 11 { 1 } else { 0 };

        Instruction {
            operation: Operation::from_u32(operation),
            first_mode: ParameterMode::from_u32(first_mode),
            second_mode: ParameterMode::from_u32(second_mode),
            third_mode: ParameterMode::from_u32(third_mode),
        }
    }
}

struct Computer {
    int_code: Vec<i32>,
    current_position: usize,
    current_instruction: Instruction,
    finished: bool,
    input: i32,
    diagnostic_code: i32,
}

impl Computer {
    fn new(int_code: Vec<i32>, input: i32) -> Self {
        let instruction = int_code[0];
        Self {
            int_code,
            current_position: 0,
            current_instruction: Instruction::new(instruction),
            finished: false,
            input,
            diagnostic_code: 0,
        }
    }

    fn update_current_instruction(&mut self) {
        let instruction = self.int_code[self.current_position];
        self.current_instruction = Instruction::new(instruction);
    }

    fn operand1(&self) -> i32 {
        match self.current_instruction.first_mode {
            ParameterMode::Immediate => self.int_code[self.current_position + 1],
            ParameterMode::Position => {
                self.int_code[self.int_code[self.current_position + 1 as usize] as usize]
            }
        }
    }

    fn operand1_direct(&self) -> i32 {
        self.int_code[self.current_position + 1]
    }

    fn operand2(&self) -> i32 {
        match self.current_instruction.second_mode {
            ParameterMode::Immediate => self.int_code[self.current_position + 2],
            ParameterMode::Position => {
                self.int_code[self.int_code[self.current_position + 2 as usize] as usize]
            }
        }
    }

    fn set_result(&mut self, result: i32, offset: usize) {
        let result_position = self.int_code[self.current_position + offset];
        self.int_code[result_position as usize] = result;
    }

    fn execute_step(&mut self) {
        self.update_current_instruction();
        let mut step = 4;
        match self.current_instruction.operation {
            Operation::Add => {
                //Calculate the result
                self.set_result(self.operand1() + self.operand2(), step - 1);
                //Update current position
                self.current_position += step;
            }
            Operation::Multiply => {
                //Calculate the result
                self.set_result(self.operand1() * self.operand2(), step - 1);
                //Update current position
                self.current_position += step;
            }
            Operation::Save => {
                //Read input
                let operand1 = self.operand1_direct();
                self.int_code[operand1 as usize] = self.input;
                //Update current position
                step = 2;
                self.current_position += step;
            }
            Operation::Output => {
                println!("Output {}", self.operand1());
                self.diagnostic_code = self.operand1();
                //Update current position+
                step = 2;
                self.current_position += step;
            }
            Operation::JumpIfTrue => {
                //Update the current position
                step = 3;
                self.current_position = if self.operand1() != 0 {
                    self.operand2() as usize
                } else {
                    self.current_position + step
                }
            }
            Operation::JumpIfFalse => {
                //Update the current position
                step = 3;
                self.current_position = if self.operand1() == 0 {
                    self.operand2() as usize
                } else {
                    self.current_position + step
                }
            }
            Operation::LessThan => {
                if self.operand1() < self.operand2() {
                    self.set_result(1, step - 1);
                } else {
                    self.set_result(0, step - 1);
                }
                //Update the current position
                self.current_position += step;
            }
            Operation::Equals => {
                if self.operand1() == self.operand2() {
                    self.set_result(1, step - 1);
                } else {
                    self.set_result(0, step - 1);
                }
                //Update the current position
                self.current_position += step;
            }
            Operation::Halt => {
                self.finished = true;
            }
        }
    }

    fn run(&mut self) -> i32 {
        while !self.finished {
            self.execute_step();
        }

        self.int_code[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let int_code = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1002, 188, 27, 224, 1001, 224, -2241,
            224, 4, 224, 102, 8, 223, 223, 1001, 224, 6, 224, 1, 223, 224, 223, 101, 65, 153, 224,
            101, -108, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 1, 224, 1, 224, 223, 223, 1,
            158, 191, 224, 101, -113, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7, 224, 1,
            223, 224, 223, 1001, 195, 14, 224, 1001, 224, -81, 224, 4, 224, 1002, 223, 8, 223, 101,
            3, 224, 224, 1, 224, 223, 223, 1102, 47, 76, 225, 1102, 35, 69, 224, 101, -2415, 224,
            224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 224, 223, 223, 1101, 32, 38, 224,
            101, -70, 224, 224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 224, 223, 223, 1102,
            66, 13, 225, 1102, 43, 84, 225, 1101, 12, 62, 225, 1102, 30, 35, 225, 2, 149, 101, 224,
            101, -3102, 224, 224, 4, 224, 102, 8, 223, 223, 101, 4, 224, 224, 1, 223, 224, 223,
            1101, 76, 83, 225, 1102, 51, 51, 225, 1102, 67, 75, 225, 102, 42, 162, 224, 101, -1470,
            224, 224, 4, 224, 102, 8, 223, 223, 101, 1, 224, 224, 1, 223, 224, 223, 4, 223, 99, 0,
            0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1,
            99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265,
            1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1,
            99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1108,
            226, 677, 224, 1002, 223, 2, 223, 1005, 224, 329, 101, 1, 223, 223, 108, 226, 226, 224,
            1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1, 223, 1107, 677, 226, 224, 1002, 223,
            2, 223, 1006, 224, 359, 101, 1, 223, 223, 1008, 226, 226, 224, 1002, 223, 2, 223, 1005,
            224, 374, 101, 1, 223, 223, 8, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 389, 101, 1,
            223, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 404, 1001, 223, 1, 223, 7,
            226, 226, 224, 1002, 223, 2, 223, 1005, 224, 419, 101, 1, 223, 223, 107, 226, 677, 224,
            1002, 223, 2, 223, 1005, 224, 434, 101, 1, 223, 223, 107, 226, 226, 224, 1002, 223, 2,
            223, 1005, 224, 449, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1006,
            224, 464, 1001, 223, 1, 223, 1007, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 479,
            1001, 223, 1, 223, 1107, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 494, 101, 1, 223,
            223, 1108, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 509, 101, 1, 223, 223, 7, 677,
            226, 224, 1002, 223, 2, 223, 1005, 224, 524, 1001, 223, 1, 223, 1008, 677, 226, 224,
            102, 2, 223, 223, 1005, 224, 539, 1001, 223, 1, 223, 1108, 226, 226, 224, 102, 2, 223,
            223, 1005, 224, 554, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1006, 224,
            569, 1001, 223, 1, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 584, 101, 1,
            223, 223, 8, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 599, 1001, 223, 1, 223, 108,
            677, 677, 224, 1002, 223, 2, 223, 1005, 224, 614, 101, 1, 223, 223, 108, 226, 677, 224,
            102, 2, 223, 223, 1005, 224, 629, 101, 1, 223, 223, 8, 677, 226, 224, 102, 2, 223, 223,
            1006, 224, 644, 1001, 223, 1, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006, 224,
            659, 1001, 223, 1, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 674, 101, 1,
            223, 223, 4, 223, 99, 226,
        ];

        let mut computer = Computer::new(int_code.clone(), 1);
        computer.run();
        assert_eq!(computer.diagnostic_code, 13087969);
    }

    #[test]
    fn test_part2() {
        let int_code = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1002, 188, 27, 224, 1001, 224, -2241,
            224, 4, 224, 102, 8, 223, 223, 1001, 224, 6, 224, 1, 223, 224, 223, 101, 65, 153, 224,
            101, -108, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 1, 224, 1, 224, 223, 223, 1,
            158, 191, 224, 101, -113, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7, 224, 1,
            223, 224, 223, 1001, 195, 14, 224, 1001, 224, -81, 224, 4, 224, 1002, 223, 8, 223, 101,
            3, 224, 224, 1, 224, 223, 223, 1102, 47, 76, 225, 1102, 35, 69, 224, 101, -2415, 224,
            224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 224, 223, 223, 1101, 32, 38, 224,
            101, -70, 224, 224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 224, 223, 223, 1102,
            66, 13, 225, 1102, 43, 84, 225, 1101, 12, 62, 225, 1102, 30, 35, 225, 2, 149, 101, 224,
            101, -3102, 224, 224, 4, 224, 102, 8, 223, 223, 101, 4, 224, 224, 1, 223, 224, 223,
            1101, 76, 83, 225, 1102, 51, 51, 225, 1102, 67, 75, 225, 102, 42, 162, 224, 101, -1470,
            224, 224, 4, 224, 102, 8, 223, 223, 101, 1, 224, 224, 1, 223, 224, 223, 4, 223, 99, 0,
            0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1,
            99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265,
            1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1,
            99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 1108,
            226, 677, 224, 1002, 223, 2, 223, 1005, 224, 329, 101, 1, 223, 223, 108, 226, 226, 224,
            1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1, 223, 1107, 677, 226, 224, 1002, 223,
            2, 223, 1006, 224, 359, 101, 1, 223, 223, 1008, 226, 226, 224, 1002, 223, 2, 223, 1005,
            224, 374, 101, 1, 223, 223, 8, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 389, 101, 1,
            223, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 404, 1001, 223, 1, 223, 7,
            226, 226, 224, 1002, 223, 2, 223, 1005, 224, 419, 101, 1, 223, 223, 107, 226, 677, 224,
            1002, 223, 2, 223, 1005, 224, 434, 101, 1, 223, 223, 107, 226, 226, 224, 1002, 223, 2,
            223, 1005, 224, 449, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1006,
            224, 464, 1001, 223, 1, 223, 1007, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 479,
            1001, 223, 1, 223, 1107, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 494, 101, 1, 223,
            223, 1108, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 509, 101, 1, 223, 223, 7, 677,
            226, 224, 1002, 223, 2, 223, 1005, 224, 524, 1001, 223, 1, 223, 1008, 677, 226, 224,
            102, 2, 223, 223, 1005, 224, 539, 1001, 223, 1, 223, 1108, 226, 226, 224, 102, 2, 223,
            223, 1005, 224, 554, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1006, 224,
            569, 1001, 223, 1, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 584, 101, 1,
            223, 223, 8, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 599, 1001, 223, 1, 223, 108,
            677, 677, 224, 1002, 223, 2, 223, 1005, 224, 614, 101, 1, 223, 223, 108, 226, 677, 224,
            102, 2, 223, 223, 1005, 224, 629, 101, 1, 223, 223, 8, 677, 226, 224, 102, 2, 223, 223,
            1006, 224, 644, 1001, 223, 1, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006, 224,
            659, 1001, 223, 1, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 674, 101, 1,
            223, 223, 4, 223, 99, 226,
        ];

        let mut computer = Computer::new(int_code.clone(), 5);
        computer.run();
        assert_eq!(computer.diagnostic_code, 14110739);
    }
}
