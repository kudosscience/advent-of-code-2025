use std::fs;
use std::collections::HashSet;

fn is_repeated_sequence(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    // Try all possible base sequence lengths (1 to len/2)
    // The sequence must repeat at least twice, so base length <= len/2
    for base_len in 1..=len / 2 {
        // The total length must be divisible by the base length
        if len % base_len != 0 {
            continue;
        }
        
        let base = &s[..base_len];
        
        // Base sequence cannot start with '0'
        if base.starts_with('0') {
            continue;
        }
        
        // Check if the entire string is made of this base repeated
        let repeat_count = len / base_len;
        let repeated = base.repeat(repeat_count);
        
        if repeated == s {
            return true;
        }
    }
    
    false
}

fn find_invalid_ids_in_range(start: u64, end: u64) -> Vec<u64> {
    let mut invalid_ids_set = HashSet::new();
    
    // For efficiency, we generate repeated sequences and check if they're in range
    // rather than checking every number in the range
    
    // Determine the digit lengths we need to consider
    let start_str = start.to_string();
    let end_str = end.to_string();
    let min_len = start_str.len();
    let max_len = end_str.len();
    
    for total_len in min_len..=max_len {
        // Try all possible base lengths that divide total_len
        // and allow for at least 2 repetitions
        for base_len in 1..=total_len / 2 {
            if total_len % base_len != 0 {
                continue;
            }
            
            let repeat_count = total_len / base_len;
            if repeat_count < 2 {
                continue;
            }
            
            // Generate all base sequences of the required length
            let base_start = if base_len == 1 { 1 } else { 10u64.pow((base_len - 1) as u32) };
            let base_end = 10u64.pow(base_len as u32) - 1;
            
            for base in base_start..=base_end {
                // Create the repeated number
                let base_str = base.to_string();
                let repeated_str = base_str.repeat(repeat_count);
                
                // Check if the repeated string has the expected length
                if repeated_str.len() != total_len {
                    continue;
                }
                
                let repeated: u64 = repeated_str.parse().unwrap();
                
                if repeated >= start && repeated <= end {
                    invalid_ids_set.insert(repeated);
                }
            }
        }
    }
    
    invalid_ids_set.into_iter().collect()
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let input = input.trim();
    let mut ranges = Vec::new();
    
    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = part.split('-').collect();
        if parts.len() == 2 {
            let start: u64 = parts[0].parse().expect("Invalid start number");
            let end: u64 = parts[1].parse().expect("Invalid end number");
            ranges.push((start, end));
        }
    }
    
    ranges
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let ranges = parse_input(&input);
    
    let mut total_sum: u64 = 0;
    
    for (start, end) in ranges {
        let invalid_ids = find_invalid_ids_in_range(start, end);
        for id in invalid_ids {
            total_sum += id;
        }
    }
    
    println!("Sum of all invalid IDs: {}", total_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_repeated_sequence() {
        // Part 2: sequences repeated at least twice
        assert!(is_repeated_sequence(55));
        assert!(is_repeated_sequence(6464));
        assert!(is_repeated_sequence(123123));
        assert!(is_repeated_sequence(11));
        assert!(is_repeated_sequence(22));
        assert!(is_repeated_sequence(99));
        assert!(is_repeated_sequence(1010));
        
        // New cases for part 2
        assert!(is_repeated_sequence(111));      // 1 repeated 3 times
        assert!(is_repeated_sequence(999));      // 9 repeated 3 times
        assert!(is_repeated_sequence(1111111));  // 1 repeated 7 times
        assert!(is_repeated_sequence(123123123)); // 123 repeated 3 times
        assert!(is_repeated_sequence(1212121212)); // 12 repeated 5 times
        assert!(is_repeated_sequence(565656));   // 56 repeated 3 times
        assert!(is_repeated_sequence(824824824)); // 824 repeated 3 times
        
        assert!(!is_repeated_sequence(101));
        assert!(!is_repeated_sequence(12));
        assert!(!is_repeated_sequence(123));
        assert!(!is_repeated_sequence(1234));
    }

    #[test]
    fn test_example_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges = parse_input(input);
        
        let mut total_sum: u64 = 0;
        for (start, end) in ranges {
            let invalid_ids = find_invalid_ids_in_range(start, end);
            for id in &invalid_ids {
                total_sum += id;
            }
        }
        
        assert_eq!(total_sum, 4174379265);
    }

    #[test]
    fn test_individual_ranges_part2() {
        // 11-22 still has two invalid IDs, 11 and 22
        let ids = find_invalid_ids_in_range(11, 22);
        assert!(ids.contains(&11));
        assert!(ids.contains(&22));
        assert_eq!(ids.len(), 2);

        // 95-115 now has two invalid IDs, 99 and 111
        let ids = find_invalid_ids_in_range(95, 115);
        assert!(ids.contains(&99));
        assert!(ids.contains(&111));
        assert_eq!(ids.len(), 2);

        // 998-1012 now has two invalid IDs, 999 and 1010
        let ids = find_invalid_ids_in_range(998, 1012);
        assert!(ids.contains(&999));
        assert!(ids.contains(&1010));
        assert_eq!(ids.len(), 2);

        // 1188511880-1188511890 still has one invalid ID, 1188511885
        let ids = find_invalid_ids_in_range(1188511880, 1188511890);
        assert!(ids.contains(&1188511885));
        assert_eq!(ids.len(), 1);

        // 222220-222224 still has one invalid ID, 222222
        let ids = find_invalid_ids_in_range(222220, 222224);
        assert!(ids.contains(&222222));
        assert_eq!(ids.len(), 1);

        // 1698522-1698528 still contains no invalid IDs
        let ids = find_invalid_ids_in_range(1698522, 1698528);
        assert_eq!(ids.len(), 0);

        // 446443-446449 still has one invalid ID, 446446
        let ids = find_invalid_ids_in_range(446443, 446449);
        assert!(ids.contains(&446446));
        assert_eq!(ids.len(), 1);

        // 38593856-38593862 still has one invalid ID, 38593859
        let ids = find_invalid_ids_in_range(38593856, 38593862);
        assert!(ids.contains(&38593859));
        assert_eq!(ids.len(), 1);

        // 565653-565659 now has one invalid ID, 565656
        let ids = find_invalid_ids_in_range(565653, 565659);
        assert!(ids.contains(&565656));
        assert_eq!(ids.len(), 1);

        // 824824821-824824827 now has one invalid ID, 824824824
        let ids = find_invalid_ids_in_range(824824821, 824824827);
        assert!(ids.contains(&824824824));
        assert_eq!(ids.len(), 1);

        // 2121212118-2121212124 now has one invalid ID, 2121212121
        let ids = find_invalid_ids_in_range(2121212118, 2121212124);
        assert!(ids.contains(&2121212121));
        assert_eq!(ids.len(), 1);
    }
}
