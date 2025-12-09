fn main() {
    let input = std::fs::read_to_string("data/08.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<(i64, i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .collect()
}

fn distance_squared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx * dx + dy * dy + dz * dz
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        if px == py {
            return;
        }
        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
            self.size[py] += self.size[px];
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
            self.size[px] += self.size[py];
        } else {
            self.parent[py] = px;
            self.size[px] += self.size[py];
            self.rank[px] += 1;
        }
    }
}

fn part1(input: &str) -> i64 {
    let boxes = parse(input);
    let n = boxes.len();

    // Compute all pairwise distances
    let mut distances: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_squared(&boxes[i], &boxes[j]);
            distances.push((dist, i, j));
        }
    }

    // Sort by distance
    distances.sort_by_key(|&(d, _, _)| d);

    // Make 1000 connections using Union-Find
    let mut uf = UnionFind::new(n);
    for &(_, i, j) in distances.iter().take(1000) {
        uf.union(i, j);
    }

    // Find circuit sizes
    let mut circuit_sizes: Vec<i64> = Vec::new();
    for i in 0..n {
        if uf.find(i) == i {
            circuit_sizes.push(uf.size[i] as i64);
        }
    }

    // Sort descending and multiply top 3
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
}

fn part2(input: &str) -> i64 {
    let boxes = parse(input);
    let n = boxes.len();

    // Compute all pairwise distances
    let mut distances: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_squared(&boxes[i], &boxes[j]);
            distances.push((dist, i, j));
        }
    }

    // Sort by distance
    distances.sort_by_key(|&(d, _, _)| d);

    // Keep connecting until all in one circuit
    let mut uf = UnionFind::new(n);
    let mut num_circuits = n;

    for &(_, i, j) in &distances {
        let pi = uf.find(i);
        let pj = uf.find(j);
        if pi != pj {
            uf.union(i, j);
            num_circuits -= 1;
            if num_circuits == 1 {
                // This was the last connection needed
                return boxes[i].0 * boxes[j].0;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1_example() {
        // After 10 connections: 5, 4, 2 => 5 * 4 * 2 = 40
        let boxes = parse(EXAMPLE);
        let n = boxes.len();

        let mut distances: Vec<(i64, usize, usize)> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                let dist = distance_squared(&boxes[i], &boxes[j]);
                distances.push((dist, i, j));
            }
        }
        distances.sort_by_key(|&(d, _, _)| d);

        let mut uf = UnionFind::new(n);
        for &(_, i, j) in distances.iter().take(10) {
            uf.union(i, j);
        }

        let mut circuit_sizes: Vec<i64> = Vec::new();
        for i in 0..n {
            if uf.find(i) == i {
                circuit_sizes.push(uf.size[i] as i64);
            }
        }
        circuit_sizes.sort_by(|a, b| b.cmp(a));

        assert_eq!(circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2], 40);
    }

    #[test]
    fn test_part2_example() {
        // Last connection is between 216,146,977 and 117,168,530
        // 216 * 117 = 25272
        assert_eq!(part2(EXAMPLE), 25272);
    }
}
