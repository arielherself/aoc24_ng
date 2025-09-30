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
    let mut a = Vec::new();
    while let Ok(len) = stdin().read_line(&mut buf)
        && len != 0
    {
        a.push(
            buf.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Box<_>>(),
        );
        buf.clear();
    }
    let a = a.into_boxed_slice();
    let n = a.len();
    let m = a[0].len();

    let mut q = VecDeque::new();
    let mut dp = boxed_slice![boxed_slice![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 0 {
                q.push_back((i, j));
                dp[i][j] = 1;
            }
        }
    }

    let mut res = 0;
    while let Some((i, j)) = q.pop_front() {
        if dp[i][j] == -1 {
            continue;
        }
        let w = std::mem::replace(&mut dp[i][j], -1);
        if a[i][j] == 9 {
            res += w;
        }
        for (i1, j1) in [
            (i.checked_sub(1), Some(j)),
            (Some(i), j.checked_sub(1)),
            (Some(i + 1), Some(j)),
            (Some(i), Some(j + 1)),
        ] {
            if i1.is_none() || j1.is_none() {
                continue;
            }
            let i1 = i1.unwrap();
            let j1 = j1.unwrap();
            if i1 >= n || j1 >= m {
                continue;
            }
            if a[i1][j1] == a[i][j] + 1 {
                dp[i1][j1] += w;
                q.push_back((i1, j1));
            }
        }
    }
    println!("{res}");
}
