fn main() {
    let input = std::fs::read_to_string("data/06.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<(char, Vec<u64>)> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    // Find max width
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Pad lines to max width
    let padded: Vec<String> = lines.iter().map(|l| format!("{:width$}", l, width = max_width)).collect();

    // The last non-empty line contains operators
    let operator_line = padded.iter().rev().find(|l| !l.trim().is_empty()).unwrap();

    // Parse columns into problems
    let mut problems: Vec<(char, Vec<u64>)> = vec![];
    let mut col = 0;

    while col < max_width {
        // Skip separator columns (all spaces)
        let is_separator = padded.iter().all(|line| {
            line.chars().nth(col).map_or(true, |c| c == ' ')
        });

        if is_separator {
            col += 1;
            continue;
        }

        // Find the extent of this problem (consecutive non-separator columns)
        let start_col = col;
        while col < max_width {
            let is_sep = padded.iter().all(|line| {
                line.chars().nth(col).map_or(true, |c| c == ' ')
            });
            if is_sep {
                break;
            }
            col += 1;
        }
        let end_col = col;

        // Extract the problem: get the operator and numbers
        let problem_str: String = operator_line.chars().skip(start_col).take(end_col - start_col).collect();
        let op = problem_str.trim().chars().next().unwrap_or('+');

        let mut numbers = vec![];
        for line in &padded {
            if line == operator_line {
                continue;
            }
            let num_str: String = line.chars().skip(start_col).take(end_col - start_col).collect();
            let num_str = num_str.trim();
            if !num_str.is_empty() {
                if let Ok(n) = num_str.parse::<u64>() {
                    numbers.push(n);
                }
            }
        }

        if !numbers.is_empty() {
            problems.push((op, numbers));
        }
    }

    problems
}

fn part1(input: &str) -> u64 {
    let problems = parse(input);
    problems
        .iter()
        .map(|(op, nums)| {
            match op {
                '+' => nums.iter().sum(),
                '*' => nums.iter().product(),
                _ => 0,
            }
        })
        .sum()
}

fn parse2(input: &str) -> Vec<(char, Vec<u64>)> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return vec![];
    }

    // Find max width
    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Pad lines to max width
    let padded: Vec<String> = lines
        .iter()
        .map(|l| format!("{:width$}", l, width = max_width))
        .collect();

    // The last non-empty line contains operators
    let op_line_idx = padded
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .unwrap();

    // Read columns right-to-left, grouping by problems (separated by all-space columns)
    let mut problems: Vec<(char, Vec<u64>)> = vec![];
    let mut col = max_width as i32 - 1;

    while col >= 0 {
        let c = col as usize;

        // Skip separator columns (all spaces)
        let is_separator = padded.iter().all(|line| {
            line.chars().nth(c).map_or(true, |ch| ch == ' ')
        });

        if is_separator {
            col -= 1;
            continue;
        }

        // Find extent of this problem (going left until separator)
        let end_col = c; // rightmost column of problem
        while col >= 0 {
            let cc = col as usize;
            let is_sep = padded.iter().all(|line| {
                line.chars().nth(cc).map_or(true, |ch| ch == ' ')
            });
            if is_sep {
                break;
            }
            col -= 1;
        }
        let start_col = (col + 1) as usize;

        // Find operator for this problem
        let op_str: String = padded[op_line_idx]
            .chars()
            .skip(start_col)
            .take(end_col - start_col + 1)
            .collect();
        let op = op_str.trim().chars().next().unwrap_or('+');

        // Each column within the problem is a number (digits top to bottom, MSD first)
        // Read columns right-to-left within the problem
        let mut numbers = vec![];
        for problem_col in (start_col..=end_col).rev() {
            let mut num_str = String::new();
            for (line_idx, line) in padded.iter().enumerate() {
                if line_idx == op_line_idx {
                    continue;
                }
                if let Some(ch) = line.chars().nth(problem_col) {
                    if ch.is_ascii_digit() {
                        num_str.push(ch);
                    }
                }
            }
            if !num_str.is_empty() {
                if let Ok(n) = num_str.parse::<u64>() {
                    numbers.push(n);
                }
            }
        }

        if !numbers.is_empty() {
            problems.push((op, numbers));
        }
    }

    problems
}

fn part2(input: &str) -> u64 {
    let problems = parse2(input);
    problems
        .iter()
        .map(|(op, nums)| match op {
            '+' => nums.iter().sum(),
            '*' => nums.iter().product(),
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        assert_eq!(part1(input), 4277556);
    }

    #[test]
    fn test_individual_problems() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        let problems = parse(input);
        assert_eq!(problems.len(), 4);

        // 123 * 45 * 6 = 33210
        assert_eq!(problems[0], ('*', vec![123, 45, 6]));

        // 328 + 64 + 98 = 490
        assert_eq!(problems[1], ('+', vec![328, 64, 98]));

        // 51 * 387 * 215 = 4243455
        assert_eq!(problems[2], ('*', vec![51, 387, 215]));

        // 64 + 23 + 314 = 401
        assert_eq!(problems[3], ('+', vec![64, 23, 314]));
    }

    #[test]
    fn test_example_part2() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        assert_eq!(part2(input), 3263827);
    }

    #[test]
    fn test_individual_problems_part2() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +";
        let problems = parse2(input);

        // Reading right to left:
        // Rightmost: 4 + 431 + 623 = 1058
        // Second: 175 * 581 * 32 = 3253600
        // Third: 8 + 248 + 369 = 625
        // Fourth: 356 * 24 * 1 = 8544
        assert_eq!(problems.len(), 4);
        assert_eq!(problems[0], ('+', vec![4, 431, 623]));
        assert_eq!(problems[1], ('*', vec![175, 581, 32]));
        assert_eq!(problems[2], ('+', vec![8, 248, 369]));
        assert_eq!(problems[3], ('*', vec![356, 24, 1]));
    }
}
