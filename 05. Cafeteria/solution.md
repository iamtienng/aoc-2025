# Day 5: Cafeteria - Solution

## Part 1: Counting Fresh Available Ingredients

**Algorithm:**

1. Parse the fresh ingredient ID ranges from the input file
2. Merge overlapping or touching ranges into a sorted list of non-overlapping intervals
3. For each available ingredient ID in the second section, check if it falls within any merged range using binary search
4. Count how many ingredient IDs are found to be fresh

**Key Insight:** By merging overlapping ranges first, we reduce the number of comparisons needed and can efficiently check membership using binary search on the sorted merged ranges.

**Time Complexity:** O(n log n) for sorting and merging ranges, O(m log n) for checking m ingredient IDs against n merged ranges.

## Part 2: Counting All Fresh Ingredient IDs

**Algorithm:**

1. Parse only the fresh ingredient ID ranges (ignore the available IDs section)
2. Merge overlapping or touching ranges into non-overlapping intervals
3. Sum the length of each merged interval (end - start + 1) to get the total count of fresh IDs

**Key Insight:** After merging ranges, the total number of fresh ingredient IDs is simply the sum of all interval lengths. No need to enumerate individual IDs.

**Time Complexity:** O(n log n) for sorting and merging ranges, O(n) for summing interval lengths.

## Core Function: Range Merging

The `merge_ranges` function is central to both parts:

- Sorts ranges by start position
- Iteratively merges overlapping or adjacent ranges (when `new_start <= current_end + 1`)
- Returns a minimal set of non-overlapping intervals covering the same IDs

This preprocessing step makes both membership checking (Part 1) and counting (Part 2) efficient.
