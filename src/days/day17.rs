use regex::Regex;
use std::fmt::{Display, Formatter, Result, Write};
use std::io::{BufRead, Lines};

struct Cpu {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
}

impl Cpu {
    fn new(registers: [u64; 3]) -> Self {
        Cpu {
            a: registers[0],
            b: registers[1],
            c: registers[2],
            pc: 0,
        }
    }

    fn combo(&self, value: u8) -> u64 {
        match value {
            0..4 => value as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }

    fn adv(&mut self, operand: u8) {
        self.a >>= self.combo(operand);
    }

    fn bxl(&mut self, operand: u8) {
        self.b ^= operand as u64;
    }

    fn bst(&mut self, operand: u8) {
        self.b = self.combo(operand) & 0x7;
    }

    fn jnz(&mut self, operand: u8) {
        if self.a != 0 {
            self.pc = operand as usize;
        }
    }

    fn bxc(&mut self) {
        self.b ^= self.c;
    }

    fn out(&mut self, operand: u8) -> u8 {
        (self.combo(operand) & 0x7) as u8
    }

    fn bdv(&mut self, operand: u8) {
        self.b = self.a >> self.combo(operand);
    }

    fn cdv(&mut self, operand: u8) {
        self.c = self.a >> self.combo(operand);
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "A: {}, B: {}, C: {}, PC: {}",
            self.a, self.b, self.c, self.pc
        )
    }
}

struct Program(Vec<u8>);

struct ProgramIterator<'a> {
    program: &'a Program,
    cpu: Cpu,
}

impl<'a> Iterator for ProgramIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cpu.pc < self.program.0.len() {
            let opcode = self.program.0[self.cpu.pc];
            let operand = self.program.0[self.cpu.pc + 1];

            self.cpu.pc += 2;

            match opcode {
                0 => self.cpu.adv(operand),
                1 => self.cpu.bxl(operand),
                2 => self.cpu.bst(operand),
                3 => self.cpu.jnz(operand),
                4 => self.cpu.bxc(),
                5 => return Some(self.cpu.out(operand)),
                6 => self.cpu.bdv(operand),
                7 => self.cpu.cdv(operand),
                _ => panic!(),
            }
        }

        None
    }
}

impl<'a> Display for ProgramIterator<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.cpu.fmt(f)
    }
}

impl Program {
    fn run(&self, registers: [u64; 3]) -> ProgramIterator<'_> {
        ProgramIterator {
            program: self,
            cpu: Cpu::new(registers),
        }
    }
}

#[test]
fn check_program() {
    let program = Program(vec![0, 1, 5, 4, 3, 0]);
    let output = vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0];

    for (a, &b) in program.run([729, 0, 0]).zip(output.iter()) {
        assert_eq!(a, b);
    }

    let program = Program(vec![0, 3, 5, 4, 3, 0]);

    for (a, &b) in program.run([117440, 0, 0]).zip(program.0.iter()) {
        assert_eq!(a, b);
    }
}

fn check_first_output(program: &Program, a: u64, expected: u8) -> Option<u64> {
    let output = program.run([a, 0, 0]).next().unwrap();

    if output == expected {
        Some(a)
    } else {
        None
    }
}

fn find_values(n: usize, program: &Program) -> Vec<u64> {
    assert!(n > 0);
    assert!(n <= program.0.len());

    let expected = program.0[program.0.len() - n];

    if n == 1 {
        (1..8)
            .filter_map(|a| check_first_output(program, a, expected))
            .collect()
    } else {
        find_values(n - 1, program)
            .iter()
            .flat_map(|v| (0..8).map(move |i| (v << 3) + i))
            .filter_map(|a| check_first_output(program, a, expected))
            .collect()
    }
}

fn get_a(program: &Program) -> u64 {
    let vs = find_values(program.0.len(), program);

    if vs.is_empty() {
        u64::MAX
    } else {
        vs[0]
    }
}

fn get_output(program: &Program, registers: [u64; 3]) -> String {
    let mut output = String::new();

    for o in program.run(registers) {
        if output.is_empty() {
            write!(output, "{o}").unwrap();
        } else {
            write!(output, ",{o}").unwrap();
        }
    }

    output
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

    let program = Program(program);

    (get_output(&program, registers), get_a(&program))
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
    (solution.0, solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
