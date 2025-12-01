fn main() {
    let str = std::fs::read_to_string("data/01.txt").expect("Failed to read input file");
    let input = parse(str);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn parse(input: String) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|line| {
            let dir = line.chars().next().unwrap();
            let dist: i32 = line[1..].parse().unwrap();
            (dir, dist)
        })
        .collect()
}

fn part1(input: &[(char, i32)]) -> i32 {
    let mut position: i32 = 50;
    let mut count = 0;

    for &(dir, dist) in input {
        match dir {
            'L' => position = (position - dist).rem_euclid(100),
            'R' => position = (position + dist).rem_euclid(100),
            _ => panic!("Invalid direction"),
        }
        if position == 0 {
            count += 1;
        }
    }

    count
}

fn part2(input: &[(char, i32)]) -> i32 {
    let mut position: i32 = 50;
    let mut count = 0;

    for &(dir, dist) in input {
        // Count how many times we pass through 0 during this rotation
        count += match dir {
            'R' => (position + dist) / 100,
            'L' => {
                if position == 0 {
                    dist / 100
                } else if dist >= position {
                    1 + (dist - position) / 100
                } else {
                    0
                }
            }
            _ => panic!("Invalid direction"),
        };

        // Update position
        position = match dir {
            'L' => (position - dist).rem_euclid(100),
            'R' => (position + dist).rem_euclid(100),
            _ => panic!("Invalid direction"),
        };
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = parse("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string());
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = parse("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string());
        assert_eq!(part2(&input), 6);
    }
}
