#![feature(btree_cursors)]

use std::{
    collections::BTreeSet,
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
    let a = buf
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Box<_>>();

    let tot = a.iter().sum::<usize>();

    let mut b = boxed_slice![-1; tot];
    let mut oc = BTreeSet::new();
    let mut ep = BTreeSet::new();
    let mut eps = boxed_slice![BTreeSet::new(); 11];
    let mut ptr = 0;
    for (i, &x) in a.iter().enumerate() {
        if i % 2 == 0 {
            oc.insert((ptr, x));
            for _ in 0..x {
                b[ptr] = i as isize / 2;
                ptr += 1;
            }
        } else {
            ep.insert((ptr, x));
            eps[x].insert((ptr, x));
            ptr += x;
        }
    }

    while let Some((i1, x1)) = oc.pop_last() {
        let mut p = (usize::MAX, usize::MAX);
        for t in &mut eps[x1..] {
            if let Some((i0, x0)) = t.first().cloned()
                && i0 < i1
            {
                p = p.min((i0, x0));
            }
        }

        if p.0 != usize::MAX {
            let (i0, x0) = p;
            eps[p.1.min(10)].pop_first();
            {
                let x = b[i1];
                b[i0..i0 + x1].fill(x);
            }
            b[i1..i1 + x1].fill(-1);
            let rem = x0 - x1;
            oc.insert((i0, x1));
            ep.remove(&(i0, x0));
            if rem != 0 {
                ep.insert((i0 + x1, rem));
                eps[rem.min(10)].insert((i0 + x1, rem));
            }
            oc.remove(&(i1, x1));
            let mut l = i1;
            let mut r = i1 + x1;
            if let Some((i2, x2)) = ep
                .lower_bound(std::ops::Bound::Included(&(i1, 0)))
                .next()
                .cloned()
                && i2 == i1 + x1
            {
                ep.remove(&(i2, x2));
                eps[x2.min(10)].remove(&(i2, x2));
                r = i2 + x2;
            }
            if let Some((i2, x2)) = ep
                .upper_bound(std::ops::Bound::Excluded(&(i1, 0)))
                .prev()
                .cloned()
                && i2 + x2 == i1
            {
                ep.remove(&(i2, x2));
                eps[x2.min(10)].remove(&(i2, x2));
                l = i2;
            }
            ep.insert((l, r - l));
            eps[(r - l).min(10)].insert((l, r - l));
        }
    }

    let mut res = 0;
    for (i, x) in b.into_iter().enumerate() {
        res += i as isize * x.max(0);
    }
    println!("{res}");
}
