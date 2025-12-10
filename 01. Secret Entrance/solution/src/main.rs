use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// Part 1: Count zero hits only at final positions
fn part_one() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut pos = 50;
    let mut count_zero = 0i64;

    for line in reader.lines() {
        let s = line?.trim().to_string();
        if s.is_empty() {
            continue;
        }

        let mut chars = s.chars();
        let dir = chars.next().unwrap();
        let dist_str: String = chars.collect();
        let dist: i64 = dist_str.trim().parse().unwrap();

        match dir {
            'R' | 'r' => pos = (pos + dist).rem_euclid(100),
            'L' | 'l' => pos = (pos - dist).rem_euclid(100),
            _ => panic!("Invalid direction '{}'", dir),
        }

        if pos == 0 {
            count_zero += 1;
        }
    }

    let mut out = File::create("../output_part_one.txt")?;
    writeln!(out, "{}", count_zero)?;

    Ok(())
}

// Part 2: Count zero hits at ALL intermediate clicks
fn part_two() -> io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    let mut pos: i64 = 50;
    let mut total_zero_hits = 0i64;

    for line in reader.lines() {
        let s = line?.trim().to_string();
        if s.is_empty() {
            continue;
        }

        let mut chars = s.chars();
        let dir = chars.next().unwrap();
        let dist_str: String = chars.collect();
        let dist: i64 = dist_str.trim().parse().unwrap();

        let d: i64 = match dir {
            'R' | 'r' => 1,
            'L' | 'l' => -1,
            _ => panic!("Invalid direction '{}'", dir),
        };

        // Count intermediate hits on zero
        let pos_mod = pos.rem_euclid(100);

        // find minimal positive i such that (pos + d*i) mod 100 = 0
        let i0 = if d == 1 {
            (100 - pos_mod) % 100
        } else {
            pos_mod
        };

        let minimal_i = if i0 == 0 { 100 } else { i0 };

        if minimal_i <= dist {
            let count = 1 + (dist - minimal_i) / 100;
            total_zero_hits += count;
        }

        // update final position
        pos = (pos + d * dist).rem_euclid(100);
    }

    let mut out = File::create("../output_part_two.txt")?;
    writeln!(out, "{}", total_zero_hits)?;

    Ok(())
}

fn main() -> io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
