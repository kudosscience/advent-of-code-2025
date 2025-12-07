use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let result_part1 = solve_part1(&input);
    println!("Part 1: The beam is split {} times", result_part1);
    
    let result_part2 = solve_part2(&input);
    println!("Part 2: {} different timelines", result_part2);
}

fn solve_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find the starting position 'S'
    let mut start_col = 0;
    for row in grid.iter() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_col = col_idx;
                break;
            }
        }
    }

    // Count unique splitters hit
    let mut split_count = 0;
    let mut visited_splitters: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    let mut beams: std::collections::VecDeque<(usize, usize)> = std::collections::VecDeque::new();
    beams.push_back((1, start_col));
    let mut processed_beams: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();

    while let Some((row, col)) = beams.pop_front() {
        if row >= rows || col >= cols {
            continue;
        }
        if processed_beams.contains(&(row, col)) {
            continue;
        }
        processed_beams.insert((row, col));

        let mut current_row = row;
        while current_row < rows {
            let ch = grid[current_row][col];
            if ch == '^' {
                if !visited_splitters.contains(&(current_row, col)) {
                    visited_splitters.insert((current_row, col));
                    split_count += 1;
                    let next_row = current_row + 1;
                    if col > 0 {
                        beams.push_back((next_row, col - 1));
                    }
                    if col + 1 < cols {
                        beams.push_back((next_row, col + 1));
                    }
                }
                break;
            }
            current_row += 1;
        }
    }
    split_count
}

fn solve_part2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Find the starting position 'S'
    let mut start_col = 0;
    for row in grid.iter() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                start_col = col_idx;
                break;
            }
        }
    }

    // For part 2, we need to count timelines.
    // Each timeline is a unique path through the manifold.
    // When a particle hits a splitter, the timeline splits into 2.
    // We track how many timelines are currently at each column position,
    // and propagate them row by row.
    
    // Map from column -> number of timelines at that column
    let mut timelines: HashMap<usize, u64> = HashMap::new();
    timelines.insert(start_col, 1); // Start with 1 timeline at the starting position
    
    // Process row by row, starting from row 1 (below S)
    for row in 1..rows {
        if timelines.is_empty() {
            break;
        }
        
        // Check what's at each column where we have active timelines
        let mut new_timelines: HashMap<usize, u64> = HashMap::new();
        
        for (&col, &count) in timelines.iter() {
            if col >= cols {
                continue;
            }
            
            let ch = grid[row][col];
            
            if ch == '^' {
                // Splitter: each timeline splits into two
                // Left path
                if col > 0 {
                    *new_timelines.entry(col - 1).or_insert(0) += count;
                }
                // Right path  
                if col + 1 < cols {
                    *new_timelines.entry(col + 1).or_insert(0) += count;
                }
            } else {
                // Empty space or other: timelines continue straight down
                *new_timelines.entry(col).or_insert(0) += count;
            }
        }
        
        timelines = new_timelines;
    }
    
    // Sum up all remaining timelines
    timelines.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE), 40);
    }
}
