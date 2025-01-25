use regex::Regex;
use std::fmt::Write;
use std::io::{BufRead, Lines};

#[derive(Debug, Clone)]
struct Computer {
    registers: [u64; 3],
    program: Vec<u8>,
    pc: usize,
    out: Vec<u8>,
}

impl Computer {
    fn new(program: Vec<u8>) -> Self {
        let registers = [0; 3];
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
    }

    fn bxl(&mut self, operand: u8) {
        //println!("bxl {operand}");
        self.registers[1] ^= operand as u64;
    }

    fn bst(&mut self, operand: u8) {
        //println!("bst {operand}");
        self.registers[1] = self.combo(operand) & 0x7;
    }

    fn jnz(&mut self, operand: u8) -> bool {
        //println!("jnz {operand}");
        if self.registers[0] == 0 {
            false
        } else {
            self.pc = operand as usize;
            true
        }
        //println!();
    }

    fn bxc(&mut self, _operand: u8) {
        //println!("bxc");
        self.registers[1] ^= self.registers[2];
    }

    fn out(&mut self, operand: u8) {
        //println!("out {operand}");
        //println!();
        self.out.push((self.combo(operand) & 0x7) as u8);
    }

    fn bdv(&mut self, operand: u8) {
        //println!("bdv {operand}");
        self.registers[1] = self.registers[0] >> self.combo(operand);
    }

    fn cdv(&mut self, operand: u8) {
        //println!("cdv {operand}");
        self.registers[2] = self.registers[0] >> self.combo(operand);
    }

    fn run(&mut self, registers: [u64; 3]) {
        self.registers = registers;
        self.pc = 0;
        self.out.clear();

        while self.pc < self.program.len() {
            let opcode = self.program[self.pc];
            let operand = self.program[self.pc + 1];

            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => {
                    if self.jnz(operand) {
                        continue;
                    }
                }
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!(),
            }

            //println!("A: {:#050b}", self.registers[0]);
            //println!("B: {:#050b}", self.registers[1]);
            //println!("C: {:#050b}", self.registers[2]);

            self.pc += 2;
        }
    }

    fn out_string(&self) -> String {
        self.out.iter().fold(String::new(), |mut s, &n| {
            if s.is_empty() {
                write!(s, "{}", n).ok();
            } else {
                write!(s, ",{}", n).ok();
            }
            s
        })
    }

    fn match_the_end(&self) -> bool {
        self.out
            .iter()
            .rev()
            .zip(self.program.iter().rev())
            .all(|(a, b)| a == b)
    }

    fn get_register_a_values(&mut self, n: usize) -> Vec<u64> {
        if n == 0 {
            return vec![0];
        }

        let mut ret = vec![];

        for next in self.get_register_a_values(n - 1) {
            for i in 0..8 {
                let a = (next << 3) + i;

                self.run([a, 0, 0]);

                let is_ok = self.out.len() == n && self.match_the_end();

                if is_ok {
                    ret.push(a);
                }
            }
        }

        ret
    }

    fn get_register_a_value(&mut self) -> u64 {
        let n = self.program.len();

        assert!(n > 0);

        for next in self.get_register_a_values(n - 1) {
            for i in 0..8 {
                let a = (next << 3) + i;

                self.run([a, 0, 0]);

                let is_ok = self.out.len() == n && self.match_the_end();

                if is_ok {
                    return a;
                }
            }
        }

        0
    }
}

#[test]
fn check_match_the_end() {
    let mut c = Computer::new(vec![0, 1, 2, 3, 4]);

    c.out = vec![4];
    assert!(c.match_the_end());

    c.out = vec![3];
    assert!(!c.match_the_end());

    c.out = vec![3, 4];
    assert!(c.match_the_end());

    c.out = vec![2, 4];
    assert!(!c.match_the_end());

    c.out = vec![0, 1, 2, 3, 4];
    assert!(c.match_the_end());

    c.out = vec![0, 1, 2, 3, 4, 5];
    assert!(!c.match_the_end());
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

    let mut computer = Computer::new(program);

    computer.run(registers);

    (computer.out_string(), computer.get_register_a_value())
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
