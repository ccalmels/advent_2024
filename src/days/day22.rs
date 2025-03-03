use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{BufRead, Lines};

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

fn compute_buyer(mut secret: Secret) -> (u64, HashMap<u32, i32>) {
    let mut seq: u32 = 0;
    let mut prev = (secret as i32) % 10;
    let mut prices: HashMap<u32, i32> = HashMap::new();

    for i in 0..2000 {
        secret = next_secret(secret);

        let price = (secret as i32) % 10;

        seq = (seq << 8) | ((price - prev) as u8) as u32;

        if i > 2 {
            prices.entry(seq).or_insert(price);
        }

        prev = price;
    }

    (secret as u64, prices)
}

fn resolve<T>(lines: Lines<T>) -> (u64, i32)
where
    T: BufRead,
{
    let buyers = lines
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let res = buyers.into_par_iter().map(compute_buyer).reduce(
        || (0, HashMap::new()),
        |mut a: (u64, HashMap<u32, i32>), b: (u64, HashMap<u32, i32>)| {
            if a.1.is_empty() {
                b
            } else {
                a.0 += b.0;

                for (k, v) in b.1 {
                    *a.1.entry(k).or_default() += v;
                }
                a
            }
        },
    );

    let mut p2 = 0;

    for &r in res.1.values() {
        p2 = p2.max(r);
    }

    (res.0, p2)
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
