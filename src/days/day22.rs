use rayon::prelude::*;
use std::io::{BufRead, Lines};

const BUYERS_COUNT: usize = 1632;

type Secret = u32;

fn next_secret(mut secret: Secret) -> Secret {
    // 1.
    secret ^= secret << 6;
    secret &= 0xffffff;

    // 2.
    secret ^= secret >> 5;
    secret &= 0xffffff;

    // 3.
    secret ^= secret << 11;
    secret &= 0xffffff;

    secret
}

#[test]
fn check_next_secret() {
    let mut secret = 123;

    let secrets = (0..10)
        .map(|_| {
            secret = next_secret(secret);
            secret
        })
        .collect::<Vec<_>>();

    assert_eq!(
        secrets,
        [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254
        ]
    );
}

// from -9 to 9 => 0 to 18
const BASE: usize = 19;
const PRICES_SIZE: usize = BASE * BASE * BASE * BASE;

fn next_sequence(sequence: usize, delta: usize) -> usize {
    (sequence * BASE + delta) % PRICES_SIZE
}

#[test]
fn check_array() {
    let mut sequence = 0usize;

    sequence = next_sequence(sequence, 13);

    assert_eq!(sequence, 13);

    sequence = next_sequence(sequence, 4);

    assert_eq!(sequence, 13 * BASE + 4);

    sequence = next_sequence(sequence, 0);

    assert_eq!(sequence, 13 * BASE * BASE + 4 * BASE);

    sequence = next_sequence(sequence, 17);

    assert_eq!(sequence, 13 * BASE * BASE * BASE + 4 * BASE * BASE + 17);

    sequence = next_sequence(sequence, 8);

    assert_eq!(sequence, 4 * BASE * BASE * BASE + 17 * BASE + 8);
}

type Prices = (u64, Vec<i32>);

fn compute_buyers(secrets: &[Secret]) -> Prices {
    let mut prices = vec![0; PRICES_SIZE];
    let mut it_was_buyer = vec![u32::MAX; PRICES_SIZE];
    let mut p1 = 0u64;

    for (index, &(mut secret)) in secrets.iter().enumerate() {
        let mut seq: usize = 0;
        let mut prev = (secret as i32) % 10;
        let index = index as u32;

        for i in 0..2000 {
            secret = next_secret(secret);

            let price = (secret as i32) % 10;

            seq = next_sequence(seq, (price + 9 - prev) as usize);

            if i > 2 && it_was_buyer[seq] != index {
                prices[seq] += price;
                it_was_buyer[seq] = index;
            }

            prev = price;
        }

        p1 += secret as u64;
    }

    (p1, prices)
}

fn resolve<T>(lines: Lines<T>) -> (u64, i32)
where
    T: BufRead,
{
    let chunk_size = BUYERS_COUNT.div_ceil(rayon::current_num_threads());

    let (p1, prices) = lines
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .par_chunks(chunk_size)
        .map(compute_buyers)
        .reduce(
            || (0, vec![0; PRICES_SIZE]),
            |mut a: Prices, b: Prices| {
                a.0 += b.0;

                for (va, vb) in a.1.iter_mut().zip(b.1) {
                    *va += vb;
                }

                a
            },
        );

    let p2 = prices.into_iter().max().unwrap();

    (p1, p2)
}

#[test]
fn check() {
    const TEST1: &str = "1
10
100
2024";
    const TEST2: &str = "1
2
3
2024";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST1).lines()).0, 37327623);
    assert_eq!(resolve(Cursor::new(TEST2).lines()).1, 23);
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2024::Day::new(file!(), resolve_string) }
