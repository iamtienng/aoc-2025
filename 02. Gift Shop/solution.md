# Solution Explanation

## Problem Overview

Find all numbers in given ranges that are formed by repeating a digit pattern multiple times (e.g., 123123, 5555, 101010).

## Algorithm

### Mathematical Foundation

For a base pattern `s` with `k` digits repeated `t` times:

```
n = s × ((10^(k×t) - 1) / (10^k - 1))
```

**Example:** Pattern `s = 123` (k=3 digits) repeated t=2 times:

- n = 123 × ((10^6 - 1) / (10^3 - 1))
- n = 123 × (999999 / 999)
- n = 123 × 1001 = 123123

Valid patterns have no leading zeros, so: `10^(k-1) ≤ s ≤ 10^k - 1`

### Part 1: Exactly 2 Repeats

**Constraint:** t = 2

**Process:**

1. For each pattern length k from 1 to max_digits/2:
   - Compute factor F = (10^(2k) - 1) / (10^k - 1)
   - For each range [a, b]:
     - Find valid s values: ceil(a/F) ≤ s ≤ floor(b/F)
     - Intersect with [10^(k-1), 10^k - 1]
     - Generate n = s × F for each valid s
2. Collect all unique generated numbers
3. Sum them

### Part 2: At Least 2 Repeats

**Constraint:** t ≥ 2

**Process:**

1. For each pattern length k from 1 to max_digits:
   - For each repeat count t from 2 to max_digits/k:
     - Compute factor F = (10^(k×t) - 1) / (10^k - 1)
     - For each range [a, b]:
       - Find valid s values: ceil(a/F) ≤ s ≤ floor(b/F)
       - Intersect with [10^(k-1), 10^k - 1]
       - Generate n = s × F for each valid s
2. Collect all unique generated numbers (using HashSet to avoid duplicates)
3. Sum them

### Key Optimizations

- **Direct generation:** Instead of checking every number in ranges, we directly compute which patterns yield valid numbers
- **No duplicates:** Using a HashSet ensures each number is counted once (e.g., 11 from k=1,t=2 won't be counted twice)
- **Overflow handling:** All arithmetic operations check for overflow since numbers can be very large (up to u128)
