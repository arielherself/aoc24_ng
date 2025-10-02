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

struct QuickUnion<T: Copy + Default + std::ops::AddAssign> {
    c: Box<[usize]>,
    sz: Box<[usize]>,
    data: Box<[T]>,
}

impl<T: Copy + Default + std::ops::AddAssign> QuickUnion<T> {
    fn new(n: usize) -> Self {
        Self {
            c: (0..n).collect(),
            sz: boxed_slice![1; n],
            data: boxed_slice![Default::default(); n],
        }
    }

    fn query(&mut self, mut i: usize) -> usize {
        while i != self.c[i] {
            self.c[i] = self.c[self.c[i]];
            i = self.c[i];
        }
        i
    }

    fn merge(&mut self, i: usize, j: usize) {
        if self.connected(i, j) {
            return;
        }
        self.sz[self.query(j)] += self.sz[self.query(i)];
        let prev = self.data[self.query(i)];
        self.data[self.query(j)] += prev;
        self.c[self.query(i)] = self.c[self.query(j)];
    }

    fn connected(&mut self, i: usize, j: usize) -> bool {
        self.query(i) == self.query(j)
    }

    fn query_size(&mut self, i: usize) -> usize {
        self.sz[self.query(i)]
    }

    fn query_data(&mut self, i: usize) -> T {
        self.data[self.query(i)]
    }

    fn set_data(&mut self, i: usize, value: T) {
        self.data[self.query(i)] = value
    }
}

fn main() {
    let mut buf = String::new();
    let mut a = Vec::new();
    while let Ok(len) = stdin().read_line(&mut buf)
        && len != 0
    {
        let line = std::mem::take(&mut buf);
        a.push(
            line.trim()
                .as_bytes()
                .iter()
                .copied()
                .collect::<Box<[u8]>>(),
        );
    }

    let n = a.len();
    let m = a[0].len();

    let mut qu = QuickUnion::<usize>::new(n * m);
    for i in 0..n {
        for j in 0..m {
            let mut curr = 0;
            for (i1, j1) in [
                (i.checked_sub(1), Some(j)),
                (Some(i + 1), Some(j)),
                (Some(i), j.checked_sub(1)),
                (Some(i), Some(j + 1)),
            ] {
                if i1.is_none() || j1.is_none() {
                    curr += 1;
                    continue;
                }
                let i1 = i1.unwrap();
                let j1 = j1.unwrap();
                if i1 >= n || j1 >= m {
                    curr += 1;
                    continue;
                }

                if a[i1][j1] == a[i][j] {
                    qu.merge(i * m + j, i1 * m + j1);
                } else {
                    curr += 1;
                }
            }
            let prev = qu.query_data(i * m + j);
            qu.set_data(i * m + j, prev + curr);
        }
    }

    let mut res = 0;
    for i in 0..n * m {
        if qu.query(i) != i {
            continue;
        }
        eprintln!(
            "({}, {}): data = {}, size = {}",
            i / m,
            i % m,
            qu.query_data(i),
            qu.query_size(i)
        );
        res += qu.query_data(i) * qu.query_size(i);
    }
    println!("{res}");
}
