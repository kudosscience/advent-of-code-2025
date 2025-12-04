use std::fs;

// Eight directions: up, down, left, right, and four diagonals
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1),
];

fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for (dr, dc) in &DIRECTIONS {
        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            if grid[new_row as usize][new_col as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

fn find_accessible_rolls(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut accessible = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '@' && count_adjacent_rolls(grid, row, col) < 4 {
                accessible.push((row, col));
            }
        }
    }
    accessible
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Part 1: Count initially accessible rolls
    let part1_count = find_accessible_rolls(&grid).len();
    println!("Part 1 - Initially accessible paper rolls: {}", part1_count);

    // Part 2: Keep removing accessible rolls until none remain accessible
    let mut total_removed = 0;

    loop {
        let accessible = find_accessible_rolls(&grid);
        if accessible.is_empty() {
            break;
        }

        // Remove all currently accessible rolls
        for (row, col) in &accessible {
            grid[*row][*col] = '.';
        }
        total_removed += accessible.len();
    }

    println!("Part 2 - Total rolls removed: {}", total_removed);
}
