use std::{str, fmt, cmp, collections, clone};

#[derive(fmt::Debug)]
struct ParseInstructionError;

#[derive(fmt::Debug, cmp::PartialEq, cmp::Eq, clone::Clone)]
enum Operation {
    JMP,
    ACC,
    NOP
}
impl str::FromStr for Operation {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..] {
            "jmp" => Ok(Operation::JMP),
            "acc" => Ok(Operation::ACC),
            "nop" => Ok(Operation::NOP),
            _ => Err(ParseInstructionError)
        }
    }
}

#[derive(fmt::Debug, cmp::PartialEq, cmp::Eq, clone::Clone)]
struct Instruction {
    operation: Operation,
    argument: i64,
}

impl str::FromStr for Instruction {
    type Err = ParseInstructionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        Ok(Instruction{
            operation: parts[0].parse()?,
            argument: parts[1].parse()
                .or(Err(ParseInstructionError))? 
        })
    }
}

fn execute(instructions: &Vec<Instruction>) -> Result<i64, i64> {
    let mut counter: i64 = 0;
    let mut accumulator: i64 = 0;
    let mut past_instructions: collections::HashSet<i64> = collections::HashSet::new();
    loop {
        if past_instructions.contains(&counter) {
            return Err(accumulator);
        }
        past_instructions.insert(counter);
        let instruction = match instructions.get(counter as usize) {
            Some(instruction) => instruction,
            None => return Ok(accumulator)
        };
        match instruction.operation {
            Operation::NOP => {
                counter += 1;
            },
            Operation::ACC => {
                accumulator += instruction.argument;
                counter += 1
            },
            Operation::JMP => {
                counter += instruction.argument;
            }
        }
    }
}

pub fn run(input: &str) {
    let instructions: Vec<Instruction> = input.lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    println!("Part 1: {:?}", part_1(&instructions));
    println!("Part 2: {:?}", part_2(&instructions));
}

fn part_1(instructions: &Vec<Instruction>) -> Result<i64, i64> {
    execute(&instructions)
}

fn part_2(instructions: &Vec<Instruction>) -> Result<i64, i64> {
    for (i, instruction) in instructions.iter().enumerate() {
        let replacement: Instruction;
        match instruction.operation {
            Operation::NOP => {
                replacement = Instruction{
                    operation: Operation::JMP, 
                    argument: instruction.argument,
                };
            },
            Operation::JMP => {
                replacement = Instruction{
                    operation: Operation::NOP, 
                    argument: instruction.argument,
                }
            },
            Operation::ACC => {
                continue;
            }
        }
        let mut tmp = instructions.to_vec();
        tmp[i] = replacement;
        let res = execute(&tmp);
        if res.is_ok() {
            return res;
        }
    }
    Err(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_instruction() {
        assert_eq!("nop +0".parse::<Instruction>().unwrap(), Instruction{
            operation: Operation::NOP,
            argument: 0
        });
        assert_eq!("jmp -3".parse::<Instruction>().unwrap(), Instruction{
            operation: Operation::JMP,
            argument: -3 
        });
        assert_eq!("acc +1".parse::<Instruction>().unwrap(), Instruction{
            operation: Operation::ACC,
            argument: 1 
        });
    }

    #[test]
    fn part_1_test() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let input: Vec<Instruction> = input.lines()
            .filter_map(|line| line.parse().ok())
            .collect();
        assert_eq!(part_1(&input).unwrap_err(), 5);
    }
    #[test]
    fn part_2_test() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let input: Vec<Instruction> = input.lines()
            .filter_map(|line| line.parse().ok())
            .collect();
        assert_eq!(part_2(&input).unwrap(), 8);
    }
}
