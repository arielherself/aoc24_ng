#![feature(btree_cursors)]
#![allow(unused)]

use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    io::{Read as _, stdin},
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

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut a = buf
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..25 {
        let mut b = Vec::new();
        for &x in &a {
            let s = x.to_string();
            if s == "0" {
                b.push(1);
            } else if s.len() % 2 == 0 {
                b.push(s[..s.len() / 2].parse().unwrap());
                b.push(s[s.len() / 2..].parse().unwrap());
            } else {
                b.push(x.checked_mul(2024).unwrap());
            }
        }
        std::mem::swap(&mut a, &mut b);
        eprintln!("{}", a.iter().max().unwrap());
    }

    println!("{}", a.len());
}
