use std::{
    collections::BTreeSet,
    io::{Read as _, stdin},
};

fn boxed_slice<T: Clone>(value: T, len: usize) -> Box<[T]> {
    std::iter::repeat_n(value, len).collect()
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

    let mut b = boxed_slice(-1, tot);
    let mut ptr = 0;
    let mut oc = BTreeSet::new();
    let mut ep = BTreeSet::new();
    for (i, &x) in a.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..x {
                b[ptr] = i as isize / 2;
                oc.insert(ptr);
                ptr += 1;
            }
        } else {
            for _ in 0..x {
                ep.insert(ptr);
                ptr += 1;
            }
        }
    }
    debug_assert_eq!(ptr, tot);

    while !oc.is_empty() && !ep.is_empty() {
        let x = oc.pop_last().unwrap();
        let y = ep.pop_first().unwrap();
        if y > x {
            break;
        }
        oc.insert(y);
        ep.insert(x);

        b.swap(x, y);
    }

    let mut res = 0;
    for (i, x) in b.into_iter().enumerate() {
        if x == -1 {
            break;
        }
        res += i as isize * x;
    }
    println!("{res}");
}
