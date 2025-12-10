# Day 10: Factory - Solution Explanation

## Part 1: Indicator Light Configuration (Binary System)

### Problem

Configure indicator lights (binary on/off) by pressing buttons that toggle specific lights. Find the minimum number of button presses needed.

### Algorithm: Gaussian Elimination over GF(2)

The problem is modeled as solving a linear system over the binary field GF(2):

- **A·x = t (mod 2)**
  - `A` = button-to-light matrix (A[i][j] = 1 if button j toggles light i)
  - `x` = number of presses for each button (mod 2, so 0 or 1)
  - `t` = target state vector (0 = off, 1 = on)

**Steps:**

1. **Parse Input**: Extract target state and button configurations
2. **Gaussian Elimination**: Row-reduce the augmented matrix [A | t] in GF(2)
   - Find pivot columns (basic variables)
   - Identify free variables (non-pivot columns)
3. **Solution Space**:
   - Find particular solution `p0` (set free variables to 0)
   - Generate null space basis vectors (one per free variable)
4. **Optimization**: Try all 2^k combinations of null space basis vectors
   - Each combination gives a valid solution
   - Select the one with minimum Hamming weight (fewest 1s = fewest presses)

**Complexity**: O(n²m) for Gaussian elimination + O(2^k · m) for optimization where k = number of free variables

## Part 2: Joltage Counter Configuration (Integer System)

### Problem

Increment counters to reach target values by pressing buttons that add 1 to specific counters. Find the minimum number of button presses needed.

### Algorithm: Integer Linear Programming (ILP)

The problem is formulated as an ILP optimization:

- **Minimize**: Σ x_j (total button presses)
- **Subject to**: For each counter i: Σ A[i][j] · x_j = target[i]
- **Constraints**: x_j ≥ 0, x_j ∈ ℤ (non-negative integers)

**Steps:**

1. **Parse Input**: Extract target values and button configurations
2. **Build ILP Model**:
   - Variables: x_j = number of times to press button j
   - Objective: minimize sum of all x_j
   - Constraints: one equation per counter
3. **Solve**: Use CBC solver (via PuLP library)
4. **Extract Solution**: Sum all variable values to get minimum presses

**Why ILP?**: Unlike Part 1 (binary/mod 2), Part 2 requires exact integer counts. ILP efficiently handles this with constraint satisfaction and optimization.

**Complexity**: ILP solving is NP-hard in general, but practical instances solve quickly with modern solvers like CBC.

## Key Differences

| Aspect                  | Part 1                                   | Part 2                     |
| ----------------------- | ---------------------------------------- | -------------------------- |
| **Domain**              | Binary (GF(2))                           | Non-negative integers      |
| **Operation**           | Toggle (XOR)                             | Increment (+1)             |
| **Method**              | Gaussian elimination + exhaustive search | Integer Linear Programming |
| **Solution uniqueness** | Multiple solutions (null space)          | Unique optimal solution    |
