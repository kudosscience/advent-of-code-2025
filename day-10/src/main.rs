use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let total = solve(&input);
    println!("Total minimum button presses: {}", total);
}

fn solve(input: &str) -> usize {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| solve_machine(line))
        .sum()
}

fn solve_machine(line: &str) -> usize {
    let (target, buttons) = parse_machine(line);
    find_min_presses(&target, &buttons)
}

fn parse_machine(line: &str) -> (Vec<bool>, Vec<Vec<usize>>) {
    // Parse indicator light diagram [.##.]
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let diagram = &line[bracket_start + 1..bracket_end];
    let target: Vec<bool> = diagram.chars().map(|c| c == '#').collect();
    
    // Parse button wiring schematics (x,y,z) - stop at curly brace
    let rest = &line[bracket_end + 1..];
    let curly_start = rest.find('{').unwrap_or(rest.len());
    let buttons_str = &rest[..curly_start];
    
    let mut buttons = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = buttons_str.chars().collect();
    
    while i < chars.len() {
        if chars[i] == '(' {
            let mut j = i + 1;
            while j < chars.len() && chars[j] != ')' {
                j += 1;
            }
            let button_str: String = chars[i + 1..j].iter().collect();
            let indices: Vec<usize> = button_str
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse().unwrap())
                .collect();
            buttons.push(indices);
            i = j + 1;
        } else {
            i += 1;
        }
    }
    
    (target, buttons)
}

fn find_min_presses(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let num_lights = target.len();
    let num_buttons = buttons.len();
    
    // Convert target to binary vector
    let target_bits: Vec<u8> = target.iter().map(|&b| if b { 1 } else { 0 }).collect();
    
    // Build matrix where each column represents a button's effect
    // Matrix is num_lights rows x num_buttons columns
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; num_buttons]; num_lights];
    for (btn_idx, button) in buttons.iter().enumerate() {
        for &light_idx in button {
            if light_idx < num_lights {
                matrix[light_idx][btn_idx] = 1;
            }
        }
    }
    
    // We need to find x (button presses) such that matrix * x = target (mod 2)
    // and minimize sum(x)
    // Since each button press is binary (0 or 1 effective presses mod 2),
    // we enumerate all 2^num_buttons possibilities
    
    if num_buttons <= 20 {
        // Brute force for small number of buttons
        find_min_by_enumeration(&matrix, &target_bits, num_buttons, num_lights)
    } else {
        // For larger number of buttons, use Gaussian elimination to find 
        // the solution space, then search through it
        find_min_with_gauss(&matrix, &target_bits, num_buttons, num_lights)
    }
}

fn find_min_by_enumeration(
    matrix: &[Vec<u8>],
    target: &[u8],
    num_buttons: usize,
    num_lights: usize,
) -> usize {
    let mut min_presses = usize::MAX;
    
    for mask in 0u32..(1u32 << num_buttons) {
        let presses = mask.count_ones() as usize;
        if presses >= min_presses {
            continue;
        }
        
        // Check if this combination of button presses achieves the target
        let mut result = vec![0u8; num_lights];
        for btn in 0..num_buttons {
            if (mask >> btn) & 1 == 1 {
                for light in 0..num_lights {
                    result[light] ^= matrix[light][btn];
                }
            }
        }
        
        if result == target {
            min_presses = presses;
        }
    }
    
    if min_presses == usize::MAX {
        panic!("No solution found!");
    }
    min_presses
}

fn find_min_with_gauss(
    matrix: &[Vec<u8>],
    target: &[u8],
    num_buttons: usize,
    num_lights: usize,
) -> usize {
    // Create augmented matrix [A|b]
    let mut aug: Vec<Vec<u8>> = vec![vec![0; num_buttons + 1]; num_lights];
    for row in 0..num_lights {
        for col in 0..num_buttons {
            aug[row][col] = matrix[row][col];
        }
        aug[row][num_buttons] = target[row];
    }
    
    // Gaussian elimination in GF(2)
    let mut pivot_cols = Vec::new();
    let mut pivot_row = 0;
    
    for col in 0..num_buttons {
        // Find pivot
        let mut found = false;
        for row in pivot_row..num_lights {
            if aug[row][col] == 1 {
                aug.swap(pivot_row, row);
                found = true;
                break;
            }
        }
        
        if !found {
            continue;
        }
        
        pivot_cols.push(col);
        
        // Eliminate
        for row in 0..num_lights {
            if row != pivot_row && aug[row][col] == 1 {
                for c in 0..=num_buttons {
                    aug[row][c] ^= aug[pivot_row][c];
                }
            }
        }
        
        pivot_row += 1;
    }
    
    // Check for inconsistency
    for row in pivot_row..num_lights {
        if aug[row][num_buttons] == 1 {
            panic!("No solution exists!");
        }
    }
    
    // Find free variables
    let pivot_set: std::collections::HashSet<usize> = pivot_cols.iter().copied().collect();
    let free_vars: Vec<usize> = (0..num_buttons).filter(|c| !pivot_set.contains(c)).collect();
    
    let num_free = free_vars.len();
    
    // Enumerate all 2^num_free combinations of free variables
    let mut min_presses = usize::MAX;
    
    for free_mask in 0u64..(1u64 << num_free) {
        let mut solution = vec![0u8; num_buttons];
        
        // Set free variables
        for (i, &var) in free_vars.iter().enumerate() {
            solution[var] = ((free_mask >> i) & 1) as u8;
        }
        
        // Back-substitute to find pivot variables
        for (idx, &pivot_col) in pivot_cols.iter().enumerate().rev() {
            let row = idx;
            let mut val = aug[row][num_buttons];
            for col in (pivot_col + 1)..num_buttons {
                val ^= aug[row][col] * solution[col];
            }
            solution[pivot_col] = val;
        }
        
        // Count presses
        let presses: usize = solution.iter().map(|&x| x as usize).sum();
        min_presses = min_presses.min(presses);
    }
    
    if min_presses == usize::MAX {
        panic!("No solution found!");
    }
    min_presses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        assert_eq!(solve_machine(line), 2);
    }

    #[test]
    fn test_example_2() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        assert_eq!(solve_machine(line), 3);
    }

    #[test]
    fn test_example_3() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve_machine(line), 2);
    }

    #[test]
    fn test_all_examples() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve(&input), 7);
    }
}
