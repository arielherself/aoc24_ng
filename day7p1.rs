use std::io::stdin;

fn boxed_slice<T: Clone>(value: T, len: usize) -> Box<[T]> {
    std::iter::repeat_n(value, len).collect()
}

fn check(target: isize, curr: isize, operands: &[isize]) -> bool {
    if operands.is_empty() {
        return target == curr;
    }

    for next in [
        curr.saturating_add(operands[0]),
        curr.saturating_mul(operands[0]),
    ] {
        if check(target, next, &operands[1..]) {
            return true;
        }
    }

    false
}

fn main() {
    let mut buf = String::new();
    let mut res = 0;
    while let Ok(len) = stdin().read_line(&mut buf)
        && len != 0
    {
        let (left, right) = buf.split_once(':').unwrap();
        let left = left.parse().unwrap();
        let right = right
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect::<Box<[isize]>>();
        if check(left, right[0], &right[1..]) {
            res += left;
        }
        buf.clear();
    }
    println!("{res}");
}
