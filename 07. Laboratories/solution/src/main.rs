use std::collections::{HashMap, HashSet};
use std::fs;

pub fn part_two(input_file: &str) -> u64 {
    // Read the input file content.
    let input = fs::read_to_string(input_file).expect("Unable to read file");

    // Parse the grid into a vector of character vectors.
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // 1. Find the starting position 'S'.
    let (mut start_row, mut start_col) = (0, 0);

    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|&cell| cell == 'S') {
            start_row = r;
            start_col = c;
            break;
        }
    }

    // 2. Initialize DP table (or map). We only need to store the previous and current row.
    // timeline_counts_prev[c] = number of timelines reaching column c in the previous row.
    // Using HashMap for sparse storage since many columns will have 0 timelines.
    let mut timeline_counts_prev: HashMap<usize, u64> = HashMap::new();

    // The particle starts at S, and one timeline continues down to the next row at the same column.
    if start_row + 1 < rows {
        timeline_counts_prev.insert(start_col, 1);
    } else {
        // 'S' is on the last row, 1 timeline ends immediately.
        return 1;
    }

    // 3. Iterate over the grid rows, starting from the row immediately below 'S'.
    for row_idx in (start_row + 1)..rows {
        let mut timeline_counts_curr: HashMap<usize, u64> = HashMap::new();

        // Iterate through all columns in the current row.
        for col in 0..cols {
            let mut count_at_col: u64 = 0;

            // Contribution 1: From a non-splitter cell ('.') directly above.
            // If the cell above (row_idx - 1, col) is '.', the path continues straight down.
            if let Some(prev_count) = timeline_counts_prev.get(&col) {
                if grid[row_idx - 1][col] != '^' {
                    count_at_col += prev_count;
                }
            }

            // Contribution 2: From a splitter ('^') in the previous row to the right (col + 1).
            // A splitter at col + 1 splits left to col.
            if col + 1 < cols {
                if grid[row_idx - 1][col + 1] == '^' {
                    if let Some(prev_count) = timeline_counts_prev.get(&(col + 1)) {
                        count_at_col += prev_count;
                    }
                }
            }

            // Contribution 3: From a splitter ('^') in the previous row to the left (col - 1).
            // A splitter at col - 1 splits right to col.
            if col > 0 {
                if grid[row_idx - 1][col - 1] == '^' {
                    if let Some(prev_count) = timeline_counts_prev.get(&(col - 1)) {
                        count_at_col += prev_count;
                    }
                }
            }

            // Only store non-zero counts.
            if count_at_col > 0 {
                timeline_counts_curr.insert(col, count_at_col);
            }
        }

        // Update active columns for the next iteration (next row).
        timeline_counts_prev = timeline_counts_curr;

        // If no more active timelines, we can stop the simulation.
        if timeline_counts_prev.is_empty() {
            break;
        }
    }

    // 4. The total number of timelines is the sum of timelines reaching the last processed row.
    timeline_counts_prev.values().sum()
}

// Function to write the result to a file.
pub fn main() {
    let part_one_result = part_one("../input.txt");
    fs::write("../output_part_one.txt", part_one_result.to_string()).expect("Unable to write file");

    let part_two_result = part_two("../input.txt");
    fs::write("../output_part_two.txt", part_two_result.to_string()).expect("Unable to write file");
}

// Re-include part_one for a complete, runnable solution block.
// This is necessary to avoid duplicating the file reading and grid parsing in `solve()`.
pub fn part_one(input_file: &str) -> i64 {
    let input = fs::read_to_string(input_file).expect("Unable to read file");
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let max_col_idx = cols.saturating_sub(1);

    let (mut start_row, mut start_col) = (0, 0);
    let mut found_start = false;

    for (r, row) in grid.iter().enumerate() {
        if let Some(c) = row.iter().position(|&cell| cell == 'S') {
            start_row = r;
            start_col = c;
            found_start = true;
            break;
        }
    }

    if !found_start {
        return 0;
    }

    let mut active_columns: HashSet<usize> = HashSet::new();
    active_columns.insert(start_col);

    let mut total_splits: i64 = 0;

    for row_idx in (start_row + 1)..rows {
        let mut new_active_columns: HashSet<usize> = HashSet::new();

        for col in active_columns.drain() {
            let cell = grid[row_idx][col];

            match cell {
                '^' => {
                    total_splits += 1;

                    if col > 0 {
                        new_active_columns.insert(col - 1);
                    }

                    if col < max_col_idx {
                        new_active_columns.insert(col + 1);
                    }
                }
                '.' | 'S' => {
                    new_active_columns.insert(col);
                }
                _ => {
                    new_active_columns.insert(col);
                }
            }
        }

        active_columns = new_active_columns;

        if active_columns.is_empty() {
            break;
        }
    }

    total_splits
}
