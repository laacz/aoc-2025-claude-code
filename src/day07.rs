use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start = (0, 0);
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                start = (row, col);
            }
        }
    }
    (grid, start)
}

fn part1(input: &str) -> usize {
    let (grid, start) = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();

    // Track positions where beams are active (beams merge at same position)
    let mut beam_positions: HashSet<usize> = HashSet::new();
    beam_positions.insert(start.1);

    let mut splits = 0;

    // Process row by row, starting from S
    for row in start.0..rows {
        let mut next_positions: HashSet<usize> = HashSet::new();

        for &col in &beam_positions {
            if grid[row][col] == '^' {
                // Beam hits splitter - count this split
                splits += 1;

                // Spawn beams to left and right (they continue on same row, then down)
                if col > 0 {
                    next_positions.insert(col - 1);
                }
                if col + 1 < cols {
                    next_positions.insert(col + 1);
                }
            } else {
                // Beam continues through this position
                next_positions.insert(col);
            }
        }

        beam_positions = next_positions;
    }

    splits
}

fn part2(input: &str) -> u64 {
    let (grid, start) = parse(input);
    let rows = grid.len();
    let cols = grid[0].len();

    // Track number of timelines at each position
    let mut timelines: HashMap<usize, u64> = HashMap::new();
    timelines.insert(start.1, 1);

    // Process row by row, starting from S
    for row in start.0..rows {
        let mut next_timelines: HashMap<usize, u64> = HashMap::new();

        for (&col, &count) in &timelines {
            if grid[row][col] == '^' {
                // Each timeline splits into two (left and right)
                if col > 0 {
                    *next_timelines.entry(col - 1).or_insert(0) += count;
                }
                if col + 1 < cols {
                    *next_timelines.entry(col + 1).or_insert(0) += count;
                }
            } else {
                // Timeline continues through this position
                *next_timelines.entry(col).or_insert(0) += count;
            }
        }

        timelines = next_timelines;
    }

    // Sum all timelines at the end
    timelines.values().sum()
}

fn main() {
    let input = std::fs::read_to_string("data/07.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 40);
    }
}
