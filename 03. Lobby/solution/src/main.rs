use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn digits_from_str(s: &str) -> Vec<u8> {
    s.bytes()
        .map(|b| {
            if !(b'0'..=b'9').contains(&b) {
                panic!("invalid character in input");
            }
            b - b'0'
        })
        .collect()
}

// Generic: lấy subsequence lớn nhất độ dài k
fn max_subseq_k(digits: &[u8], k: usize) -> Vec<u8> {
    let n = digits.len();
    assert!(n >= k, "line has fewer than {} digits", k);

    let mut stack: Vec<u8> = Vec::with_capacity(k);
    let mut to_remove = n - k;

    for &d in digits {
        while !stack.is_empty() && to_remove > 0 && *stack.last().unwrap() < d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }

    stack.truncate(k);
    stack
}

/* ---------------------- PART 1 ------------------------- */

fn part_one() -> io::Result<()> {
    const K: usize = 2;

    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut total: u128 = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        let s = line.trim();
        if s.is_empty() {
            continue;
        }

        let digits = digits_from_str(s);
        if digits.len() < K {
            panic!("Line shorter than {}", K);
        }

        let best = max_subseq_k(&digits, K);

        let mut num: u128 = 0;
        for &d in &best {
            num = num * 10 + (d as u128);
        }

        total += num;
    }

    let mut out = File::create("../output_part_one.txt")?;
    writeln!(out, "{}", total)?;
    Ok(())
}

/* ---------------------- PART 2 ------------------------- */

fn part_two() -> io::Result<()> {
    const K: usize = 12;

    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut total: u128 = 0;

    for line_res in reader.lines() {
        let line = line_res?;
        let s = line.trim();
        if s.is_empty() {
            continue;
        }

        let digits = digits_from_str(s);
        if digits.len() < K {
            panic!("Line shorter than {}", K);
        }

        let best = max_subseq_k(&digits, K);

        let mut num: u128 = 0;
        for &d in &best {
            num = num * 10 + (d as u128);
        }

        total += num;
    }

    let mut out = File::create("../output_part_two.txt")?;
    writeln!(out, "{}", total)?;
    Ok(())
}

/* ---------------------- MAIN ------------------------- */

fn main() -> io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
