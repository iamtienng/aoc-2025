use std::fs::File;
use std::io::{self, Read, Write};

fn parse_u128(s: &str) -> u128 {
    s.trim().parse::<u128>().expect("invalid number")
}

fn merge_ranges(mut ranges: Vec<(u128, u128)>) -> Vec<(u128, u128)> {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_by_key(|&(a, _)| a);
    let mut merged = Vec::with_capacity(ranges.len());
    let mut cur = ranges[0];
    for (a, b) in ranges.into_iter().skip(1) {
        if a <= cur.1 + 1 {
            // overlapping or touching
            if b > cur.1 {
                cur.1 = b;
            }
        } else {
            merged.push(cur);
            cur = (a, b);
        }
    }
    merged.push(cur);
    merged
}

fn read_ranges_only() -> io::Result<Vec<(u128, u128)>> {
    let mut buf = String::new();
    File::open("../input.txt")?.read_to_string(&mut buf)?;
    let mut lines = buf.lines();
    let mut ranges = Vec::new();

    // read until blank line
    for line in &mut lines {
        let s = line.trim();
        if s.is_empty() {
            break;
        }
        let mut parts = s.splitn(2, '-');
        let a = parse_u128(parts.next().unwrap());
        let b = parse_u128(parts.next().unwrap());
        let (l, r) = if a <= b { (a, b) } else { (b, a) };
        ranges.push((l, r));
    }
    Ok(ranges)
}

pub fn part_one() -> io::Result<()> {
    // read whole input
    let mut buf = String::new();
    File::open("../input.txt")?.read_to_string(&mut buf)?;
    let mut lines = buf.lines();

    // parse ranges
    let mut ranges = Vec::new();
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }
        let mut parts = line.splitn(2, '-');
        let a = parse_u128(parts.next().unwrap());
        let b = parse_u128(parts.next().unwrap());
        let (l, r) = if a <= b { (a, b) } else { (b, a) };
        ranges.push((l, r));
    }

    let merged = merge_ranges(ranges);

    // remaining lines: query IDs
    let mut count = 0u128;
    for line in lines {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        let id = parse_u128(s);
        // check membership by binary search
        let mut lo = 0usize;
        let mut hi = merged.len();
        let mut found = false;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let (a, b) = merged[mid];
            if id < a {
                hi = mid;
            } else if id > b {
                lo = mid + 1;
            } else {
                found = true;
                break;
            }
        }
        if found {
            count += 1;
        }
    }

    let mut out = File::create("../output_part_one.txt")?;
    writeln!(out, "{}", count)?;
    Ok(())
}

pub fn part_two() -> io::Result<()> {
    let ranges = read_ranges_only()?;
    let merged = merge_ranges(ranges);

    // sum of lengths of merged intervals
    let mut total = 0u128;
    for (a, b) in merged {
        total += b - a + 1;
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
