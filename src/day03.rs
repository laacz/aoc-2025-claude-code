fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn max_joltage(bank: &[u32]) -> u32 {
    let n = bank.len();
    // Compute max suffix: max_suffix[i] = max of bank[i..n]
    let mut max_suffix = vec![0; n + 1];
    for i in (0..n).rev() {
        max_suffix[i] = bank[i].max(max_suffix[i + 1]);
    }

    let mut max_val = 0;
    for i in 0..n - 1 {
        let value = bank[i] * 10 + max_suffix[i + 1];
        max_val = max_val.max(value);
    }
    max_val
}

fn part1(input: &str) -> u32 {
    let banks = parse(input);
    banks.iter().map(|bank| max_joltage(bank)).sum()
}

fn max_joltage_k(bank: &[u32], k: usize) -> u64 {
    let n = bank.len();
    let mut result = 0u64;
    let mut start = 0;

    for remaining in (1..=k).rev() {
        let end = n - remaining; // inclusive
        // Find max in bank[start..=end], pick leftmost max for more future options
        let mut best_idx = start;
        let mut best_val = bank[start];
        for i in start + 1..=end {
            if bank[i] > best_val {
                best_val = bank[i];
                best_idx = i;
            }
        }

        result = result * 10 + best_val as u64;
        start = best_idx + 1;
    }

    result
}

fn part2(input: &str) -> u64 {
    let banks = parse(input);
    banks.iter().map(|bank| max_joltage_k(bank, 12)).sum()
}

fn main() {
    let input = std::fs::read_to_string("data/03.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]), 98);
        assert_eq!(max_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]), 89);
        assert_eq!(max_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]), 78);
        assert_eq!(max_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]), 92);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
    }

    #[test]
    fn test_max_joltage_k() {
        assert_eq!(max_joltage_k(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12), 987654321111);
        assert_eq!(max_joltage_k(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12), 811111111119);
        assert_eq!(max_joltage_k(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12), 434234234278);
        assert_eq!(max_joltage_k(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12), 888911112111);
    }
}
