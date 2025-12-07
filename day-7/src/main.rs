use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let result = solve(&input);
    println!("The beam is split {} times", result);
}

fn solve(input: &str) -> usize {
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

    // BFS/simulation of tachyon beams
    // Each beam is represented by its column position and current row
    // Beams move downward until they hit a splitter or exit the grid
    
    let mut split_count = 0;
    let mut visited_splitters: HashSet<(usize, usize)> = HashSet::new();
    
    // Queue of active beams: (row, col)
    // Start just below the 'S' position
    let mut beams: VecDeque<(usize, usize)> = VecDeque::new();
    beams.push_back((1, start_col)); // Start from row 1 (below S which is at row 0)

    // Track which beams we've already processed to avoid duplicates
    let mut processed_beams: HashSet<(usize, usize)> = HashSet::new();

    while let Some((row, col)) = beams.pop_front() {
        // Skip if out of bounds or already processed
        if row >= rows || col >= cols {
            continue;
        }
        
        if processed_beams.contains(&(row, col)) {
            continue;
        }
        processed_beams.insert((row, col));

        // Move the beam downward until it hits a splitter or exits
        let mut current_row = row;
        
        while current_row < rows {
            let ch = grid[current_row][col];
            
            if ch == '^' {
                // Hit a splitter
                if !visited_splitters.contains(&(current_row, col)) {
                    visited_splitters.insert((current_row, col));
                    split_count += 1;
                    
                    // Create two new beams from the splitter going left and right
                    // The new beams start from the row below the splitter
                    let next_row = current_row + 1;
                    
                    if col > 0 {
                        beams.push_back((next_row, col - 1)); // Left beam
                    }
                    if col + 1 < cols {
                        beams.push_back((next_row, col + 1)); // Right beam
                    }
                }
                break; // Current beam stops at splitter
            }
            
            current_row += 1;
        }
    }

    split_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = ".......S.......
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
        assert_eq!(solve(input), 21);
    }
}
