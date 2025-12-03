use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

type Num = u128;

/// safe pow10 that returns None on overflow
fn pow10_checked(exp: u32) -> Option<Num> {
    let mut v: Num = 1;
    for _ in 0..exp {
        if v > Num::MAX / 10 {
            return None;
        }
        v *= 10;
    }
    Some(v)
}

fn parse_ranges(line: &str) -> Vec<(Num, Num)> {
    line.split(',')
        .filter_map(|part| {
            let p = part.trim();
            if p.is_empty() {
                return None;
            }
            let mut it = p.splitn(2, '-');
            let a = it.next()?.trim().parse::<Num>().ok()?;
            let b = it.next()?.trim().parse::<Num>().ok()?;
            Some(if a <= b { (a, b) } else { (b, a) })
        })
        .collect()
}

/// compute ((10^(k*t) - 1) / (10^k - 1))
fn compute_factor(k: u32, t: u32) -> Option<Num> {
    let ten_k = pow10_checked(k)?;
    let pow_kt = pow10_checked(k * t)?;
    let numerator = pow_kt.checked_sub(1)?;
    let denom = ten_k.checked_sub(1)?;
    Some(numerator / denom)
}

fn s_interval_for_range(a: Num, b: Num, factor: Num, s_min: Num, s_max: Num) -> Option<(Num, Num)> {
    let s_low = {
        let q = a / factor;
        if q * factor < a { q + 1 } else { q }
    };
    let s_high = b / factor;

    let l = s_low.max(s_min);
    let r = s_high.min(s_max);

    if l > r { None } else { Some((l, r)) }
}

fn generate_for_k_t(k: u32, repeats: u32, ranges: &[(Num, Num)], out_set: &mut HashSet<Num>) {
    let factor = match compute_factor(k, repeats) {
        Some(f) => f,
        None => return,
    };

    let s_min = match pow10_checked(k.saturating_sub(1)) {
        Some(v) => v,
        None => return,
    };
    let s_max = match pow10_checked(k) {
        Some(v) => v - 1,
        None => return,
    };

    for &(a, b) in ranges {
        let smallest_n = s_min.saturating_mul(factor);
        let largest_n = s_max.saturating_mul(factor);
        if smallest_n > b || largest_n < a {
            continue;
        }

        if let Some((l, r)) = s_interval_for_range(a, b, factor, s_min, s_max) {
            for s in l..=r {
                out_set.insert(s.saturating_mul(factor));
            }
        }
    }
}

fn sum_set(hs: &HashSet<Num>) -> Num {
    hs.iter().copied().sum()
}

/// Part 1
fn part_one(ranges: &[(Num, Num)], max_digits: u32) -> Num {
    let mut set_p1 = HashSet::new();

    // exactly 2 repeats → total digits = 2k → k ≤ max_digits/2
    for k in 1..=(max_digits / 2) {
        generate_for_k_t(k, 2, ranges, &mut set_p1);
    }

    sum_set(&set_p1)
}

/// Part 2
fn part_two(ranges: &[(Num, Num)], max_digits: u32) -> Num {
    let mut set_p2 = HashSet::new();

    // repeats >= 2, pattern length k arbitrary as long as k*t ≤ max_digits
    for k in 1..=max_digits {
        let t_max = max_digits / k;
        for t in 2..=t_max {
            generate_for_k_t(k, t, ranges, &mut set_p2);
        }
    }

    sum_set(&set_p2)
}

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let line = line.trim();
    let ranges = parse_ranges(line);

    if ranges.is_empty() {
        File::create("../output_part_one.txt")?.write_all(b"0\n")?;
        File::create("../output_part_two.txt")?.write_all(b"0\n")?;
        return Ok(());
    }

    // determine max_digits from the largest upper bound
    let max_b = ranges.iter().map(|&(_, b)| b).max().unwrap_or(0);

    let max_digits = {
        let mut tmp = max_b;
        let mut d = 1u32;
        while tmp >= 10 {
            tmp /= 10;
            d += 1;
        }
        d
    };

    let sum1 = part_one(&ranges, max_digits);
    let sum2 = part_two(&ranges, max_digits);

    let mut out1 = File::create("../output_part_one.txt")?;
    writeln!(out1, "{}", sum1)?;

    let mut out2 = File::create("../output_part_two.txt")?;
    writeln!(out2, "{}", sum2)?;

    Ok(())
}
