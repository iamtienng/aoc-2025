# Day 7: Laboratories - Solution Explanation

## Part 1: Classical Tachyon Beam Splitting

### Problem Summary

A tachyon beam starts at position 'S' and travels downward through a grid. When it encounters a splitter ('^'), the beam stops and two new beams emerge from the left and right positions adjacent to the splitter. We need to count the total number of splits.

### Algorithm

The solution uses a **column-tracking simulation** approach:

1. **Initialize**: Start with the column containing 'S'
2. **Iterate row-by-row**: For each row below the start:
   - For each active column, check the cell:
     - If it's a splitter ('^'): increment split counter, add left and right columns to active set
     - If it's empty ('.'): the beam continues straight down to the same column
3. **Track active columns**: Use a HashSet to avoid counting duplicate beams in the same column
4. **Terminate**: Stop when no active columns remain or we reach the bottom

**Time Complexity**: O(rows × cols)  
**Space Complexity**: O(cols) for the active columns set

---

## Part 2: Quantum Tachyon Splitting (Many-Worlds Interpretation)

### Problem Summary

In the quantum version, a single particle takes **both** paths at each splitter, creating parallel timelines. We need to count how many distinct timelines exist after the particle completes all possible journeys.

### Algorithm

This requires **dynamic programming with timeline counting**:

1. **State Definition**:
   - `timeline_counts[col]` = number of timelines reaching column `col` in the current row

2. **Base Case**:
   - One timeline starts at 'S' and continues to the row below

3. **Transition** (for each cell in the current row):
   - Timelines from empty cells above ('.'): continue straight down
   - Timelines from splitters above ('^'):
     - Left neighbor splitter contributes timelines going right
     - Right neighbor splitter contributes timelines going left
   - Sum all contributions to get the count at each column

4. **Optimization**:
   - Use HashMap for sparse storage (many columns have 0 timelines)
   - Only store previous and current row to save memory

5. **Result**: Sum of all timeline counts in the final row

**Key Insight**: Instead of tracking individual paths, we count how many timelines reach each position. When paths converge (multiple timelines reach the same cell), they still represent distinct timeline histories.

**Time Complexity**: O(rows × cols)  
**Space Complexity**: O(cols) for the timeline count maps

---

## Implementation Notes

- Both solutions parse the grid from the input file
- Part 1 tracks which columns have active beams using a HashSet
- Part 2 tracks the number of timelines at each column using a HashMap
- The code handles edge cases like splitters at grid boundaries
- Empty timeline maps trigger early termination for efficiency
