// use regex::Regex;
use std::fs;
// use itertools::Itertools;


#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl Instruction {
    fn decode_instruction(&mut self, instruction: String) {
        let mut values: Vec<usize> = vec![];
        for value in instruction.split_whitespace().skip(1).step_by(2) {
            values.push(value.parse::<usize>().unwrap());
        }

        if !values.is_empty() {
            self.amount= values[0];
            self.from = values[1];
            self.to= values[2];
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Storage{
    stacks: [Vec<char>; 9],
    instructions: Vec<Instruction>,
}

impl Storage {

    fn fill_crates(&mut self, file: String) {
        // reads the first 8 lines and fills the stacks respectively
        for line in file.split('\n').collect::<Vec<&str>>().into_iter().take(8).rev() {
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c != ' ' {
                    self.stacks[i].push(c);
                }
            }
        }
    }

    fn read_instructions(&mut self, instructions: Vec<String>) {
        for instruction in instructions {
            let mut inst: Instruction = Default::default();
            inst.decode_instruction(instruction);
            self.instructions.push(inst);
        }
    }

    fn execute_crate_movert_9000_instructions(&mut self) {
    // works like a queue so we'll just implement it like one 
        for instruction in self.instructions.clone() {
            println!("move {} from {} to {}", instruction.amount, instruction.from, instruction.to);
            for _ in 0..instruction.amount {
                let item = self.stacks[instruction.from-1].pop(); 
                if item.is_none() {
                    break;
                }
                self.stacks[instruction.to - 1].push(item.unwrap());
            }
        }
    }

    fn execute_crate_movert_9001_instructions(&mut self) {
        for instruction in self.instructions.clone() {
            println!("move {} from {} to {}", instruction.amount, instruction.from, instruction.to);
            let mut items = vec![];
            for _ in 0..instruction.amount {
                let tmp = self.stacks[instruction.from-1].pop();
                if tmp.is_none() {
                    break;
                }
                items.push(tmp.unwrap());
            }
            for _ in 0..instruction.amount {
                self.stacks[instruction.to - 1].push(items.pop().unwrap());
            }
        }
    }
}

fn main() {

    let file = fs::read_to_string("input").unwrap();
    let mut instructions = file.split('\n').skip(10).map(|i| i.to_string()).collect::<Vec<String>>();
    instructions.pop(); // removing the last one since it is and empty string
    let mut storage1: Storage = Default::default();
    let mut storage2: Storage = Default::default();

    storage1.fill_crates(file.clone());
    storage2.fill_crates(file);
    storage1.read_instructions(instructions.clone());
    storage2.read_instructions(instructions);
    storage1.execute_crate_movert_9000_instructions();
    storage2.execute_crate_movert_9001_instructions(); // big difference
    
    let mut p1 = vec![];
    for list in storage1.stacks {
        p1.push(list.last().unwrap().to_string())
    }

    let mut p2 = vec![];
    for list in storage2.stacks {
        p2.push(list.last().unwrap().to_string())
    }

    println!("{:?}", p1.concat());
    println!("{:?}", p2.concat());
}
