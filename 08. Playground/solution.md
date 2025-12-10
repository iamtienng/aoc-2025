# Day 8: Playground - Solution Explanation

## Problem Summary

We have junction boxes positioned in 3D space. We need to connect them with strings of lights, where each connection allows electricity to flow between two boxes. Connected boxes form circuits.

## Algorithm Overview

Both parts use **Kruskal's Minimum Spanning Tree algorithm** with a **Disjoint Set Union (DSU)** data structure.

### Part 1: Connect 1000 Shortest Pairs

**Goal:** After making the 1000 shortest connections, multiply the sizes of the three largest circuits.

**Algorithm:**

1. **Generate all edges**: Create all possible pairs of junction boxes and calculate their squared Euclidean distances
2. **Sort edges**: Sort all edges by distance in ascending order
3. **Initialize DSU**: Start with each junction box in its own circuit
4. **Process 1000 edges**: For each of the 1000 shortest edges, attempt to union the two junction boxes
   - If they're already in the same circuit, nothing happens
   - If they're in different circuits, merge them into one circuit
5. **Find largest circuits**: Get all circuit sizes, sort them in descending order, and multiply the top three

**Time Complexity:** O(n² log n) where n is the number of junction boxes

- O(n²) to generate all edges
- O(n² log n²) to sort them
- O(1000 × α(n)) for DSU operations (effectively constant)

### Part 2: Connect Until One Circuit

**Goal:** Continue connecting until all junction boxes form a single circuit. Return the product of the X coordinates of the last two connected boxes.

**Algorithm:**

1. **Generate and sort edges**: Same as Part 1
2. **Initialize DSU**: Start with n separate circuits
3. **Process edges sequentially**:
   - For each edge (in sorted order), attempt to union the two junction boxes
   - Track the number of remaining circuits
   - When a successful union reduces the circuit count to 1, we've found our answer
4. **Return result**: Multiply the X coordinates of the last two connected junction boxes

**Time Complexity:** O(n² log n)

- Same generation and sorting as Part 1
- In worst case, process all edges until only one circuit remains

## Key Data Structures

**DSU (Disjoint Set Union):**

- **Path compression**: Optimizes find operations by flattening the tree structure
- **Union by size**: Attaches smaller trees under larger ones to keep trees shallow
- **Circuit tracking**: Maintains count of distinct circuits and size of each component

**Edge Structure:**

- Stores indices of two junction boxes and their squared distance
- Using squared distance avoids floating-point operations and maintains ordering

## Why This Works

This is essentially building a **Minimum Spanning Forest** incrementally. By always connecting the closest unconnected junction boxes, we minimize the total string light length while ensuring all boxes can eventually be powered. The DSU efficiently tracks which boxes are already connected, preventing redundant connections.
