use std::io::stdin;

fn boxed_slice<T: Clone>(value: T, len: usize) -> Box<[T]> {
    std::iter::repeat_n(value, len).collect()
}

macro_rules! unwrap_or_continue {
    ($x: expr) => {{
        if $x.is_none() {
            continue;
        }
        unsafe { $x.unwrap_unchecked() }
    }};
}

fn main() {
    let mut buf = String::new();
    let mut grid = Vec::<Box<[u8]>>::new();
    while let Ok(len) = stdin().read_line(&mut buf)
        && len != 0
    {
        grid.push(buf.trim().as_bytes().into());
        buf.clear();
    }

    let n = grid.len();
    let m = grid[0].len();
    eprintln!("{n}, {m}");

    let mut vis = boxed_slice(boxed_slice(false, m), n);

    for i1 in 0..n {
        for j1 in 0..m {
            if grid[i1][j1] == b'.' {
                continue;
            }
            for i2 in 0..n {
                for j2 in 0..m {
                    if i1 == i2 && j1 == j2 {
                        continue;
                    }

                    if grid[i1][j1] != grid[i2][j2] {
                        continue;
                    }

                    let i3 = unwrap_or_continue!((2 * i2).checked_sub(i1));
                    let j3 = unwrap_or_continue!((2 * j2).checked_sub(j1));
                    if i3 >= n || j3 >= m {
                        continue;
                    }

                    vis[i3][j3] = true;
                }
            }
        }
    }

    let res = vis
        .into_iter()
        .map(|v| v.into_iter().map(|x| x as usize).sum::<usize>())
        .sum::<usize>();

    println!("{res}");
}
