fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> usize {
    let grid = parse(input);
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            // Count adjacent paper rolls
            let mut adjacent = 0;
            for dr in -1..=1i32 {
                for dc in -1..=1i32 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                        if grid[nr as usize][nc as usize] == '@' {
                            adjacent += 1;
                        }
                    }
                }
            }

            // Accessible if fewer than 4 adjacent rolls
            if adjacent < 4 {
                count += 1;
            }
        }
    }

    count
}

fn count_adjacent(grid: &Vec<Vec<char>>, r: usize, c: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut adjacent = 0;
    for dr in -1..=1i32 {
        for dc in -1..=1i32 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                if grid[nr as usize][nc as usize] == '@' {
                    adjacent += 1;
                }
            }
        }
    }
    adjacent
}

fn part2(input: &str) -> usize {
    let mut grid = parse(input);
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };

    let mut total_removed = 0;

    loop {
        // Find all accessible rolls (fewer than 4 adjacent)
        let mut to_remove = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' && count_adjacent(&grid, r, c) < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (r, c) in &to_remove {
            grid[*r][*c] = '.';
        }
        total_removed += to_remove.len();
    }

    total_removed
}

fn main() {
    let input = std::fs::read_to_string("data/04.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 43);
    }
}
