use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() != 2 {
        panic!("Expected input to have two sections separated by blank line");
    }
    
    // Parse fresh ingredient ID ranges
    let mut ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|line| {
            let mut nums = line.split('-');
            let start: u64 = nums.next().unwrap().parse().unwrap();
            let end: u64 = nums.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();
    
    // Parse available ingredient IDs
    let ingredient_ids: Vec<u64> = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    
    // Part 1: Count how many ingredient IDs are fresh (fall within any range)
    let fresh_count = ingredient_ids
        .iter()
        .filter(|&&id| is_fresh(id, &ranges))
        .count();
    
    println!("Part 1 - Number of fresh ingredient IDs: {}", fresh_count);
    
    // Part 2: Count total unique IDs considered fresh by the ranges
    let merged_ranges = merge_ranges(&mut ranges);
    let total_fresh_ids: u64 = merged_ranges
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum();
    
    println!("Part 2 - Total IDs considered fresh: {}", total_fresh_ids);
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

fn merge_ranges(ranges: &mut [(u64, u64)]) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return vec![];
    }
    
    // Sort ranges by start value
    ranges.sort_by_key(|&(start, _)| start);
    
    let mut merged: Vec<(u64, u64)> = vec![ranges[0]];
    
    for &(start, end) in ranges.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        // Check if ranges overlap or are adjacent
        if start <= last.1 + 1 {
            // Merge by extending the end if necessary
            last.1 = last.1.max(end);
        } else {
            // No overlap, add as new range
            merged.push((start, end));
        }
    }
    
    merged
}
