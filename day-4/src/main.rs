use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    // Eight directions: up, down, left, right, and four diagonals
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut accessible_count = 0;

    for row in 0..rows {
        for col in 0..cols {
            // Only consider paper rolls
            if grid[row][col] != '@' {
                continue;
            }

            // Count adjacent paper rolls
            let mut adjacent_rolls = 0;
            for (dr, dc) in &directions {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                // Check bounds
                if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                    if grid[new_row as usize][new_col as usize] == '@' {
                        adjacent_rolls += 1;
                    }
                }
            }

            // Forklift can access if fewer than 4 adjacent rolls
            if adjacent_rolls < 4 {
                accessible_count += 1;
            }
        }
    }

    println!("Number of accessible paper rolls: {}", accessible_count);
}
