use regex::Regex;
use std::collections::HashMap;
use std::io::{BufRead, Lines};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Operation {
    Value(bool),
    Xor(u32, u32),
    Or(u32, u32),
    And(u32, u32),
}

fn convert(name: &[u8]) -> u32 {
    name[2] as u32 | ((name[1] as u32) << 8) | ((name[0] as u32) << 16)
}

fn compute(tree: &HashMap<u32, Operation>, name: u32) -> Option<bool> {
    let operation = tree.get(&name)?;

    Some(match *operation {
        Operation::Value(x) => x,
        Operation::Xor(a, b) => compute(tree, a)? != compute(tree, b)?,
        Operation::Or(a, b) => compute(tree, a)? || compute(tree, b)?,
        Operation::And(a, b) => compute(tree, a)? && compute(tree, b)?,
    })
}

fn resolve<T>(lines: Lines<T>) -> (u64, String)
where
    T: BufRead,
{
    let re_init = Regex::new(r"(\S+): (\d)").unwrap();
    let re_rule = Regex::new(r"(\S+) (\w+) (\S+) -> (\S+)").unwrap();
    let mut is_rule = false;
    let mut tree: HashMap<u32, Operation> = HashMap::new();
    let mut inverted_tree: HashMap<Operation, u32> = HashMap::new();

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            is_rule = true;
            continue;
        }

        if is_rule {
            let caps = re_rule.captures(&line).unwrap();

            let a = convert(caps.get(1).unwrap().as_str().as_bytes());
            let b = convert(caps.get(3).unwrap().as_str().as_bytes());
            let c = convert(caps.get(4).unwrap().as_str().as_bytes());
            let ope = caps.get(2).unwrap().as_str();

            let (a, b) = if a > b { (b, a) } else { (a, b) };

            let operation = match ope {
                "XOR" => Operation::Xor(a, b),
                "OR" => Operation::Or(a, b),
                "AND" => Operation::And(a, b),
                _ => panic!(),
            };

            tree.insert(c, operation);
            inverted_tree.insert(operation, c);
        } else {
            let caps = re_init.captures(&line).unwrap();

            let name = convert(caps.get(1).unwrap().as_str().as_bytes());
            let value = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

            tree.insert(name, Operation::Value(value == 1));
        }
    }

    let mut p1: u64 = 0;
    let mut count = 0;

    while let Some(bit) = compute(
        &tree,
        convert(&[b'z', b'0' + count / 10, b'0' + count % 10]),
    ) {
        p1 |= if bit { 1 << count } else { 0 };
        count += 1;
    }

    if cfg!(test) {
        return (p1, String::from(""));
    }

    /*
     * https://www.101computing.net/binary-additions-using-logic-gates/
     */
    let mut inverted = vec![];
    let mut x = convert(b"x00");
    let mut y = convert(b"y00");
    let mut z = convert(b"z00");
    let s = *inverted_tree.get(&Operation::Xor(x, y)).unwrap();
    let mut cin = *inverted_tree.get(&Operation::And(x, y)).unwrap();

    if s != z {
        inverted.extend([s, z]);
    }

    for i in 1..(count - 1) {
        let digits = (b'0' + i / 10, b'0' + i % 10);

        x = convert(&[b'x', digits.0, digits.1]);
        y = convert(&[b'y', digits.0, digits.1]);
        z = convert(&[b'z', digits.0, digits.1]);

        let mut s1 = *inverted_tree.get(&Operation::Xor(x, y)).unwrap();

        let (mut a, mut b) = if cin < s1 { (cin, s1) } else { (s1, cin) };
        let s2 = if let Some(v) = inverted_tree.get(&Operation::Xor(a, b)) {
            *v
        } else {
            /*
             * Here we want to know if the bad wire is s1 or cin so
             * let's look up the other way by checking z
             *
             * This code is ugly.
             */
            if let Some(Operation::Xor(left, right)) = tree.get(&z) {
                let (i, j): (&mut u32, &u32);

                if *left == s1 {
                    i = &mut cin;
                    j = right;
                } else if *right == s1 {
                    i = &mut cin;
                    j = left;
                } else if *left == cin {
                    i = &mut s1;
                    j = right;
                } else if *right == cin {
                    i = &mut s1;
                    j = left;
                } else {
                    panic!();
                }

                inverted.extend([*i, *j]);

                let ope = tree.get(j).unwrap();
                inverted_tree.insert(*ope, *i);

                *i = *j;
            }

            (a, b) = if cin < s1 { (cin, s1) } else { (s1, cin) };
            *inverted_tree.get(&Operation::Xor(a, b)).unwrap()
        };

        if s2 != z {
            inverted_tree.insert(*tree.get(&z).unwrap(), s2);

            inverted.extend([s2, z]);
        }

        let cout1 = *inverted_tree.get(&Operation::And(x, y)).unwrap();
        let cout2 = *inverted_tree.get(&Operation::And(a, b)).unwrap();

        let (a, b) = if cout1 < cout2 {
            (cout1, cout2)
        } else {
            (cout2, cout1)
        };
        cin = *inverted_tree.get(&Operation::Or(a, b)).unwrap();
    }

    let mut p2 = String::from("");
    inverted.sort_unstable();

    for v in inverted {
        p2 = format!(
            "{}{}{}{}{}",
            p2,
            if p2.is_empty() { "" } else { "," },
            (v >> 16) as u8 as char,
            ((v >> 8) & 0xff) as u8 as char,
            (v & 0xff) as u8 as char,
        );
    }

    (p1, p2)
}

#[test]
fn check() {
    const TEST1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    const TEST2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST1).lines()).0, 4);
    assert_eq!(resolve(Cursor::new(TEST2).lines()).0, 2024);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
