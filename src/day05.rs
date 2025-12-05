use std::fs;

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|line| {
            let mut nums = line.split('-');
            let start: u64 = nums.next().unwrap().parse().unwrap();
            let end: u64 = nums.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    let ingredients: Vec<u64> = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ingredients)
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

fn part1(input: &str) -> usize {
    let (ranges, ingredients) = parse(input);
    ingredients.iter().filter(|&&id| is_fresh(id, &ranges)).count()
}

fn merge_ranges(ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut sorted: Vec<(u64, u64)> = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    for (start, end) in sorted {
        if let Some(last) = merged.last_mut() {
            // Check if current range overlaps or is adjacent to the last merged range
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }
    merged
}

fn part2(input: &str) -> u64 {
    let (ranges, _) = parse(input);
    let merged = merge_ranges(&ranges);
    merged.iter().map(|(start, end)| end - start + 1).sum()
}

fn main() {
    let input = fs::read_to_string("data/05.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 14);
    }
}
