# Solution: Forklift Paper Roll Access

## Problem Overview

The grid contains paper rolls marked with `@`. A roll is **accessible** to forklifts if it has fewer than 4 neighboring `@` symbols in the 8 adjacent directions (including diagonals).

**Neighbor offsets:**

```
(-1,-1), (-1,0), (-1,1)
(0,-1),         (0,1)
(1,-1),  (1,0), (1,1)
```

---

## Part 1: Count Initially Accessible Rolls

**Algorithm:**

1. For each cell `(i, j)` in the grid:
   - Skip if `grid[i][j] != '@'`
   - Count neighbors: `deg(i,j) = number of adjacent positions containing '@'`
   - If `deg(i,j) < 4`, the roll is accessible

2. Sum all accessible rolls

**Complexity:** O(rows × cols × 8) = O(n) where n is grid size

**Key insight:** This is a pure local check. The grid never changes, and each roll is evaluated independently.

---

## Part 2: Iterative Removal Process

Once a roll becomes accessible (`deg < 4`), it can be removed. Removing a roll decreases the degree of its neighbors, potentially making _them_ accessible. This creates a cascading "peeling" effect.

**Algorithm (Topological Sort / BFS-based peeling):**

1. **Initialization:**
   - Compute `deg(i,j)` for all `@` cells
   - Mark all `@` cells as `alive`
   - Enqueue all cells where `deg < 4` (initially accessible)

2. **Process queue:**

   ```
   while queue is not empty:
       pop (i, j)
       if not alive[i][j]: continue

       alive[i][j] = false
       removed_count += 1

       for each neighbor (ni, nj):
           if alive[ni][nj]:
               deg[ni][nj] -= 1
               if deg[ni][nj] < 4:
                   enqueue (ni, nj)
   ```

3. **Termination:** Queue empties when no more rolls can be removed

**Complexity:** O(n) — each cell is processed at most once

**Key insight:** The grid "shrinks" layer by layer. Rolls on the periphery (with fewer neighbors) are removed first, exposing inner rolls until reaching a stable core where all remaining rolls have `deg ≥ 4`.

---

## Why This Works

- **Part 1:** Static snapshot — count rolls satisfying the accessibility condition
- **Part 2:** Dynamic process — simulate the actual removal sequence using a queue to track newly accessible rolls as the grid changes

The queue-based approach ensures we process rolls in the correct order without redundant checks.
