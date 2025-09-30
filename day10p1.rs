#![feature(btree_cursors)]
#![allow(unused)]

use std::{
    collections::{BTreeSet, HashSet},
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

    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if a[i][j] != 0 {
                continue;
            }
            let mut q = vec![(i, j)];
            let mut st = HashSet::new();
            while let Some((i, j)) = q.pop() {
                if a[i][j] == 9 {
                    st.insert((i, j));
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
                        q.push((i1, j1));
                    }
                }
            }
            res += st.len();
        }
    }
    println!("{res}");
}
