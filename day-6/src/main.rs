use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let result = solve(&input);
    println!("Grand total: {}", result);
}

fn solve(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    
    if lines.is_empty() {
        return 0;
    }
    
    // Find the maximum line length to handle all columns
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    
    // Pad all lines to the same length
    let padded_lines: Vec<String> = lines
        .iter()
        .map(|l| format!("{:width$}", l, width = max_len))
        .collect();
    
    // Convert to char vectors for easier column access
    let char_grid: Vec<Vec<char>> = padded_lines.iter().map(|l| l.chars().collect()).collect();
    
    // The last line contains the operators
    let num_rows = char_grid.len();
    let operator_row = num_rows - 1;
    
    // Find problem boundaries - columns that are all spaces (including operator row)
    let mut is_separator: Vec<bool> = vec![true; max_len];
    for col in 0..max_len {
        for row in 0..num_rows {
            let c = char_grid[row][col];
            if c != ' ' {
                is_separator[col] = false;
                break;
            }
        }
    }
    
    // Identify problem ranges (start_col, end_col exclusive)
    let mut problems: Vec<(usize, usize)> = Vec::new();
    let mut in_problem = false;
    let mut start = 0;
    
    for col in 0..max_len {
        if is_separator[col] {
            if in_problem {
                problems.push((start, col));
                in_problem = false;
            }
        } else {
            if !in_problem {
                start = col;
                in_problem = true;
            }
        }
    }
    if in_problem {
        problems.push((start, max_len));
    }
    
    // For each problem, parse the numbers
    // Each COLUMN is ONE NUMBER, reading top-to-bottom as most-significant to least-significant digit
    let mut grand_total: u64 = 0;
    
    for (start_col, end_col) in problems {
        // Find the operator for this problem
        let mut op = '+';
        for col in start_col..end_col {
            let c = char_grid[operator_row][col];
            if c == '*' || c == '+' {
                op = c;
                break;
            }
        }
        
        // Parse numbers: each column is ONE number
        // Read top-to-bottom as most-significant to least-significant digit
        let mut numbers: Vec<u64> = Vec::new();
        
        for col in start_col..end_col {
            // Check if this column has any digits (in the non-operator rows)
            let mut has_digit = false;
            for row in 0..(num_rows - 1) {
                if char_grid[row][col].is_ascii_digit() {
                    has_digit = true;
                    break;
                }
            }
            
            if has_digit {
                // Build the number from top to bottom
                let mut num: u64 = 0;
                for row in 0..(num_rows - 1) {
                    let c = char_grid[row][col];
                    if c.is_ascii_digit() {
                        num = num * 10 + c.to_digit(10).unwrap() as u64;
                    }
                }
                numbers.push(num);
            }
        }
        
        // Calculate result
        let result = match op {
            '*' => {
                if numbers.is_empty() {
                    0
                } else {
                    numbers.iter().product::<u64>()
                }
            },
            '+' => numbers.iter().sum::<u64>(),
            _ => 0,
        };
        
        grand_total += result;
    }
    
    grand_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part2() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        
        // Each column is one number, reading top-to-bottom as MSB to LSB
        // The expected answer is 3263827
        assert_eq!(solve(input), 3263827);
    }
}
