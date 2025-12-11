use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

fn parse_input(content: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue; // Skip malformed lines
        }

        let device = parts[0].trim().to_string();
        let outputs_str = parts[1].trim();

        // Outputs can be empty (e.g., 'rmn: ').
        let outputs: Vec<String> = if outputs_str.is_empty() {
            Vec::new()
        } else {
            outputs_str
                .split(' ')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        graph.insert(device, outputs);
    }

    graph
}

fn count_paths_to_target(
    node: &str,
    target: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, String), u64>,
) -> u64 {
    let memo_key = (node.to_string(), target.to_string());

    // 1. Check Memoization
    if let Some(&count) = memo.get(&memo_key) {
        return count;
    }

    // 2. Base Case: If the current node is the target, we've found 1 complete path.
    if node == target {
        memo.insert(memo_key, 1);
        return 1;
    }

    // A node that is not the target and has no outputs has 0 paths to the target.
    let outputs = graph.get(node).map(|v| v.as_slice()).unwrap_or(&[]);

    // 3. Recursive Step: Sum the paths from all neighbors to the target.
    let mut total_paths = 0;
    for neighbor in outputs {
        total_paths += count_paths_to_target(neighbor, target, graph, memo);
    }

    // 4. Memoize and Return
    memo.insert(memo_key, total_paths);
    total_paths
}

pub fn part_one() -> io::Result<()> {
    let file_path = "../input.txt";
    let content = fs::read_to_string(file_path)?;

    let graph = parse_input(&content);

    // Note: For Part One, we can use the generalized function with 'out' as the target.
    let mut memo: HashMap<(String, String), u64> = HashMap::new();
    let paths_count = count_paths_to_target("you", "out", &graph, &mut memo);

    let output_path = "../output_part_one.txt";
    let mut file = fs::File::create(output_path)?;
    writeln!(file, "{}", paths_count)?;

    Ok(())
}

pub fn part_two() -> io::Result<()> {
    let file_path = "../input.txt";
    let content = fs::read_to_string(file_path)?;

    let graph = parse_input(&content);

    // Memoization table for P(A -> B) results.
    let mut memo: HashMap<(String, String), u64> = HashMap::new();

    let start = "svr";
    let target1 = "dac";
    let target2 = "fft";
    let end = "out";

    // --- Scenario 1: SVR -> DAC -> FFT -> OUT ---
    let p1_to_t1 = count_paths_to_target(start, target1, &graph, &mut memo);
    let t1_to_t2 = count_paths_to_target(target1, target2, &graph, &mut memo);
    let t2_to_end = count_paths_to_target(target2, end, &graph, &mut memo);

    // Calculate paths for this scenario. Check for overflow is prudent, though u64 is large.
    let scenario1_paths = p1_to_t1
        .checked_mul(t1_to_t2)
        .unwrap_or(u64::MAX)
        .checked_mul(t2_to_end)
        .unwrap_or(u64::MAX);

    // --- Scenario 2: SVR -> FFT -> DAC -> OUT ---
    let p2_to_t2 = count_paths_to_target(start, target2, &graph, &mut memo);
    let t2_to_t1 = count_paths_to_target(target2, target1, &graph, &mut memo);
    let t1_to_end = count_paths_to_target(target1, end, &graph, &mut memo);

    // Calculate paths for this scenario.
    let scenario2_paths = p2_to_t2
        .checked_mul(t2_to_t1)
        .unwrap_or(u64::MAX)
        .checked_mul(t1_to_end)
        .unwrap_or(u64::MAX);

    // --- Total Paths ---
    let total_paths = scenario1_paths
        .checked_add(scenario2_paths)
        .unwrap_or(u64::MAX);

    // 4. Write the result to the output file.
    let output_path = "../output_part_two.txt";
    let mut file = fs::File::create(output_path)?;
    writeln!(file, "{}", total_paths)?;

    Ok(())
}

fn main() -> io::Result<()> {
    part_one()?;
    part_two()?;
    Ok(())
}
