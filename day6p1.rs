use std::io::stdin;

fn boxed_slice<T: Clone>(value: T, len: usize) -> Box<[T]> {
    std::iter::repeat_n(value, len).collect()
}

fn main() {
    let mut lines: Vec<Box<[u8]>> = Vec::new();
    let mut buf = String::new();
    while let Ok(len) = stdin().read_line(&mut buf)
        && len != 0
    {
        lines.push(buf.as_bytes().into());
        buf.clear();
    }

    let n = lines.len();
    let m = lines[0].len();

    let mut vis = boxed_slice(boxed_slice([false; 4], m), n);
    let mut x = 0;
    let mut y = 0;
    let mut d = 0;
    'outer: for i in 0..n {
        for j in 0..m {
            if lines[i][j] == b'^' {
                (x, y) = (i, j);
                break 'outer;
            }
        }
    }

    let n = n as isize;
    let m = m as isize;

    loop {
        if vis[x][y][d] {
            break;
        }
        vis[x][y][d] = true;
        let (new_x, new_y) = match d {
            0 => (x as isize - 1, y as isize),
            1 => (x as isize, y as isize + 1),
            2 => (x as isize + 1, y as isize),
            3 => (x as isize, y as isize - 1),
            _ => unreachable!(),
        };
        if !(0..n).contains(&new_x) || !(0..m).contains(&new_y) {
            break;
        }
        let new_x = new_x as usize;
        let new_y = new_y as usize;
        if lines[new_x][new_y] == b'#' {
            d = (d + 1) % 4;
        } else {
            (x, y) = (new_x, new_y);
        }
    }

    let result = vis
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|v| v.contains(&true) as usize)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("{result}");
}
