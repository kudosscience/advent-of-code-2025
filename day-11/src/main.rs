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

/// Counts paths from current node to "out" that visit both required nodes.
/// State tracks which of the required nodes (dac, fft) have been visited:
/// 0 = neither, 1 = dac only, 2 = fft only, 3 = both
fn count_paths_with_requirements<'a>(
    current: &'a str,
    state: u8,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, u8), u64>,
) -> u64 {
    // Update state based on current node
    let new_state = if current == "dac" {
        state | 1
    } else if current == "fft" {
        state | 2
    } else {
        state
    };

    // Base case: reached the destination
    if current == "out" {
        // Only count this path if we've visited both dac and fft (state == 3)
        return if new_state == 3 { 1 } else { 0 };
    }

    // Check if we've already computed the paths from this node with this state
    if let Some(&count) = memo.get(&(current, new_state)) {
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
        .map(|&next| count_paths_with_requirements(next, new_state, graph, memo))
        .sum();

    // Cache the result
    memo.insert((current, new_state), total_paths);

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

    // Part 1: Count all paths from "you" to "out"
    let mut memo: HashMap<&str, u64> = HashMap::new();
    let path_count = count_paths("you", &graph, &mut memo);
    println!("Part 1: Number of different paths from 'you' to 'out': {}", path_count);

    // Part 2: Count paths from "svr" to "out" that visit both "dac" and "fft"
    let mut memo2: HashMap<(&str, u8), u64> = HashMap::new();
    let path_count2 = count_paths_with_requirements("svr", 0, &graph, &mut memo2);
    println!("Part 2: Number of paths from 'svr' to 'out' visiting both 'dac' and 'fft': {}", path_count2);
}
