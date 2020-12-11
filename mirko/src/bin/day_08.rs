use std::collections::HashSet;

fn main() {
    let mut code: BootCode = include_str!("../../inputs/08.txt").parse().unwrap();

    let (part_1, _) = code.run_code();
    let part_2 = code.fix_code();

    println!("part 1: {}", part_1);
    println!("part 2: {}", part_2);
}

#[derive(Debug)]
enum Instruction {
    Accumulate(i16),
    Jump(i16),
    NoOperation(i16),
}

enum TerminationCode {
    Success,
    InfiniteLoopAborted,
}

#[derive(Debug)]
struct BootCode {
    instructions: Vec<Instruction>,
}

impl BootCode {
    fn run_code(&self) -> (i16, TerminationCode) {
        let mut executed_instructions = HashSet::<usize>::new();

        let mut index: isize = 0;
        let mut accumulator = 0;
        loop {
            executed_instructions.insert(index as usize);
            // executate instruction
            match self.instructions[index as usize] {
                Instruction::Accumulate(value) => {
                    accumulator += value;
                    index += 1;
                }
                Instruction::Jump(value) => {
                    index += value as isize;
                }
                Instruction::NoOperation(_) => {
                    index += 1;
                }
            };

            // exit if we are about to enter an infinite loop,
            // or if we reached the end of the program
            if executed_instructions.contains(&(index as usize)) {
                return (accumulator, TerminationCode::InfiniteLoopAborted);
            }

            if index as usize == self.instructions.len() {
                return (accumulator, TerminationCode::Success);
            }
        }
    }

    fn fix_code(&mut self) -> i16 {
        use Instruction::*;

        for i in 0..self.instructions.len() {
            match self.instructions[i] {
                Accumulate(_) => continue,
                Jump(value) => {
                    self.instructions[i] = NoOperation(value);

                    match self.run_code() {
                        (value, TerminationCode::Success) => return value,
                        _ => self.instructions[i] = Jump(value),
                    }
                }
                NoOperation(value) => {
                    self.instructions[i] = Jump(value);

                    match self.run_code() {
                        (value, TerminationCode::Success) => return value,
                        _ => self.instructions[i] = NoOperation(value),
                    }
                }
            }
        }

        0
    }
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        let amount: i16 = words.next_back().unwrap().parse().unwrap();

        match words.next().unwrap() {
            "acc" => Ok(Instruction::Accumulate(amount)),
            "jmp" => Ok(Instruction::Jump(amount)),
            _ => Ok(Instruction::NoOperation(amount)),
        }
    }
}

impl std::str::FromStr for BootCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = s.lines().map(|line| line.parse().unwrap()).collect();

        Ok(BootCode { instructions })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut code: BootCode = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            .parse()
            .unwrap();

        assert_eq!(code.run_code().0, 5);
        assert_eq!(code.fix_code(), 8);
    }
}
