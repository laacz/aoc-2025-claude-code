use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("data/02.txt").unwrap();
    println!("part1: {}", part1(input.trim()));
    println!("part2: {}", part2(input.trim()));
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            (start, end)
        })
        .collect()
}

fn part1(input: &str) -> u64 {
    let ranges = parse(input);

    // Find max value in any range
    let max_val = ranges.iter().map(|r| r.1).max().unwrap_or(0);

    // Generate all repeated pattern numbers up to max_val
    let mut invalid_ids: Vec<u64> = Vec::new();

    // Generate patterns of length 2, 4, 6, 8, 10, ... (half_len digits repeated twice)
    for half_len in 1..=10 {
        let start = 10u64.pow(half_len - 1);
        let end = 10u64.pow(half_len);
        for prefix in start..end {
            let pattern = format!("{}{}", prefix, prefix);
            let num: u64 = pattern.parse().unwrap();
            if num > max_val {
                break;
            }
            invalid_ids.push(num);
        }
    }

    // Sum all invalid IDs that appear in any range
    let mut sum = 0u64;
    for &id in &invalid_ids {
        for &(start, end) in &ranges {
            if id >= start && id <= end {
                sum += id;
                break; // Don't count the same ID twice
            }
        }
    }

    sum
}

fn part2(input: &str) -> u64 {
    let ranges = parse(input);
    let max_val = ranges.iter().map(|r| r.1).max().unwrap_or(0);

    // Use HashSet to avoid counting duplicates (e.g., 1111 = "1"x4 or "11"x2)
    let mut invalid_ids: HashSet<u64> = HashSet::new();

    // Generate patterns: pattern_len digits repeated rep_count times
    for pattern_len in 1..=5 {
        for rep_count in 2..=(10 / pattern_len) {
            let total_len = pattern_len * rep_count;
            if total_len > 10 {
                break;
            }

            // Generate all patterns of pattern_len digits (no leading zeros)
            let start = if pattern_len == 1 {
                1
            } else {
                10u64.pow((pattern_len - 1) as u32)
            };
            let end = 10u64.pow(pattern_len as u32);

            for prefix in start..end {
                let pattern = prefix.to_string();
                let repeated = pattern.repeat(rep_count);
                let num: u64 = repeated.parse().unwrap();
                if num <= max_val {
                    invalid_ids.insert(num);
                }
            }
        }
    }

    // Sum all invalid IDs that appear in any range
    let mut sum = 0u64;
    for &id in &invalid_ids {
        for &(start, end) in &ranges {
            if id >= start && id <= end {
                sum += id;
                break;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part2(input), 4174379265);
    }
}
