use std::io::stdin;

fn boxed_slice<T: Clone>(value: T, len: usize) -> Box<[T]> {
    std::iter::repeat_n(value, len).collect()
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

                    let i1 = i1 as isize;
                    let i2 = i2 as isize;
                    let j1 = j1 as isize;
                    let j2 = j2 as isize;

                    let id = i2 - i1;
                    let jd = j2 - j1;

                    let mut i = i2;
                    let mut j = j2;

                    while (0..n as isize).contains(&i) && (0..m as isize).contains(&j) {
                        vis[i as usize][j as usize] = true;

                        i += id;
                        j += jd;
                    }
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
