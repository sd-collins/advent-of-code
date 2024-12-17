use itertools::Itertools;
use util::PerfTimer;

fn input() -> (u128, u128, u128, Vec<u8>) {
    let raw = util::get_day_input(17);
    let mut lines = raw.lines();
    let a_line = lines.next().unwrap();
    let a = a_line
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();

    let b_line = lines.next().unwrap();
    let b = b_line
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();

    let c_line = lines.next().unwrap();
    let c = c_line
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();

    let program = lines
        .nth(1)
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    (a, b, c, program)
}

struct State<'a> {
    program: &'a [u8],
    pc: usize,
    a: u128,
    b: u128,
    c: u128,
}

enum ExecResult {
    Continue,
    Halt,
    Output(u128),
}

impl State<'_> {
    fn combo_operand(&self, combo: u8) -> u128 {
        match combo {
            v @ 0..=3 => u128::from(v),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            o => panic!("Invalid combo operand {o:?}"),
        }
    }

    fn execute_next_instruction(&mut self) -> ExecResult {
        if self.pc >= self.program.len() {
            return ExecResult::Halt;
        }
        let instruction = self.program[self.pc];
        let operand = self.program[self.pc + 1];

        match instruction {
            0 => {
                // adv <combo>
                let operand = self.combo_operand(operand);
                self.a >>= operand;
                self.pc += 2;
                ExecResult::Continue
            }
            1 => {
                // bxl <literal>
                self.b ^= u128::from(operand);
                self.pc += 2;
                ExecResult::Continue
            }
            2 => {
                // bst <combo>
                let operand = self.combo_operand(operand);
                self.b = operand % 8;
                self.pc += 2;
                ExecResult::Continue
            }
            3 => {
                // jnz <literal>
                if self.a != 0 {
                    self.pc = usize::from(operand);
                    ExecResult::Continue
                } else {
                    self.pc += 2;
                    ExecResult::Continue
                }
            }
            4 => {
                // bxc <ignored>
                self.b ^= self.c;
                self.pc += 2;
                ExecResult::Continue
            }
            5 => {
                // out <combo>
                let operand = self.combo_operand(operand);
                let output = operand % 8;
                self.pc += 2;
                ExecResult::Output(output)
            }
            6 => {
                // bdv <combo>
                let operand = self.combo_operand(operand);
                self.b = self.a >> operand;
                self.pc += 2;
                ExecResult::Continue
            }
            7 => {
                // cdv <combo>
                let operand = self.combo_operand(operand);
                self.c = self.a >> operand;
                self.pc += 2;
                ExecResult::Continue
            }
            o => {
                panic!("Invalid instruction {o:?}");
            }
        }
    }
}

#[allow(clippy::too_many_lines)]
fn main() {
    let (a, b, c, program) = input();

    {
        let _timer = PerfTimer::new("Part 1");
        let mut state = State {
            program: &program,
            pc: 0,
            a,
            b,
            c,
        };

        let mut output = Vec::new();

        loop {
            match state.execute_next_instruction() {
                ExecResult::Continue => {}
                ExecResult::Halt => break,
                ExecResult::Output(x) => output.push(x),
            }
        }

        let part_1: String = output.into_iter().join(",");
        println!("Part 1: {part_1}");
    }

    {
        // Print the program for debugging
        for (i, (opcode, raw_operand)) in program.iter().tuples().enumerate() {
            let instruction = match opcode {
                0 => "adv",
                1 => "bxl",
                2 => "bst",
                3 => "jnz",
                4 => "bxc",
                5 => "out",
                6 => "bdv",
                7 => "cdv",
                _ => panic!("Invalid instruction {opcode:?}"),
            };

            let operand = match opcode {
                0 | 2 | 5 | 6 | 7 => {
                    // combo operand
                    match raw_operand {
                        0..=3 => raw_operand.to_string(),
                        4 => String::from("a"),
                        5 => String::from("b"),
                        6 => String::from("c"),
                        _ => panic!("Invalid combo operand {raw_operand:?}"),
                    }
                }
                _ => raw_operand.to_string(),
            };

            println!("{i:02}: {instruction} {operand}");
        }
    }

    {
        let _timer = PerfTimer::new("Part 2");

        let mut current_possible_a_values = vec![0];

        for value in program.iter().copied().rev() {
            let mut new_possible_a_values = Vec::new();
            for current_a in current_possible_a_values {
                for possible_a_part in 0..8 {
                    let possible_a = (current_a << 3) + possible_a_part;
                    let mut state = State {
                        program: &program,
                        pc: 0,
                        a: possible_a,
                        b: 0,
                        c: 0,
                    };

                    let output = loop {
                        match state.execute_next_instruction() {
                            ExecResult::Continue => {}
                            ExecResult::Halt => panic!("Program halted unexpectedly before output"),
                            ExecResult::Output(x) => {
                                break x;
                            }
                        }
                    };
                    if output == u128::from(value) {
                        new_possible_a_values.push(possible_a);
                    }
                }
            }
            current_possible_a_values = new_possible_a_values;
        }

        let part_2 = *current_possible_a_values.iter().min().unwrap();
        println!("Part 2: {part_2}");

        // Check accuracy

        let mut state = State {
            program: &program,
            pc: 0,
            a: part_2,
            b,
            c,
        };

        let mut output = Vec::new();

        loop {
            match state.execute_next_instruction() {
                ExecResult::Continue => {}
                ExecResult::Halt => break,
                ExecResult::Output(x) => {
                    output.push(x);
                    if output.len() > program.len()
                        || output[output.len() - 1] != u128::from(program[output.len() - 1])
                    {
                        panic!(
                            "Output does not match program at index {}",
                            output.len() - 1
                        );
                    }
                }
            }
        }

        assert!(output
            .iter()
            .copied()
            .eq(program.iter().copied().map(u128::from)));
        println!("Output matches program: {output:?} == {program:?}");
    }
}
