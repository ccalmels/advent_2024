use regex::Regex;
use std::io::{BufRead, Lines};

#[derive(Debug, Clone)]
struct Computer {
    registers: [u64; 3],
    program: Vec<u8>,
    pc: usize,
    out: Vec<u8>,
}

impl Computer {
    fn new(registers: [u64; 3], program: Vec<u8>) -> Self {
        let pc = 0;
        let out = vec![];

        Computer {
            registers,
            program,
            pc,
            out,
        }
    }

    fn combo(&self, value: u8) -> u64 {
        match value {
            0..4 => value as u64,
            4..7 => self.registers[(value - 4) as usize],
            _ => panic!(),
        }
    }

    fn adv(&mut self, operand: u8) {
        //println!("adv {operand}");
        self.registers[0] >>= self.combo(operand);
        self.pc += 2;
    }

    fn bxl(&mut self, operand: u8) {
        //println!("bxl {operand}");
        self.registers[1] ^= operand as u64;
        self.pc += 2;
    }

    fn bst(&mut self, operand: u8) {
        //println!("bst {operand}");
        self.registers[1] = self.combo(operand) & 0x7;
        self.pc += 2;
    }

    fn jnz(&mut self, operand: u8) {
        //println!("jnz {operand}");
        if self.registers[0] == 0 {
            self.pc += 2;
        } else {
            self.pc = operand as usize;
        }
        //println!();
    }

    fn bxc(&mut self, _operand: u8) {
        //println!("bxc");
        self.registers[1] ^= self.registers[2];
        self.pc += 2;
    }

    fn out(&mut self, operand: u8) {
        //println!("out {operand}");
        //println!();
        self.out.push((self.combo(operand) & 0x7) as u8);
        self.pc += 2;
    }

    fn bdv(&mut self, operand: u8) {
        //println!("bdv {operand}");
        self.registers[1] = self.registers[0] >> self.combo(operand);
        self.pc += 2;
    }

    fn cdv(&mut self, operand: u8) {
        //println!("cdv {operand}");
        self.registers[2] = self.registers[0] >> self.combo(operand);
        self.pc += 2;
    }

    fn start(&mut self) {
        while self.pc < self.program.len() {
            let opcode = self.program[self.pc];
            let operand = self.program[self.pc + 1];

            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!(),
            }

            //println!("A: {:#050b}", self.registers[0]);
            //println!("B: {:#050b}", self.registers[1]);
            //println!("C: {:#050b}", self.registers[2]);
        }
    }

    fn out_string(&self) -> String {
        let mut s = String::from("");

        for v in &self.out {
            if !s.is_empty() {
                s.push(',');
            }
            s.push_str(&(v.to_string()));
        }

        s
    }
}

fn resolve<T>(lines: Lines<T>) -> (String, u64)
where
    T: BufRead,
{
    let re = Regex::new(r"Register (\w): (\d+)|Program: ([\d,]+)").unwrap();
    let mut registers = [0; 3];
    let mut program = vec![];

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        let caps = re.captures(&line).unwrap();

        if let Some(values) = caps.get(3) {
            program = values
                .as_str()
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
        } else {
            let register = caps.get(1).unwrap().as_str().as_bytes()[0] - b'A';
            let value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();

            registers[register as usize] = value;
        }
    }

    let computer = Computer::new(registers, program);
    let program_len = computer.program.len();
    let mut values: Vec<Vec<u64>> = vec![vec![]; program_len + 1];

    values[0].push(0);

    let mut p2 = 0;

    'outer: for octal in 0..program_len {
        for i in 0..values[octal].len() {
            let v = values[octal][i] << 3;

            for i in 0..8 {
                let mut tmp = computer.clone();

                tmp.registers[0] = v + i;

                tmp.start();

                let is_ok = tmp.out.iter().enumerate().all(|(idx, &v)| {
                    v == computer.program[program_len - 1 - octal + idx]
                });

                if is_ok {
                    if octal == program_len - 1 {
                        p2 = v + i;
                        break 'outer
                    }
                    values[octal + 1].push(v + i);
                }
            }
        }
    }

    let mut computer = computer;

    computer.start();

    (computer.out_string(), p2)
}

#[test]
fn check() {
    const TEST1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    const TEST2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST1).lines()).0, "4,6,3,5,6,3,5,2,1,0");
    assert_eq!(resolve(Cursor::new(TEST2).lines()).1, 117440);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
