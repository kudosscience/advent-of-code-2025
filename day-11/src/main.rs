use std::collections::HashMap;
use std::fs;

/// Counts all paths from the current node to "out" using DFS with memoization.
/// Returns the number of distinct paths from `current` to "out".
fn count_paths<'a>(
    current: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    // Base case: reached the destination
    if current == "out" {
        return 1;
    }

    // Check if we've already computed the paths from this node
    if let Some(&count) = memo.get(current) {
        return count;
    }

    // Get the outputs for the current device
    let outputs = match graph.get(current) {
        Some(outputs) => outputs,
        None => return 0, // Dead end - no outputs defined for this device
    };

    // Sum up paths through all outputs
    let total_paths: u64 = outputs
        .iter()
        .map(|&next| count_paths(next, graph, memo))
        .sum();

    // Cache the result
    memo.insert(current, total_paths);

    total_paths
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    // Parse the input into a graph (adjacency list)
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        // Parse "device: output1 output2 output3"
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let device = parts[0].trim();
        let outputs: Vec<&str> = parts[1].trim().split_whitespace().collect();

        graph.insert(device, outputs);
    }

    // Count all paths from "you" to "out"
    let mut memo: HashMap<&str, u64> = HashMap::new();
    let path_count = count_paths("you", &graph, &mut memo);

    println!("Number of different paths from 'you' to 'out': {}", path_count);
}
