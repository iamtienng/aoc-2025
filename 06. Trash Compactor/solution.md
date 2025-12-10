# Day 6: Trash Compactor - Solution

## Part 1: Left-to-Right Horizontal Reading

**Algorithm:**

1. Read the input file and parse it into lines
2. Separate the bottom line (operators) from the number rows above
3. Pad all rows to the same width with spaces
4. Identify which columns contain actual data (non-space characters)
5. Scan left-to-right to find contiguous blocks of used columns (these are individual problems)
6. For each problem block:
   - Extract numbers by reading each row top-to-bottom within that column range
   - Trim whitespace and parse as u128
   - Find the operator (+or \*) in the operator row for that block
   - Apply the operation to all numbers (sum for +, product for \*)
7. Add all problem results together for the grand total

**Example:**

```
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
```

Problems: `123*45*6=33210`, `328+64+98=490`, `51*387*215=4243455`, `64+23+314=401`
Grand total: `33210 + 490 + 4243455 + 401 = 4277556`

## Part 2: Right-to-Left Vertical Reading

**Algorithm:**

1. Read and parse input the same way as Part 1
2. Pad rows to uniform width
3. Identify used columns
4. Scan **right-to-left** to find contiguous blocks
5. For each problem block:
   - Read each column as a vertical number (top digit = most significant)
   - Concatenate digits vertically: column chars from top to bottom form one number
   - Extract all such vertical numbers in the block
   - Find the operator for that block
   - Apply the operation across all vertical numbers
6. Add all problem results for the grand total

**Example (same input, different interpretation):**

```
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
```

Reading right-to-left by columns:

- Rightmost: `4+431+623=1058` (columns read as: 4, 431, 623)
- Next: `175*581*32=3253600` (columns: 175, 581, 32)
- Next: `8+248+369=625` (columns: 8, 248, 369)
- Leftmost: `356*24*1=8544` (columns: 356, 24, 1)

Grand total: `1058 + 3253600 + 625 + 8544 = 3263827`

## Key Differences

- **Part 1**: Numbers are horizontal (row-wise), problems read left-to-right
- **Part 2**: Numbers are vertical (column-wise), problems read right-to-left
