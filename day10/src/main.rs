use std::fs::File;
use std::io::Read;
use std::path::Path;
fn main() {
    println!("part_one: {}", part_one("input1.txt"));
    println!("part_one: {}", part_one("input2.txt"));
}

fn part_one(filepath: &str) -> i32 {
    let instructions: Vec<Instruction> = get_content(filepath.to_string())
        .lines()
        .into_iter()
        .map(|x| x.split(' '))
        .map(|mut x| (x.next().unwrap(), x.next()))
        .map(|(instruction, value)| match instruction {
            "noop" => Instruction::NoOp,
            "addx" => Instruction::AddX(value.unwrap().parse().unwrap()),
            _ => panic!(),
        })
        .collect();
    let mut cpu = CentralProcessingUnit::new(instructions);
    let mut sum = 0;
    let positions = vec![20, 60, 100, 140, 180, 220];
    while cpu.tick() {
        if positions.contains(&(cpu.position_1_indexed() as i32)) {
            sum += cpu.signal_strength();
            println!(
                "x:{}, signal_strength:{}, position:{}",
                cpu.register_x,
                cpu.signal_strength(),
                cpu.position
            );
        }
    }
    sum
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

struct CentralProcessingUnit {
    instructions: Vec<Instruction>,
    register_x: i32,
    position: usize,
}

impl CentralProcessingUnit {
    fn new(instructions: Vec<Instruction>) -> Self {
        let mut ret = Self::default();
        for instruction in instructions {
            ret.push(instruction)
        }
        ret
    }

    fn tick(&mut self) -> bool {
        let instruction = self.instructions.get(self.position);
        if instruction.is_none() {
            false
        } else {
            let instruction = instruction.unwrap();
            match instruction {
                Instruction::NoOp => {}
                Instruction::AddX(value) => self.register_x += value,
            }
            self.position += 1;
            true
        }
    }

    fn push(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NoOp => self.instructions.push(Instruction::NoOp),
            Instruction::AddX(value) => {
                self.instructions.push(Instruction::NoOp);
                self.instructions.push(Instruction::AddX(value))
            }
        }
    }

    fn position_1_indexed(&self) -> usize {
        self.position + 1
    }

    fn signal_strength(&self) -> i32 {
        self.register_x * self.position_1_indexed() as i32
    }
}

impl Default for CentralProcessingUnit {
    fn default() -> Self {
        CentralProcessingUnit {
            instructions: Vec::new(),
            register_x: 1,
            position: 0,
        }
    }
}

enum Instruction {
    NoOp,
    AddX(i32),
}
