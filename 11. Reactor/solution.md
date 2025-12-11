# Day 11: Reactor - Solution

## Part 1: Count All Paths

### Algorithm

The solution uses **dynamic programming with memoization** to count all paths from node `you` to node `out` in a directed graph.

**Key Steps:**

1. **Parse Input**: Build an adjacency list representation where each device maps to its list of output devices
2. **Recursive Path Counting**: For each node, recursively count paths to the target by summing paths from all its neighbors
3. **Memoization**: Cache results for each (node, target) pair to avoid redundant calculations
4. **Base Cases**:
   - If current node equals target, return 1 (found a complete path)
   - If node has no outputs and isn't the target, return 0 (dead end)

**Time Complexity**: O(V + E) with memoization, where V is vertices and E is edges  
**Space Complexity**: O(V²) for the memoization table

## Part 2: Count Paths Through Required Nodes

### Algorithm

The problem requires counting paths from `svr` to `out` that visit **both** `dac` and `fft` in any order.

**Key Insight**: Split into two scenarios based on visit order:

1. **Scenario 1**: svr → dac → fft → out
2. **Scenario 2**: svr → fft → dac → out

**Calculation:**

- For each scenario, multiply the number of paths between consecutive nodes:
  - Scenario 1: `P(svr→dac) × P(dac→fft) × P(fft→out)`
  - Scenario 2: `P(svr→fft) × P(fft→dac) × P(dac→out)`
- Sum both scenarios to get total paths

**Why This Works**: By forcing the path to go through both intermediate nodes in a specific order, we ensure both are visited. The multiplication principle applies because choosing a path in each segment is independent.

**Implementation Details:**

- Reuse the same memoized `count_paths_to_target` function from Part 1
- Use `checked_mul` and `checked_add` to handle potential overflow
- The memoization table is shared across all calculations for efficiency

**Time Complexity**: O(V + E) - same as Part 1, just 6 queries to the memoized function  
**Space Complexity**: O(V²) for memoization
