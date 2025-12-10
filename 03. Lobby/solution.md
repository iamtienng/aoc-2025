# Day 3: Lobby - Solution

## Problem Statement

Given a digit sequence D[0..n-1], pick a subsequence of length k that maximizes the lexicographical (thus numeric) value.

**Equivalent formulation**: Choose k digits, preserve original order, maximize result.

## Algorithm: Greedy Monotonic-Stack Method

Let `r = n - k`. We may delete exactly r digits.

Maintain a stack S (initially empty).

```
For each digit d in D:
    while S not empty AND r > 0 AND S.top < d:
        pop S
        r -= 1
    push d
```

After processing all digits:

- S may contain > k elements. Truncate to first k.

This yields the lexicographically maximum subsequence of length k.

## Solution Approach

### Part 1

`k = 2`. For each line (a battery bank):

```
best = max_subseq_k(D, 2)
convert to number and sum
```

### Part 2

`k = 12`. Same method:

```
best = max_subseq_k(D, 12)
convert to number and sum
```

## Correctness (Sketch)

- Any optimal solution is lexicographically maximal.
- Whenever `S.top < d` and d is available, keeping S.top cannot produce a lexicographically larger subsequence than replacing it with d.
- Each pop corresponds to removing one of the allowed r deletions.
- Removing the leftmost smaller digit always yields a lexicographically greater prefix; no later replacement can improve more.
- **Exchange argument**: If an optimal solution keeps a digit x where a later digit y > x exists and enough remaining capacity allows y to replace x, swapping yields a strictly larger subsequence; thus greedy pops are necessary.

Therefore the stack algorithm produces the unique lexicographically maximum subsequence of length k.
