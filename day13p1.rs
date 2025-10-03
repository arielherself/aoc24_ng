#![feature(btree_cursors)]
#![allow(unused)]

use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    io::{Read as _, stdin},
    num::NonZeroU64,
    ops::Index,
};

macro_rules! boxed_slice {
    [$value: expr; $len: expr] => {{
        let mut res = Box::new_uninit_slice($len);
        for x in res.iter_mut() {
            x.write($value);
        }
        unsafe { res.assume_init() }
    }}
}

/// w1 * x + w2 * y = a
/// w3 * x + w4 * y = b
///
/// w1 * w4 * x + w2 * w4 * y = a * w4
/// w3 * w2 * x + w4 * w2 * y = b * w2
///
/// (w1 * w4 - w3 * w2) * x = a * w4 - b * w2
fn work(button_a: (i64, i64), button_b: (i64, i64), prize: (i64, i64)) -> i64 {
    let (w1, w3) = button_a;
    let (w2, w4) = button_b;
    let (a, b) = prize;

    assert!(w1 * w2 * w3 * w4 != 0);

    let k = w1 * w4 - w3 * w2;
    let c = a * w4 - b * w2;

    if k == 0 {
        if 3 * w2 >= w1 {
            let t = 3 * w2 - w1;
            for x in 0..100000.min(a / w1) {
                if (a + t * x) % w2 == 0 {
                    let y = (a - w1 * x) / w2;
                    return 3 * x + y;
                }
            }
            return 0;
        } else {
            let t = w1 - 3 * w2;
            for x in (0..(a / t).min(a / w1)).rev() {
                if (a - t * x) % w2 == 0 {
                    let y = (a - w1 * x) / w2;
                    return 3 * x + y;
                }
            }
            return 0;
        }
    }

    if c % k != 0 {
        return 0;
    }

    let x = c / k;
    let y = (a - w1 * x) / w2;

    3 * x + y
}

fn main() {
    let mut buf = String::new();
    let mut lines = Vec::new();
    while let Ok(len) = stdin().read_line(&mut buf)
        && len != 0
    {
        let line = std::mem::take(&mut buf);
        lines.push(line.trim().to_string());
    }

    let result = lines
        .split(String::is_empty)
        .map(|v| {
            assert_eq!(v.len(), 3);
            let button_a = v[0].as_bytes();
            let button_b = v[1].as_bytes();
            let prize = v[2].as_bytes();

            let button_a = unsafe {
                let p1 = button_a.iter().position(|&x| x == b'+').unwrap();
                let p2 = p1 + button_a[p1..].iter().position(|&x| x == b',').unwrap();
                let p3 = p2 + button_a[p2..].iter().position(|&x| x == b'+').unwrap();
                (
                    String::from_utf8_unchecked(button_a[p1 + 1..p2].to_vec())
                        .parse::<i64>()
                        .unwrap(),
                    String::from_utf8_unchecked(button_a[p3 + 1..].to_vec())
                        .parse::<i64>()
                        .unwrap(),
                )
            };
            let button_b = unsafe {
                let p1 = button_b.iter().position(|&x| x == b'+').unwrap();
                let p2 = p1 + button_b[p1..].iter().position(|&x| x == b',').unwrap();
                let p3 = p2 + button_b[p2..].iter().position(|&x| x == b'+').unwrap();
                (
                    String::from_utf8_unchecked(button_b[p1 + 1..p2].to_vec())
                        .parse::<i64>()
                        .unwrap(),
                    String::from_utf8_unchecked(button_b[p3 + 1..].to_vec())
                        .parse::<i64>()
                        .unwrap(),
                )
            };
            let prize = unsafe {
                let p1 = prize.iter().position(|&x| x == b'=').unwrap();
                let p2 = p1 + prize[p1..].iter().position(|&x| x == b',').unwrap();
                let p3 = p2 + prize[p2..].iter().position(|&x| x == b'=').unwrap();
                (
                    String::from_utf8_unchecked(prize[p1 + 1..p2].to_vec())
                        .parse::<i64>()
                        .unwrap(),
                    String::from_utf8_unchecked(prize[p3 + 1..].to_vec())
                        .parse::<i64>()
                        .unwrap(),
                )
            };
            work(button_a, button_b, prize)
        })
        .sum::<i64>();

    println!("{result}");
}
