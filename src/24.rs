use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

type Word = i64;

enum Register {
    W,
    X,
    Y,
    Z,
    Constant(Word),
}

impl FromStr for Register {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Register::W),
            "x" => Ok(Register::X),
            "y" => Ok(Register::Y),
            "z" => Ok(Register::Z),
            _ => Ok(Register::Constant(s.parse::<Word>()?)),
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::W => write!(f, "w"),
            Register::X => write!(f, "x"),
            Register::Y => write!(f, "y"),
            Register::Z => write!(f, "z"),
            Register::Constant(c) => write!(f, "{}", c),
        }
    }
}

enum Instruction {
    Inp(Register),
    Add(Register, Register),
    Mul(Register, Register),
    Div(Register, Register),
    Mod(Register, Register),
    Eql(Register, Register),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split(' ');
        let instr = tokens.next().unwrap();

        match instr {
            "inp" => Ok(Instruction::Inp(tokens.next().unwrap().parse()?)),
            "add" => Ok(Instruction::Add(tokens.next().unwrap().parse()?, tokens.next().unwrap().parse()?)),
            "mul" => Ok(Instruction::Mul(tokens.next().unwrap().parse()?, tokens.next().unwrap().parse()?)),
            "div" => Ok(Instruction::Div(tokens.next().unwrap().parse()?, tokens.next().unwrap().parse()?)),
            "mod" => Ok(Instruction::Mod(tokens.next().unwrap().parse()?, tokens.next().unwrap().parse()?)),
            "eql" => Ok(Instruction::Eql(tokens.next().unwrap().parse()?, tokens.next().unwrap().parse()?)),
            _ => panic!("I wanted an instruction you fool, but I got \"{}\".", instr),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Inp(dst) => write!(f, "inp {}", dst),
            Instruction::Add(dst, src) => write!(f, "add {} {}", dst, src),
            Instruction::Mul(dst, src) => write!(f, "mul {} {}", dst, src),
            Instruction::Div(dst, src) => write!(f, "div {} {}", dst, src),
            Instruction::Mod(dst, src) => write!(f, "mod {} {}", dst, src),
            Instruction::Eql(dst, src) => write!(f, "eql {} {}", dst, src),
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Processor {
    w: Word,
    x: Word,
    y: Word,
    z: Word,
}

impl Processor {
    fn store(&mut self, register: &Register, word: Word) {
        match register {
            Register::W => self.w = word,
            Register::X => self.x = word,
            Register::Y => self.y = word,
            Register::Z => self.z = word,
            Register::Constant(_) => panic!("Illegal instruction"),
        };
    }

    fn fetch(&self, register: &Register) -> Word {
        match register {
            Register::W => self.w,
            Register::X => self.x,
            Register::Y => self.y,
            Register::Z => self.z,
            Register::Constant(c) => *c,
        }
    }

    pub fn run_one(&mut self, instruction: &Instruction, input: Word) -> bool {
        let (dst, result) = match instruction {
            Instruction::Inp(dst) => (dst, Some(input)),
            Instruction::Add(dst, src) => (dst, self.fetch(dst).checked_add(self.fetch(src))),
            Instruction::Mul(dst, src) => (dst, self.fetch(dst).checked_mul(self.fetch(src))),
            Instruction::Div(dst, src) => (dst, self.fetch(dst).checked_div(self.fetch(src))),
            Instruction::Mod(dst, src) => (dst, self.fetch(dst).checked_rem(self.fetch(src))),
            Instruction::Eql(dst, src) => (dst, match self.fetch(dst) == self.fetch(src) {
                true => Some(1),
                false => Some(0),
            }),
        };

        if let Some(val) = result {
            self.store(dst, val);
            return false;
        }

        return true;
    }

    pub fn run(&mut self, instructions: &[Instruction], input: Word) -> bool {
        if let Some(_) = instructions.iter().find(|instruction| {
            // println!("------------");
            // println!("Run: {}", &instruction);
            let result = self.run_one(&instruction, input);
            // println!("{}", &self);
            result
        }) {
            return true;
        }

        return false;
    }

    pub fn valid(&self) -> bool {
        self.z == 0
    }
}

impl fmt::Display for Processor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "w: {}\nx: {}\ny: {}\nz: {}", self.w, self.x, self.y, self.z)
    }
}

fn process_branch(branches: &Vec<&[Instruction]>, mut cpu: Processor, depth: usize) -> Option<Vec<Word>> {
    (0..10_i64)
        .rev()
        .filter_map(|input| {
            //println!("pb({}, {})", depth, input);
            if cpu.run(&branches[depth], input) {
                return None;
            }

            if depth < 0 {
                println!("Depth {} at {}", depth, input);
            }

            if depth < 12 {
                if let Some(mut vec) = process_branch(branches, cpu, depth + 1) {
                    vec.push(input);
                    Some(vec)
                } else {
                    None
                }
            } else {
                if cpu.valid() {
                    Some(vec![input])
                } else {
                    None
                }
            }
        })
        .next()

}

fn main() {
    let instructions: Vec<Instruction> = include_str!("../input/24.txt")
        .trim()
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let (branches, _) = instructions
        .iter()
        .enumerate()
        .fold((Vec::new(), None), |(mut branches, last_input), (i, instr)| {
            if let Instruction::Inp(_) = instr {
                if let Some(last_idx) = last_input {
                    branches.push(&instructions[last_idx..i]);
                }
                return (branches, Some(i));
            }
            return (branches, last_input);
        });

    println!("Branches: {}", branches.len());
    for instr in branches[12] {
        println!("{}", instr);
    }

    let result = process_branch(&branches, Processor::default(), 0);
    for r in result {
        for s in r {
            println!("{}", s);
        }
    }

    // if let Some(serial_number) = result {
    //     println!("Part 1: {}", serial_number);
    // } else {
    //     println!("We failed to find a serial number.");
    // }
}
