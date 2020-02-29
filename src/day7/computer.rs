pub mod computer {
    #[derive(PartialEq, Eq)]
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

    pub struct Computer {
        int_code: Vec<i32>,
        current_position: usize,
        current_instruction: Instruction,
        pub finished: bool,
        input: Vec<i32>,
        pub diagnostic_code: i32,
    }

    impl Computer {
        pub fn new(int_code: Vec<i32>, input: Vec<i32>) -> Self {
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
                    self.int_code[operand1 as usize] = self.input.remove(0);
                    //Update current position
                    step = 2;
                    self.current_position += step;
                }
                Operation::Output => {
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

        pub fn run(&mut self) -> i32 {
            while !self.finished {
                self.execute_step();
            }

            self.int_code[0]
        }

        pub fn run_until_output(&mut self) {
            self.execute_step();
            while self.current_instruction.operation != Operation::Output {
                self.execute_step();
            }
        }
    }
}
