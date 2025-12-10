use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    
    let total_part1 = solve_part1(&input);
    println!("Part 1 - Total minimum button presses: {}", total_part1);
    
    let total_part2 = solve_part2(&input);
    println!("Part 2 - Total minimum button presses: {}", total_part2);
}

fn solve_part1(input: &str) -> usize {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (target, buttons, _) = parse_machine(line);
            find_min_presses_part1(&target, &buttons)
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (_, buttons, joltage) = parse_machine(line);
            find_min_presses_part2(&joltage, &buttons)
        })
        .sum()
}

fn parse_machine(line: &str) -> (Vec<bool>, Vec<Vec<usize>>, Vec<usize>) {
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
    
    // Parse joltage requirements {x,y,z}
    let joltage = if let Some(curly_pos) = rest.find('{') {
        let curly_end = rest.find('}').unwrap();
        let joltage_str = &rest[curly_pos + 1..curly_end];
        joltage_str
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse().unwrap())
            .collect()
    } else {
        Vec::new()
    };
    
    (target, buttons, joltage)
}

fn find_min_presses_part1(target: &[bool], buttons: &[Vec<usize>]) -> usize {
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

// Part 2: Find minimum button presses to reach joltage target (non-negative integers)
// This is an Integer Linear Programming problem: minimize sum(x) subject to Ax = b, x >= 0
fn find_min_presses_part2(target: &[usize], buttons: &[Vec<usize>]) -> usize {
    let num_counters = target.len();
    let num_buttons = buttons.len();
    
    // Build matrix A where A[i][j] = 1 if button j affects counter i
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; num_buttons]; num_counters];
    for (btn_idx, button) in buttons.iter().enumerate() {
        for &counter_idx in button {
            if counter_idx < num_counters {
                matrix[counter_idx][btn_idx] = 1;
            }
        }
    }
    
    let target_i64: Vec<i64> = target.iter().map(|&x| x as i64).collect();
    
    // Use branch and bound with simplex-like relaxation
    solve_ilp(&matrix, &target_i64, num_buttons, num_counters)
}

fn solve_ilp(matrix: &[Vec<i64>], target: &[i64], num_buttons: usize, num_counters: usize) -> usize {
    // We'll use a different approach: enumerate solutions smartly
    // Key insight: The problem is equivalent to finding non-negative integer
    // solutions to a system of linear equations that minimizes sum.
    
    // Use Gaussian elimination to find the solution space, then search
    // for minimum weight solution in the null space.
    
    // Build augmented matrix [A | b]
    let mut aug: Vec<Vec<i64>> = vec![vec![0; num_buttons + 1]; num_counters];
    for i in 0..num_counters {
        for j in 0..num_buttons {
            aug[i][j] = matrix[i][j];
        }
        aug[i][num_buttons] = target[i];
    }
    
    // Gaussian elimination (over rationals, but we'll work with integers and track pivots)
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut pivot_rows: Vec<usize> = Vec::new();
    let mut current_row = 0;
    
    for col in 0..num_buttons {
        // Find a pivot in this column
        let mut pivot = None;
        for row in current_row..num_counters {
            if aug[row][col] != 0 {
                pivot = Some(row);
                break;
            }
        }
        
        if let Some(pivot_row) = pivot {
            // Swap rows
            aug.swap(current_row, pivot_row);
            pivot_cols.push(col);
            pivot_rows.push(current_row);
            
            // Eliminate other rows
            let pivot_val = aug[current_row][col];
            for row in 0..num_counters {
                if row != current_row && aug[row][col] != 0 {
                    let factor = aug[row][col];
                    for c in 0..=num_buttons {
                        aug[row][c] = aug[row][c] * pivot_val - aug[current_row][c] * factor;
                    }
                }
            }
            
            current_row += 1;
        }
    }
    
    let rank = pivot_cols.len();
    
    // Check for inconsistency
    for row in rank..num_counters {
        if aug[row][num_buttons] != 0 {
            panic!("No solution exists!");
        }
    }
    
    // Free variables are columns not in pivot_cols
    let pivot_set: std::collections::HashSet<usize> = pivot_cols.iter().copied().collect();
    let free_vars: Vec<usize> = (0..num_buttons).filter(|c| !pivot_set.contains(c)).collect();
    
    // For each assignment of free variables, solve for pivot variables
    // and check if solution is valid (non-negative integers)
    
    // Due to the elimination, we have for each pivot row:
    // pivot_val * x[pivot_col] + sum(aug[row][j] * x[j] for j > pivot_col) = aug[row][num_buttons]
    
    // We need to search over free variable assignments
    // This can be expensive, so we'll use bounds
    
    // Compute upper bounds for each free variable
    let mut upper_bounds: Vec<i64> = vec![0; num_buttons];
    for &free_var in &free_vars {
        // A free variable can be at most max(target) since each button press adds at least 1
        let max_contribution: i64 = (0..num_counters)
            .filter(|&i| matrix[i][free_var] > 0)
            .map(|i| target[i])
            .max()
            .unwrap_or(0);
        upper_bounds[free_var] = max_contribution;
    }
    
    // Also compute bounds from the equations
    for (&pivot_col, &pivot_row) in pivot_cols.iter().zip(pivot_rows.iter()) {
        let rhs = aug[pivot_row][num_buttons];
        let pivot_val = aug[pivot_row][pivot_col];
        // x[pivot_col] needs to be >= 0, so:
        // rhs - sum(aug[row][j]*x[j]) >= 0 (adjusted for sign of pivot_val)
        // This gives us constraints on the free variables
        let max_val = if pivot_val > 0 { rhs / pivot_val } else { (-rhs) / (-pivot_val) };
        upper_bounds[pivot_col] = upper_bounds[pivot_col].max(max_val);
    }
    
    // Simple enumeration with pruning
    find_min_solution(&aug, &pivot_cols, &pivot_rows, &free_vars, &upper_bounds, num_buttons)
}

fn find_min_solution(
    aug: &[Vec<i64>],
    pivot_cols: &[usize],
    pivot_rows: &[usize],
    free_vars: &[usize],
    _upper_bounds: &[i64],
    num_buttons: usize,
) -> usize {
    let num_free = free_vars.len();
    
    // For small number of free variables, enumerate
    // For larger, we need smarter search
    
    // Estimate maximum value for free variables
    let max_free_val = aug.iter()
        .map(|row| row[num_buttons].abs())
        .max()
        .unwrap_or(0);
    
    let mut best = usize::MAX;
    
    // Use recursive search with pruning
    fn search(
        free_idx: usize,
        free_vars: &[usize],
        free_values: &mut Vec<i64>,
        aug: &[Vec<i64>],
        pivot_cols: &[usize],
        pivot_rows: &[usize],
        num_buttons: usize,
        current_sum: i64,
        best: &mut usize,
        max_val: i64,
    ) {
        if free_idx == free_vars.len() {
            // Compute pivot variable values
            let mut solution = vec![0i64; num_buttons];
            for (i, &var) in free_vars.iter().enumerate() {
                solution[var] = free_values[i];
            }
            
            // Back-substitute
            let mut valid = true;
            for i in (0..pivot_cols.len()).rev() {
                let pivot_col = pivot_cols[i];
                let pivot_row = pivot_rows[i];
                let pivot_val = aug[pivot_row][pivot_col];
                
                let mut rhs = aug[pivot_row][num_buttons];
                for j in (pivot_col + 1)..num_buttons {
                    rhs -= aug[pivot_row][j] * solution[j];
                }
                
                if rhs % pivot_val != 0 {
                    valid = false;
                    break;
                }
                
                solution[pivot_col] = rhs / pivot_val;
                
                if solution[pivot_col] < 0 {
                    valid = false;
                    break;
                }
            }
            
            if valid {
                let total: i64 = solution.iter().sum();
                if total >= 0 && (total as usize) < *best {
                    *best = total as usize;
                }
            }
            return;
        }
        
        // Prune if current sum already exceeds best
        if current_sum as usize >= *best {
            return;
        }
        
        // Try values for this free variable
        for val in 0..=max_val {
            free_values[free_idx] = val;
            search(
                free_idx + 1,
                free_vars,
                free_values,
                aug,
                pivot_cols,
                pivot_rows,
                num_buttons,
                current_sum + val,
                best,
                max_val,
            );
        }
    }
    
    let mut free_values = vec![0i64; num_free];
    let search_max = max_free_val.min(500); // Cap the search space
    
    search(
        0,
        free_vars,
        &mut free_values,
        aug,
        pivot_cols,
        pivot_rows,
        num_buttons,
        0,
        &mut best,
        search_max,
    );
    
    if best == usize::MAX {
        panic!("No solution found!");
    }
    
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_part1() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let (target, buttons, _) = parse_machine(line);
        assert_eq!(find_min_presses_part1(&target, &buttons), 2);
    }

    #[test]
    fn test_example_2_part1() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let (target, buttons, _) = parse_machine(line);
        assert_eq!(find_min_presses_part1(&target, &buttons), 3);
    }

    #[test]
    fn test_example_3_part1() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let (target, buttons, _) = parse_machine(line);
        assert_eq!(find_min_presses_part1(&target, &buttons), 2);
    }

    #[test]
    fn test_all_examples_part1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve_part1(&input), 7);
    }

    #[test]
    fn test_example_1_part2() {
        let line = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let (_, buttons, joltage) = parse_machine(line);
        assert_eq!(find_min_presses_part2(&joltage, &buttons), 10);
    }

    #[test]
    fn test_example_2_part2() {
        let line = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let (_, buttons, joltage) = parse_machine(line);
        assert_eq!(find_min_presses_part2(&joltage, &buttons), 12);
    }

    #[test]
    fn test_example_3_part2() {
        let line = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let (_, buttons, joltage) = parse_machine(line);
        assert_eq!(find_min_presses_part2(&joltage, &buttons), 11);
    }

    #[test]
    fn test_all_examples_part2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve_part2(&input), 33);
    }
}
