use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

fn part_one() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    let grid: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| line.unwrap().bytes().collect())
        .collect();

    if grid.is_empty() {
        File::create("../output_part_one.txt")?.write_all(b"0\n")?;
        return Ok(());
    }

    let h = grid.len();
    let w = grid[0].len();

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut accessible = 0u128;

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != b'@' {
                continue;
            }

            let mut neigh = 0;
            for &(di, dj) in &dirs {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if ni < 0 || nj < 0 {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                if ni >= h || nj >= w {
                    continue;
                }
                if grid[ni][nj] == b'@' {
                    neigh += 1;
                }
            }

            if neigh < 4 {
                accessible += 1;
            }
        }
    }

    let mut out = File::create("../output_part_one.txt")?;
    writeln!(out, "{}", accessible)?;
    Ok(())
}

fn part_two() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    let grid: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| line.unwrap().bytes().collect())
        .collect();

    let h = grid.len();
    if h == 0 {
        File::create("../output_part_two.txt")?.write_all(b"0\n")?;
        return Ok(());
    }
    let w = grid[0].len();

    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    // degree and alive mask
    let mut deg = vec![vec![0u8; w]; h];
    let mut alive = vec![vec![false; w]; h];

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == b'@' {
                alive[i][j] = true;

                let mut d = 0;
                for &(di, dj) in &dirs {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni < 0 || nj < 0 {
                        continue;
                    }
                    let (ni, nj) = (ni as usize, nj as usize);
                    if ni < h && nj < w && grid[ni][nj] == b'@' {
                        d += 1;
                    }
                }
                deg[i][j] = d;
            }
        }
    }

    let mut queue = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if alive[i][j] && deg[i][j] < 4 {
                queue.push_back((i, j));
            }
        }
    }

    let mut removed = 0u64;

    while let Some((i, j)) = queue.pop_front() {
        if !alive[i][j] {
            continue;
        }

        alive[i][j] = false;
        removed += 1;

        for &(di, dj) in &dirs {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni < 0 || nj < 0 {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if ni < h && nj < w && alive[ni][nj] {
                if deg[ni][nj] > 0 {
                    deg[ni][nj] -= 1;
                }
                if deg[ni][nj] < 4 {
                    queue.push_back((ni, nj));
                }
            }
        }
    }

    let mut out = File::create("../output_part_two.txt")?;
    writeln!(out, "{}", removed)?;
    Ok(())
}

fn main() -> io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
