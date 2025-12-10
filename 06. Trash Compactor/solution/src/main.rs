use std::fs::File;
use std::io::{self, Read, Write};

fn parse_u128(s: &str) -> u128 {
    s.trim().parse::<u128>().expect("invalid number")
}

pub fn part_one() -> io::Result<()> {
    // Read whole file
    let mut src = String::new();
    File::open("../input.txt")?.read_to_string(&mut src)?;

    // Split into lines (preserve spacing)
    let mut lines: Vec<String> = src.lines().map(|l| l.to_string()).collect();

    // Remove trailing blank lines
    while let Some(last) = lines.last() {
        if last.trim().is_empty() {
            lines.pop();
        } else {
            break;
        }
    }

    if lines.is_empty() {
        let mut out = File::create("../output_part_one.txt")?;
        writeln!(out, "0")?;
        return Ok(());
    }

    // Bottom line is operator row
    let op_line = lines.pop().unwrap();

    // Number rows above it
    let num_rows = lines;

    // Determine max width across all rows
    let mut width = op_line.chars().count();
    for ln in &num_rows {
        width = width.max(ln.chars().count());
    }

    // Pad number rows
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(num_rows.len());
    for ln in &num_rows {
        let mut v: Vec<char> = ln.chars().collect();
        if v.len() < width {
            v.extend(std::iter::repeat(' ').take(width - v.len()));
        }
        grid.push(v);
    }

    // Pad operator row
    let mut op_chars: Vec<char> = op_line.chars().collect();
    if op_chars.len() < width {
        op_chars.extend(std::iter::repeat(' ').take(width - op_chars.len()));
    }

    // Detect used columns
    let mut used = vec![false; width];
    for c in 0..width {
        for r in 0..grid.len() {
            if grid[r][c] != ' ' {
                used[c] = true;
                break;
            }
        }
    }

    // Extract problems left → right
    let mut col = 0;
    let mut grand_total: u128 = 0;

    while col < width {
        if !used[col] {
            col += 1;
            continue;
        }

        let start = col;
        let mut end = col;
        while end + 1 < width && used[end + 1] {
            end += 1;
        }

        // Collect numbers (top → bottom)
        let mut numbers: Vec<u128> = Vec::new();
        for r in 0..grid.len() {
            let slice: String = grid[r][start..=end].iter().collect();
            let trimmed = slice.trim();
            if !trimmed.is_empty() {
                numbers.push(parse_u128(trimmed));
            }
        }

        // Operator for this block
        let mut op = None;
        for c in start..=end {
            let ch = op_chars[c];
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.expect("missing operator");

        // Evaluate
        let mut result = if op == '+' { 0u128 } else { 1u128 };

        match op {
            '+' => {
                for v in numbers {
                    result += v;
                }
            }
            '*' => {
                for v in numbers {
                    result *= v;
                }
            }
            _ => unreachable!(),
        }

        grand_total += result;

        col = end + 1;
    }

    let mut out = File::create("../output_part_one.txt")?;
    writeln!(out, "{}", grand_total)?;
    Ok(())
}

pub fn part_two() -> io::Result<()> {
    // Read whole file
    let mut src = String::new();
    File::open("../input.txt")?.read_to_string(&mut src)?;

    let mut lines: Vec<String> = src.lines().map(|l| l.to_string()).collect();
    while let Some(last) = lines.last() {
        if last.trim().is_empty() {
            lines.pop();
        } else {
            break;
        }
    }

    if lines.is_empty() {
        let mut out = File::create("../output_part_two.txt")?;
        writeln!(out, "0")?;
        return Ok(());
    }

    // Operator row
    let op_line = lines.pop().unwrap();
    let num_rows = lines;

    // Compute max width
    let mut width = op_line.chars().count();
    for ln in &num_rows {
        width = width.max(ln.chars().count());
    }

    // Pad
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(num_rows.len());
    for ln in &num_rows {
        let mut v: Vec<char> = ln.chars().collect();
        if v.len() < width {
            v.extend(std::iter::repeat(' ').take(width - v.len()));
        }
        grid.push(v);
    }

    let mut op_chars: Vec<char> = op_line.chars().collect();
    if op_chars.len() < width {
        op_chars.extend(std::iter::repeat(' ').take(width - op_chars.len()));
    }

    // Detect used columns
    let mut used = vec![false; width];
    for c in 0..width {
        for r in 0..grid.len() {
            if grid[r][c] != ' ' {
                used[c] = true;
                break;
            }
        }
    }

    // NOW: problems must be read RIGHT → LEFT
    let mut col: isize = (width as isize) - 1;
    let mut total: u128 = 0;

    while col >= 0 {
        if !used[col as usize] {
            col -= 1;
            continue;
        }

        let start = col;
        let mut end = col;
        while end - 1 >= 0 && used[(end - 1) as usize] {
            end -= 1;
        }

        let left = end as usize;
        let right = start as usize;

        // Collect digits as *vertical columns*
        let mut numbers: Vec<String> = Vec::new();

        for c in left..=right {
            let mut s = String::new();
            for r in 0..grid.len() {
                let ch = grid[r][c];
                if ch.is_ascii_digit() {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                numbers.push(s);
            }
        }

        // Parse to u128
        let nums: Vec<u128> = numbers.iter().map(|s| parse_u128(s)).collect();

        // Operator
        let mut op = None;
        for c in left..=right {
            let ch = op_chars[c];
            if ch == '+' || ch == '*' {
                op = Some(ch);
                break;
            }
        }
        let op = op.expect("missing operator");

        let mut result = if op == '+' { 0 } else { 1 };

        match op {
            '+' => {
                for v in nums {
                    result += v;
                }
            }
            '*' => {
                for v in nums {
                    result *= v;
                }
            }
            _ => unreachable!(),
        }

        total += result;

        col = (left as isize) - 1;
    }

    let mut out = File::create("../output_part_two.txt")?;
    writeln!(out, "{}", total)?;
    Ok(())
}

fn main() -> io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
