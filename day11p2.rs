#![feature(btree_cursors)]
#![allow(unused)]

use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    io::{Read as _, stdin},
    num::NonZeroU64,
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

const N: usize = 100000;
const M: usize = 76;

struct W {
    cache: [[Option<NonZeroU64>; M]; N],
}

impl W {
    fn work(&mut self, x: u64, t: usize) -> u64 {
        if x < N as u64
            && let Some(res) = self.cache[x as usize][t]
        {
            return res.get();
        }

        let mut res = 0;

        if t == 0 {
            res = 1;
        } else if x == 0 {
            res = self.work(1, t - 1);
        } else {
            let s = x.to_string();
            if s.len().is_multiple_of(2) {
                res += self.work(s[..s.len() / 2].parse().unwrap(), t - 1);
                res += self.work(s[s.len() / 2..].parse().unwrap(), t - 1);
            } else {
                res = self.work(x.checked_mul(2024).unwrap(), t - 1);
            }
        }

        if x < N as u64 {
            self.cache[x as usize][t] = Some(NonZeroU64::try_from(res).unwrap());
        }

        res
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut w = W {
        cache: [[None; _]; _],
    };

    let res = buf
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .map(|x| w.work(x, 75))
        .sum::<u64>();

    println!("{}", res);
}
