use std::{str::FromStr, num::ParseIntError};


fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug, PartialEq)]
struct InstructionProcessor {
    cycle: u32,
}

#[derive(Debug, PartialEq, Clone)]
struct Register<const N: usize> {
    values: [i32; N],
}

impl Register<1> {
    fn new() -> Self {
        Register{ values: [1; 1]}
    }
}

fn process_instructions<const N: usize>(
    instructions: &Vec<Instruction>,
    reg: &Register<N>
) -> Vec<Register<N>> {
    let mut register_outputs = Vec::new();
    let mut register = reg.clone();
    for i in instructions {
        match i {
            Instruction::Noop => {
                register_outputs.push(register.clone());
            },
            Instruction::Addx(x) => {
                register_outputs.push(register.clone());
                register_outputs.push(register.clone());
                register.values[0] += x;
            }
        }
    }
    register_outputs
}

#[derive(Debug, PartialEq)]
enum InstructionParseError {
    BadAddx(ParseIntError),
    UnexpectedInstruction(String),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
            match parts[0] {
                "addx" => Ok(Instruction::Addx(parts[1].parse::<i32>().map_err(|e| InstructionParseError::BadAddx(e))?)),
                "noop" => Ok(Instruction::Noop),
                unexpected => Err(InstructionParseError::UnexpectedInstruction(unexpected.to_string())),
            }
    }
}

fn part1(input: &str) -> i32 {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();
    let reg = Register::new();
    let results = process_instructions(&instructions, &reg);    
    
    results
        .iter()
        .enumerate()
        .filter_map(|(i, r)| {
            let offset_i = i as i32 + 1;
            let interesting = offset_i == 20 || (offset_i - 20) % 40 == 0;
            if interesting {
                let signal = offset_i * r.values[0];
                Some(signal)
            } else {
                None
            }
        })
        .take(6)
        .sum()
}


#[cfg(test)]
mod test {
    use crate::{Instruction, process_instructions, Register, part1};

    #[test]
    fn test_parse() {
        assert_eq!("noop".parse(), Ok(Instruction::Noop));
        assert_eq!("addx 3".parse(), Ok(Instruction::Addx(3)));
        assert_eq!("addx -5".parse(), Ok(Instruction::Addx(-5)));
        assert_eq!("addx 13".parse(), Ok(Instruction::Addx(13)));
    }

    #[test]
    fn test_part1_small_exampe() {
        let input = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];
        let reg = Register::new();

        let results = process_instructions(&input, &reg);

        assert_eq!(results[0].values[0], 1);
        assert_eq!(results[1].values[0], 1);
        assert_eq!(results[2].values[0], 1);
        assert_eq!(results[3].values[0], 4);
        assert_eq!(results[4].values[0], 4);
    }

    #[test]
    fn test_part1_larger_example() {
        let input = include_str!("input.example.txt");
        let result = part1(input);
        assert_eq!(13140, result);
    }
}