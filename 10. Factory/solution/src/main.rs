use std::collections::HashSet;
use std::fs;

/// Reads the input file and writes the solution to the output file.
fn main() {
    let result = part_one();
    let output_path = "../output_part_one.txt";
    fs::write(output_path, result.to_string()).expect("Unable to write to ../output_part_one.txt");
}

/// Parses a single machine line into a target vector and a button matrix.
fn parse_line(line: &str) -> (Vec<u8>, Vec<Vec<u8>>, usize, usize) {
    // Split the line to isolate the target and button areas
    let mut parts = line.split(" {");
    let problem_part = parts.next().unwrap();

    // 1. Parse Target Vector t
    let start_target = problem_part.find('[').unwrap() + 1;
    let end_target = problem_part.find(']').unwrap();
    let target_str = &problem_part[start_target..end_target];
    let target: Vec<u8> = target_str
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    let n = target.len();

    // 2. Parse Button Matrix A (collecting indices first)
    let button_area = &problem_part[end_target + 1..];
    let mut buttons_indices: Vec<Vec<usize>> = Vec::new();

    let mut current_pos = 0;
    while let Some(start) = button_area[current_pos..].find('(') {
        let start = current_pos + start;
        if let Some(end) = button_area[start..].find(')') {
            let end = start + end;
            let indices_str = &button_area[start + 1..end];
            let indices: Vec<usize> = indices_str
                .split(',')
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .collect();
            buttons_indices.push(indices);
            current_pos = end + 1;
        } else {
            break; // Malformed line
        }
    }

    let m = buttons_indices.len();
    let mut a_matrix = vec![vec![0u8; m]; n];

    // Convert indices to the full A matrix
    for (j, indices) in buttons_indices.into_iter().enumerate() {
        for i in indices {
            if i < n {
                a_matrix[i][j] = 1;
            }
        }
    }

    (target, a_matrix, n, m)
}

/// Solves the system A*p = t (mod 2) using Gaussian elimination.
/// Returns a particular solution p0 and a basis for the null space, or an error if unsolvable.
fn solve_gf2(
    a: Vec<Vec<u8>>,
    t: Vec<u8>,
    n: usize,
    m: usize,
) -> Result<(Vec<u8>, Vec<Vec<u8>>), ()> {
    // Create augmented matrix [A | t]
    let mut b: Vec<Vec<u8>> = a
        .into_iter()
        .enumerate()
        .map(|(i, mut row)| {
            row.push(t[i]);
            row
        })
        .collect();

    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();

    for j in 0..m {
        // Column (variable / button)
        if pivot_row == n {
            break;
        }

        // Find pivot
        let mut i = pivot_row;
        while i < n && b[i][j] == 0 {
            i += 1;
        }

        if i < n {
            // Found pivot at B[i][j]
            // Swap rows
            b.swap(pivot_row, i);

            // Eliminate 1s in column j in all other rows
            for k in 0..n {
                if k != pivot_row && b[k][j] == 1 {
                    // B[k] = B[k] + B[pivot_row] (mod 2) -> XOR
                    for l in 0..m + 1 {
                        b[k][l] ^= b[pivot_row][l];
                    }
                }
            }

            pivot_cols.push(j);
            pivot_row += 1;
        }
    }

    let rank = pivot_row;

    // Check for inconsistency
    for i in rank..n {
        if b[i][m] == 1 {
            return Err(()); // No solution
        }
    }

    // 1. Particular solution p0 (free variables = 0)
    let mut p0 = vec![0u8; m];
    for i in 0..rank {
        let pivot_var = pivot_cols[i];
        // B[i][m] is the result (t) column
        p0[pivot_var] = b[i][m];
    }

    // 2. Null space basis vectors
    let pivot_indices: HashSet<usize> = pivot_cols.iter().cloned().collect();
    let free_indices: Vec<usize> = (0..m).filter(|j| !pivot_indices.contains(j)).collect();
    let mut null_basis = Vec::new();

    for f in free_indices {
        let mut v = vec![0u8; m];
        v[f] = 1; // Set the current free variable to 1

        // Calculate pivot variables: v[pivot_var] = B[i][f]
        for i in 0..rank {
            let pivot_var = pivot_cols[i];
            v[pivot_var] = b[i][f];
        }
        null_basis.push(v);
    }

    Ok((p0, null_basis))
}

/// Finds the minimum Hamming weight (fewest presses) solution
/// by iterating over all linear combinations of the null space basis.
fn find_min_presses(p0: Vec<u8>, null_basis: Vec<Vec<u8>>) -> u64 {
    let m = p0.len();
    let k = null_basis.len();

    if k == 0 {
        return p0.iter().map(|&x| x as u64).sum();
    }

    let mut min_weight: u64 = u64::MAX;

    // Iterate through all 2^k linear combinations
    for i in 0..(1u64 << k) {
        let mut current_p = p0.clone();

        let mut temp_i = i;
        let mut current_basis_index = 0;

        while temp_i > 0 {
            if temp_i & 1 == 1 {
                // If the bit is 1, include null_basis[j]
                let v = &null_basis[current_basis_index];

                // current_p = current_p + v (mod 2) -> XOR
                for l in 0..m {
                    current_p[l] ^= v[l];
                }
            }
            temp_i >>= 1;
            current_basis_index += 1;
        }

        // Calculate Hamming weight (number of 1s)
        let current_weight: u64 = current_p.iter().map(|&x| x as u64).sum();
        min_weight = min_weight.min(current_weight);
    }

    min_weight
}

/// The core function to solve Part One.
fn part_one() -> u64 {
    let input_path = "../input.txt";
    let input_data =
        fs::read_to_string(input_path).expect("ERROR: Could not read file ../input.txt");

    let mut total_min_presses: u64 = 0;

    for line in input_data.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let (target, a_matrix, n, m) = parse_line(line);

        // Handle case with no buttons
        if m == 0 {
            if target.iter().all(|&x| x == 0) {
                // Already off, 0 presses
                total_min_presses += 0;
            } else {
                // Required state is non-zero, but no buttons to change it. Unsolvable.
                panic!("Unsolvable machine found (no buttons, but non-zero target state).");
            }
            continue;
        }

        match solve_gf2(a_matrix, target, n, m) {
            Ok((p0, null_basis)) => {
                let min_presses = find_min_presses(p0, null_basis);
                total_min_presses += min_presses;
            }
            Err(_) => {
                // If a system is mathematically unsolvable (inconsistent)
                panic!("Unsolvable machine found (inconsistent linear system).");
            }
        }
    }

    total_min_presses
}
