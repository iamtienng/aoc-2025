# Day 9: Movie Theater - Solution

## Part 1: Largest Rectangle with Red Corner Tiles

### Algorithm

The goal is to find the largest rectangle that has red tiles at two opposite corners.

**Approach:**

1. Parse all red tile coordinates from the input
2. Brute-force check all pairs of red tiles as potential opposite corners
3. For each pair of red tiles at positions `(x1, y1)` and `(x2, y2)`:
   - Calculate the rectangle dimensions:
     - Width: `|x1 - x2| + 1` (inclusive)
     - Height: `|y1 - y2| + 1` (inclusive)
   - Calculate area: `width × height`
4. Track the maximum area found

**Time Complexity:** O(n²) where n is the number of red tiles

**Key Insight:** In Part 1, we don't need to validate what's inside the rectangle - any two red tiles can serve as opposite corners.

---

## Part 2: Largest Rectangle with Only Red/Green Tiles

### Algorithm

Now rectangles can only contain red or green tiles. Green tiles form:

- Straight lines connecting consecutive red tiles in the input list (forming a polygon boundary)
- All tiles strictly inside the polygon formed by red tiles

**Approach:**

1. **Pre-calculate Boundary Points:**
   - Walk along each edge of the polygon (between consecutive red tiles)
   - Store all boundary points (red and green) in a HashSet for O(1) lookup

2. **Validate Each Rectangle:**
   For each pair of red tiles as potential corners:

   a. **Check Non-Red Corners:**
   - The other two corners must be "allowed" (either on boundary or strictly inside)

   b. **Check Interior Containment:**
   - Use ray casting algorithm to verify an interior point is inside the polygon
   - Cast a horizontal ray from a test point to infinity
   - Count intersections with vertical polygon edges
   - Odd count = inside, even count = outside

   c. **Check Boundary Crossing:**
   - Ensure no polygon edge crosses through the rectangle's open interior
   - Check both horizontal and vertical segments
   - A crossing occurs when a segment passes through the interior without touching corners

3. **Calculate Maximum Area:**
   - Among all valid rectangles, find the one with maximum area

**Time Complexity:** O(n³) where n is the number of red tiles

- O(n²) pairs to check
- O(n) validation per pair (ray casting + boundary checking)

**Key Insights:**

- The polygon formed by red tiles is rectilinear (only horizontal/vertical edges)
- Ray casting works by counting vertical edge crossings
- Edge case handling: 1×N rectangles only need corner validation
- Interior points must be strictly inside (not just on boundary)
- Boundary segments must not slice through the rectangle's interior
