use std::fs;

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

fn parse(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        // Parse indicator lights [.##.]
        let bracket_start = line.find('[').unwrap();
        let bracket_end = line.find(']').unwrap();
        let lights_str = &line[bracket_start + 1..bracket_end];
        let lights: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();

        // Parse buttons (0,1,2) etc - everything in parentheses before the curly brace
        let curly_start = line.find('{').unwrap();
        let buttons_section = &line[bracket_end + 1..curly_start];

        let mut buttons = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = buttons_section.chars().collect();
        while i < chars.len() {
            if chars[i] == '(' {
                let mut j = i + 1;
                while chars[j] != ')' {
                    j += 1;
                }
                let inner = &buttons_section[i + 1..j];
                let indices: Vec<usize> = inner
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                buttons.push(indices);
                i = j + 1;
            } else {
                i += 1;
            }
        }

        // Parse joltage requirements {3,5,4,7}
        let curly_end = line.find('}').unwrap();
        let joltage_str = &line[curly_start + 1..curly_end];
        let joltage: Vec<i64> = joltage_str
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        machines.push(Machine {
            lights,
            buttons,
            joltage,
        });
    }

    machines
}

fn solve_machine(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let n = target.len();
    let m = buttons.len();

    // We need to find the minimum number of button presses
    // Each button press is either 0 or 1 (since pressing twice is same as not pressing)
    // This is a system of linear equations over GF(2)
    // We want minimum Hamming weight solution

    // Try all 2^m combinations (brute force for small m)
    // For larger m, we'd need Gaussian elimination + enumerate null space

    if m <= 20 {
        let mut min_presses = usize::MAX;

        for mask in 0u32..(1 << m) {
            let mut state = vec![false; n];

            for (i, button) in buttons.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    for &idx in button {
                        state[idx] = !state[idx];
                    }
                }
            }

            if state == target {
                let presses = mask.count_ones() as usize;
                min_presses = min_presses.min(presses);
            }
        }

        min_presses
    } else {
        // Use Gaussian elimination for larger cases
        solve_with_gauss(target, buttons)
    }
}

fn solve_with_gauss(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let n = target.len();
    let m = buttons.len();

    // Build augmented matrix [A | b] where A is n x m matrix
    // A[i][j] = 1 if button j toggles light i
    // b[i] = target[i]

    // We'll represent each row as a vector of bools (including augmented column)
    let mut matrix: Vec<Vec<bool>> = vec![vec![false; m + 1]; n];

    for (j, button) in buttons.iter().enumerate() {
        for &idx in button {
            matrix[idx][j] = true;
        }
    }
    for i in 0..n {
        matrix[i][m] = target[i];
    }

    // Gaussian elimination
    let mut pivot_cols = Vec::new();
    let mut row = 0;

    for col in 0..m {
        // Find pivot
        let mut pivot_row = None;
        for r in row..n {
            if matrix[r][col] {
                pivot_row = Some(r);
                break;
            }
        }

        if let Some(pr) = pivot_row {
            matrix.swap(row, pr);
            pivot_cols.push(col);

            // Eliminate
            for r in 0..n {
                if r != row && matrix[r][col] {
                    for c in 0..=m {
                        matrix[r][c] ^= matrix[row][c];
                    }
                }
            }
            row += 1;
        }
    }

    let rank = pivot_cols.len();

    // Check consistency
    for r in rank..n {
        if matrix[r][m] {
            return usize::MAX; // No solution
        }
    }

    // Free variables are columns not in pivot_cols
    let mut free_cols: Vec<usize> = Vec::new();
    let pivot_set: std::collections::HashSet<usize> = pivot_cols.iter().cloned().collect();
    for col in 0..m {
        if !pivot_set.contains(&col) {
            free_cols.push(col);
        }
    }

    let num_free = free_cols.len();
    let mut min_presses = usize::MAX;

    // Enumerate all 2^num_free assignments to free variables
    for free_mask in 0u64..(1 << num_free) {
        let mut solution = vec![false; m];

        // Set free variables
        for (i, &col) in free_cols.iter().enumerate() {
            solution[col] = (free_mask >> i) & 1 == 1;
        }

        // Back-substitute to find pivot variables
        for (i, &col) in pivot_cols.iter().enumerate().rev() {
            let mut val = matrix[i][m];
            for j in (col + 1)..m {
                if matrix[i][j] {
                    val ^= solution[j];
                }
            }
            solution[col] = val;
        }

        let presses: usize = solution.iter().filter(|&&x| x).count();
        min_presses = min_presses.min(presses);
    }

    min_presses
}

fn part1(input: &str) -> usize {
    let machines = parse(input);
    machines
        .iter()
        .map(|m| solve_machine(&m.lights, &m.buttons))
        .sum()
}

// Solve Ax = b over integers with x >= 0, minimizing sum(x)
// This is Integer Linear Programming, but with special structure:
// A is 0/1 matrix, we want to minimize sum of button presses
fn solve_joltage(buttons: &[Vec<usize>], target: &[i64]) -> i64 {
    let n = target.len();
    let m = buttons.len();

    // Use Gaussian elimination + search
    solve_joltage_gauss(buttons, target, n, m)
}

fn solve_joltage_gauss(buttons: &[Vec<usize>], target: &[i64], n: usize, m: usize) -> i64 {
    // Build augmented matrix [A | b]
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; m + 1]; n];
    for (j, button) in buttons.iter().enumerate() {
        for &idx in button {
            matrix[idx][j] = 1;
        }
    }
    for i in 0..n {
        matrix[i][m] = target[i];
    }

    // Gaussian elimination
    let mut pivot_cols = Vec::new();
    let mut row = 0;

    for col in 0..m {
        let mut pivot_row = None;
        for r in row..n {
            if matrix[r][col] != 0 {
                pivot_row = Some(r);
                break;
            }
        }

        if let Some(pr) = pivot_row {
            matrix.swap(row, pr);
            pivot_cols.push(col);

            let pivot_val = matrix[row][col];

            for r in 0..n {
                if r != row && matrix[r][col] != 0 {
                    let factor = matrix[r][col];
                    for c in 0..=m {
                        matrix[r][c] = matrix[r][c] * pivot_val - matrix[row][c] * factor;
                    }
                    let g = matrix[r].iter().fold(0i64, |acc, &x| gcd(acc, x.abs()));
                    if g > 1 {
                        for c in 0..=m {
                            matrix[r][c] /= g;
                        }
                    }
                }
            }
            row += 1;
        }
    }

    let rank = pivot_cols.len();

    // Check consistency
    for r in rank..n {
        if matrix[r][m] != 0 {
            return i64::MAX;
        }
    }

    // Identify free variables
    let pivot_set: std::collections::HashSet<usize> = pivot_cols.iter().cloned().collect();
    let free_cols: Vec<usize> = (0..m).filter(|c| !pivot_set.contains(c)).collect();

    if free_cols.is_empty() {
        // Unique solution
        let mut solution = vec![0i64; m];
        for (i, &col) in pivot_cols.iter().enumerate() {
            let pivot_val = matrix[i][col];
            let rhs = matrix[i][m];
            if rhs % pivot_val != 0 {
                return i64::MAX;
            }
            solution[col] = rhs / pivot_val;
            if solution[col] < 0 {
                return i64::MAX;
            }
        }
        return solution.iter().sum();
    }

    // Get particular solution and null space info
    let mut denominators = vec![1i64; m];
    for (i, &col) in pivot_cols.iter().enumerate() {
        denominators[col] = matrix[i][col];
    }

    let lcm_denom = denominators.iter().fold(1i64, |acc, &d| lcm(acc, d.abs()));

    // Compute scaled particular solution
    let mut part_scaled = vec![0i64; m];
    for (i, &col) in pivot_cols.iter().enumerate() {
        part_scaled[col] = matrix[i][m] * (lcm_denom / denominators[col]);
    }

    // Compute scaled null vectors
    let mut null_scaled: Vec<Vec<i64>> = Vec::new();
    for &free_col in &free_cols {
        let mut null_vec = vec![0i64; m];
        null_vec[free_col] = lcm_denom;
        for (i, &pivot_col) in pivot_cols.iter().enumerate() {
            null_vec[pivot_col] = -matrix[i][free_col] * (lcm_denom / denominators[pivot_col]);
        }
        null_scaled.push(null_vec);
    }

    // Optimize using coordinate descent with smart bounds
    let num_free = free_cols.len();
    let mut best = i64::MAX;

    // For small number of free variables, enumerate all valid combinations
    if num_free <= 4 {
        // Small enumeration
        let max_val = target.iter().max().copied().unwrap_or(0);
        let bound = (max_val + 50).max(300);

        fn enumerate(
            idx: usize,
            t: &mut Vec<i64>,
            part: &[i64],
            null_vecs: &[Vec<i64>],
            lcm: i64,
            m: usize,
            bound: i64,
            best: &mut i64,
        ) {
            if idx == t.len() {
                let val = eval_solution_fast(t, part, null_vecs, lcm, m);
                if val < *best {
                    *best = val;
                }
                return;
            }
            for ti in -bound..=bound {
                t[idx] = ti;
                enumerate(idx + 1, t, part, null_vecs, lcm, m, bound, best);
            }
        }

        let mut t = vec![0i64; num_free];
        enumerate(
            0,
            &mut t,
            &part_scaled,
            &null_scaled,
            lcm_denom,
            m,
            bound,
            &mut best,
        );
        return best;
    }

    // For larger number of free variables, use coordinate descent
    let mut t = vec![0i64; num_free];

    // First, find a valid starting point using greedy approach
    for _ in 0..1000 {
        let mut valid = true;
        for j in 0..m {
            let mut val = part_scaled[j];
            for (i, &ti) in t.iter().enumerate() {
                val += ti * null_scaled[i][j];
            }
            if val < 0 {
                valid = false;
                // Find which t[i] can help increase this value
                for i in 0..num_free {
                    if null_scaled[i][j] > 0 {
                        t[i] += (-val / null_scaled[i][j]).max(1);
                        break;
                    } else if null_scaled[i][j] < 0 {
                        t[i] -= (-val / (-null_scaled[i][j])).max(1);
                        break;
                    }
                }
                break;
            }
        }
        if valid {
            let val = eval_solution_fast(&t, &part_scaled, &null_scaled, lcm_denom, m);
            if val < best {
                best = val;
            }
            break;
        }
    }

    // Local search - coordinate descent with proper bound computation
    let mut improved = true;
    let mut iterations = 0;
    while improved && iterations < 100 {
        improved = false;
        iterations += 1;

        for i in 0..num_free {
            // Find valid range for t[i]
            let mut min_t = i64::MIN / 2;
            let mut max_t = i64::MAX / 2;

            for j in 0..m {
                let base = part_scaled[j]
                    + t.iter()
                        .enumerate()
                        .filter(|(k, _)| *k != i)
                        .map(|(k, &tk)| tk * null_scaled[k][j])
                        .sum::<i64>();

                let coef = null_scaled[i][j];
                if coef > 0 {
                    // base + coef * t[i] >= 0  =>  t[i] >= -base/coef
                    // Use ceiling division: ceil(-base / coef)
                    let bound = div_ceil(-base, coef);
                    min_t = min_t.max(bound);
                } else if coef < 0 {
                    // base + coef * t[i] >= 0  =>  t[i] <= -base/coef (since coef < 0, inequality flips)
                    // Use floor division: floor(-base / coef)
                    let bound = div_floor(-base, coef);
                    max_t = max_t.min(bound);
                } else if base < 0 {
                    min_t = 1;
                    max_t = 0;
                }
            }

            if min_t > max_t {
                continue;
            }

            // The objective changes linearly with t[i]
            let deriv: i64 = null_scaled[i].iter().sum();

            let candidates = if deriv > 0 {
                vec![min_t]
            } else if deriv < 0 {
                vec![max_t]
            } else {
                vec![min_t, max_t, t[i]]
            };

            for &candidate in &candidates {
                if candidate >= min_t && candidate <= max_t {
                    let old_ti = t[i];
                    t[i] = candidate;
                    let val = eval_solution_fast(&t, &part_scaled, &null_scaled, lcm_denom, m);
                    if val < best {
                        best = val;
                        improved = true;
                    } else {
                        t[i] = old_ti;
                    }
                }
            }
        }
    }

    // Also try from multiple starting points
    for start in 0..20 {
        let mut t2 = vec![0i64; num_free];
        for i in 0..num_free {
            t2[i] = (start as i64 * 17 + i as i64 * 31) % 201 - 100;
        }

        // Try to make it valid
        for _ in 0..100 {
            let mut all_valid = true;
            for j in 0..m {
                let mut val = part_scaled[j];
                for (i, &ti) in t2.iter().enumerate() {
                    val += ti * null_scaled[i][j];
                }
                if val < 0 {
                    all_valid = false;
                    for i in 0..num_free {
                        if null_scaled[i][j] > 0 {
                            t2[i] += 1;
                            break;
                        } else if null_scaled[i][j] < 0 {
                            t2[i] -= 1;
                            break;
                        }
                    }
                    break;
                }
            }
            if all_valid {
                break;
            }
        }

        let val = eval_solution_fast(&t2, &part_scaled, &null_scaled, lcm_denom, m);
        if val < best {
            best = val;
        }
    }

    best
}

fn eval_solution_fast(t: &[i64], part: &[i64], null_vecs: &[Vec<i64>], lcm: i64, m: usize) -> i64 {
    let mut sum = 0i64;
    for j in 0..m {
        let mut val = part[j];
        for (i, &ti) in t.iter().enumerate() {
            val += ti * null_vecs[i][j];
        }
        if val < 0 || val % lcm != 0 {
            return i64::MAX;
        }
        sum += val / lcm;
    }
    sum
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

// Ceiling division: ceil(a / b) where b > 0
fn div_ceil(a: i64, b: i64) -> i64 {
    assert!(b > 0);
    if a >= 0 {
        (a + b - 1) / b
    } else {
        // For negative a, Rust's / truncates toward zero, which is ceiling
        a / b
    }
}

// Floor division: floor(a / b) where b < 0
fn div_floor(a: i64, b: i64) -> i64 {
    assert!(b < 0);
    // a / b where b < 0
    // We want floor division (toward negative infinity)
    // Rust truncates toward zero

    // Convert to positive divisor problem
    // floor(a / b) where b < 0 = floor(a / b)
    // = -ceil(a / (-b)) = -ceil(-a / b) for b > 0... this gets confusing

    // Let's just compute it directly
    let quot = a / b;
    let rem = a % b;
    // If there's a remainder and the signs of a and b are different, we need to adjust
    // Actually for floor division with negative b:
    // If a and b have different signs and there's a remainder, subtract 1
    if rem != 0 && (a > 0) {
        quot - 1
    } else {
        quot
    }
}

fn part2(input: &str) -> i64 {
    let machines = parse(input);
    machines
        .iter()
        .map(|m| solve_joltage(&m.buttons, &m.joltage))
        .sum()
}

fn main() {
    let input = fs::read_to_string("data/10.txt").expect("Could not read input file");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn test_machine1() {
        let machines = parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        let m = &machines[0];
        assert_eq!(solve_machine(&m.lights, &m.buttons), 2);
    }

    #[test]
    fn test_machine2() {
        let machines = parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        let m = &machines[0];
        assert_eq!(solve_machine(&m.lights, &m.buttons), 3);
    }

    #[test]
    fn test_machine3() {
        let machines = parse("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        let m = &machines[0];
        assert_eq!(solve_machine(&m.lights, &m.buttons), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 33);
    }

    #[test]
    fn test_joltage1() {
        let machines = parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        let m = &machines[0];
        assert_eq!(solve_joltage(&m.buttons, &m.joltage), 10);
    }

    #[test]
    fn test_joltage2() {
        let machines = parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        let m = &machines[0];
        assert_eq!(solve_joltage(&m.buttons, &m.joltage), 12);
    }

    #[test]
    fn test_joltage3() {
        let machines = parse("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        let m = &machines[0];
        assert_eq!(solve_joltage(&m.buttons, &m.joltage), 11);
    }
}
