use std::fs;

const NUM_BATTERIES_PART1: usize = 2;
const NUM_BATTERIES_PART2: usize = 12;

/// Find the maximum number formed by selecting exactly `k` digits from the bank
/// while maintaining their relative order.
/// 
/// Uses a greedy approach: at each step, pick the largest digit possible
/// from the valid range (ensuring enough digits remain for the rest).
fn max_joltage_from_bank(bank: &str, k: usize) -> u64 {
    let digits: Vec<u64> = bank.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u64)
        .collect();
    
    let n = digits.len();
    if k > n {
        return 0;
    }
    
    let mut result: u64 = 0;
    let mut start = 0;
    
    // We need to pick k digits. For each position in our result:
    // - We need to leave enough digits for the remaining positions
    // - Pick the maximum digit in the valid range
    for remaining in (1..=k).rev() {
        // We can pick from start to (n - remaining) inclusive
        let end = n - remaining;
        
        // Find the maximum digit in range [start, end]
        let mut best_idx = start;
        let mut best_digit = digits[start];
        for i in start..=end {
            if digits[i] > best_digit {
                best_digit = digits[i];
                best_idx = i;
            }
        }
        
        result = result * 10 + best_digit;
        start = best_idx + 1;
    }
    
    result
}

fn solve_part1(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| max_joltage_from_bank(line, NUM_BATTERIES_PART1))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| max_joltage_from_bank(line, NUM_BATTERIES_PART2))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");
    
    let part1 = solve_part1(&input);
    println!("Part 1 - Total output joltage: {}", part1);
    
    let part2 = solve_part2(&input);
    println!("Part 2 - Total output joltage: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(solve_part1(input), 357);
    }

    #[test]
    fn test_part1_individual_banks() {
        assert_eq!(max_joltage_from_bank("987654321111111", 2), 98);
        assert_eq!(max_joltage_from_bank("811111111111119", 2), 89);
        assert_eq!(max_joltage_from_bank("234234234234278", 2), 78);
        assert_eq!(max_joltage_from_bank("818181911112111", 2), 92);
    }

    #[test]
    fn test_part2_example() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(solve_part2(input), 3121910778619);
    }

    #[test]
    fn test_part2_individual_banks() {
        assert_eq!(max_joltage_from_bank("987654321111111", 12), 987654321111);
        assert_eq!(max_joltage_from_bank("811111111111119", 12), 811111111119);
        assert_eq!(max_joltage_from_bank("234234234234278", 12), 434234234278);
        assert_eq!(max_joltage_from_bank("818181911112111", 12), 888911112111);
    }
}
