# Advent of Code - Day 1: Secret Entrance

## Problem Summary

A dial with values 0–99 (mod 100) starts at position 50. Given a sequence of rotations (L for left/decrement, R for right/increment), we need to:
- **Part 1**: Count how many times the dial lands on 0 after completing a rotation
- **Part 2**: Count how many times the dial passes through 0 during all rotations (including intermediate clicks)

## Algorithm

### Part 1: Final Position Counting

For each instruction with direction `d` (where `d = +1` for R, `d = -1` for L) and distance `k`:

1. Calculate final position: `pos = (pos + d*k) mod 100`
2. If `pos == 0`, increment counter
3. Continue with updated position

This is straightforward modular arithmetic—we only care about where the dial ends up after each complete rotation.

### Part 2: Intermediate Click Counting

For each instruction, we need to count every click `i` (where `1 ≤ i ≤ k`) such that:
```
(pos + d*i) mod 100 = 0
```

**Finding the first hit:**
- For `d = +1` (right): `i₀ = (100 - pos_mod) % 100`
- For `d = -1` (left): `i₀ = pos_mod`
- Special case: if `i₀ = 0`, set `i₀ = 100` (next hit is a full cycle away)

**Counting all hits:**

If `i₀ ≤ k` (the first hit occurs within this rotation):
```
number_of_hits = 1 + (k - i₀) // 100
```

This formula accounts for:
- The first hit at position `i₀`
- Additional hits every 100 clicks thereafter

**Example:**
- Position 50, rotating R60:
  - First hit: `i₀ = (100 - 50) % 100 = 50`
  - Since 50 ≤ 60: `hits = 1 + (60 - 50) // 100 = 1 + 0 = 1`
  - The dial passes through 0 once at click 50

- Position 82, rotating L68:
  - First hit: `i₀ = 82`
  - Since 82 > 68: no hits during this rotation

After counting, update position: `pos = (pos + d*k) mod 100`

## Complexity

- **Time**: O(n) where n is the number of instructions
- **Space**: O(1)

Both parts process each instruction once with constant-time arithmetic operations.
