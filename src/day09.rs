fn main() {
    let input = std::fs::read_to_string("data/09.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let tiles = parse(input);
    let mut max_area = 0;

    // Try all pairs of tiles as opposite corners
    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            // Calculate rectangle dimensions (inclusive of both corners)
            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;

            // Area of rectangle with these as opposite corners
            let area = width * height;
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn part2(input: &str) -> i64 {
    let tiles = parse(input);
    let n = tiles.len();

    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[j];

            let width = (x2 - x1).abs() + 1;
            let height = (y2 - y1).abs() + 1;
            let area = width * height;

            // Check if rectangle is valid (all inside/on polygon)
            if is_rect_valid(&tiles, x1, y1, x2, y2) {
                max_area = max_area.max(area);
            }
        }
    }

    max_area
}

// Check if a rectangle with opposite corners (x1,y1) and (x2,y2) is entirely within the polygon
fn is_rect_valid(polygon: &[(i64, i64)], x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);

    // Check all 4 corners
    let corners = [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ];

    for &corner in &corners {
        if !point_in_polygon(corner, polygon) {
            return false;
        }
    }

    // Check all 4 edges don't cross the polygon boundary improperly
    // For a rectilinear polygon and axis-aligned rectangle, if all 4 corners are inside,
    // and the edges don't cross polygon edges, then the rectangle is valid.

    // Check each edge of the rectangle against each edge of the polygon
    let rect_edges = [
        ((min_x, min_y), (max_x, min_y)), // bottom
        ((min_x, max_y), (max_x, max_y)), // top
        ((min_x, min_y), (min_x, max_y)), // left
        ((max_x, min_y), (max_x, max_y)), // right
    ];

    let n = polygon.len();
    for rect_edge in &rect_edges {
        for i in 0..n {
            let poly_edge = (polygon[i], polygon[(i + 1) % n]);
            if edges_cross_interior(rect_edge, &poly_edge) {
                return false;
            }
        }
    }

    true
}

// Check if point is inside or on the boundary of the polygon
fn point_in_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (px, py) = point;
    let n = polygon.len();

    // First check if point is on any edge
    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        if x1 == x2 {
            // Vertical edge
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            if px == x1 && py >= min_y && py <= max_y {
                return true;
            }
        } else {
            // Horizontal edge
            let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            if py == y1 && px >= min_x && px <= max_x {
                return true;
            }
        }
    }

    // Ray casting algorithm for interior check
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        // Check if ray from point going right crosses this edge
        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }

    inside
}

// Check if two axis-aligned segments cross in their interiors (perpendicular crossing only)
// Parallel/overlapping segments are NOT considered crossings
fn edges_cross_interior(e1: &((i64, i64), (i64, i64)), e2: &((i64, i64), (i64, i64))) -> bool {
    let ((x1a, y1a), (x1b, y1b)) = *e1;
    let ((x2a, y2a), (x2b, y2b)) = *e2;

    let e1_horiz = y1a == y1b;
    let e2_horiz = y2a == y2b;

    // Parallel segments don't "cross" - they either overlap (allowed) or don't touch
    if e1_horiz == e2_horiz {
        return false;
    }

    if e1_horiz {
        // e1 horizontal, e2 vertical
        // They cross in interior if e2's x is strictly between e1's x range
        // and e1's y is strictly between e2's y range
        let (min_x1, max_x1) = (x1a.min(x1b), x1a.max(x1b));
        let (min_y2, max_y2) = (y2a.min(y2b), y2a.max(y2b));
        x2a > min_x1 && x2a < max_x1 && y1a > min_y2 && y1a < max_y2
    } else {
        // e1 vertical, e2 horizontal
        let (min_y1, max_y1) = (y1a.min(y1b), y1a.max(y1b));
        let (min_x2, max_x2) = (x2a.min(x2b), x2a.max(x2b));
        x1a > min_x2 && x1a < max_x2 && y2a > min_y1 && y2a < max_y1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 24);
    }

    #[test]
    fn test_rect_valid() {
        let polygon: Vec<(i64, i64)> = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];

        // Rectangle from 7,3 to 11,1 has corners: (7,1), (7,3), (11,1), (11,3)
        // Check each corner:
        assert!(point_in_polygon((7, 1), &polygon), "corner (7,1) should be in");
        assert!(point_in_polygon((7, 3), &polygon), "corner (7,3) should be in");
        assert!(point_in_polygon((11, 1), &polygon), "corner (11,1) should be in");
        assert!(point_in_polygon((11, 3), &polygon), "corner (11,3) should be in");

        // Rectangle from 7,3 to 11,1 - check edges don't cross
        // Rectangle edges: bottom (7,1)-(11,1), top (7,3)-(11,3), left (7,1)-(7,3), right (11,1)-(11,3)
        // Polygon edges check
        let rect_edges = [
            ((7i64, 1i64), (11i64, 1i64)), // bottom
            ((7, 3), (11, 3)), // top
            ((7, 1), (7, 3)), // left
            ((11, 1), (11, 3)), // right
        ];
        for (idx, rect_edge) in rect_edges.iter().enumerate() {
            for (pidx, i) in (0..polygon.len()).enumerate() {
                let poly_edge = (polygon[i], polygon[(i + 1) % polygon.len()]);
                if edges_cross_interior(rect_edge, &poly_edge) {
                    println!("rect edge {} {:?} crosses poly edge {} {:?}", idx, rect_edge, pidx, poly_edge);
                }
            }
        }

        // Rectangle from 7,3 to 11,1 - should be valid (area 15)
        assert!(is_rect_valid(&polygon, 7, 3, 11, 1), "7,3 to 11,1 should be valid");

        // Rectangle from 9,5 to 2,3 - should be valid (area 24)
        assert!(is_rect_valid(&polygon, 9, 5, 2, 3), "9,5 to 2,3 should be valid");

        // Rectangle from 2,5 to 11,1 - should be INVALID (goes outside)
        assert!(!is_rect_valid(&polygon, 2, 5, 11, 1), "2,5 to 11,1 should be invalid");
    }

    #[test]
    fn test_point_in_polygon() {
        let polygon: Vec<(i64, i64)> = vec![
            (7, 1),
            (11, 1),
            (11, 7),
            (9, 7),
            (9, 5),
            (2, 5),
            (2, 3),
            (7, 3),
        ];
        // Red tiles should be in
        assert!(point_in_polygon((7, 1), &polygon), "7,1 should be in");
        assert!(point_in_polygon((11, 1), &polygon), "11,1 should be in");
        assert!(point_in_polygon((2, 5), &polygon), "2,5 should be in");

        // Green tiles on edges should be in
        assert!(point_in_polygon((8, 1), &polygon), "8,1 should be in (on edge)");
        assert!(point_in_polygon((9, 1), &polygon), "9,1 should be in (on edge)");

        // Interior green tiles should be in
        assert!(point_in_polygon((8, 2), &polygon), "8,2 should be in (interior)");
        assert!(point_in_polygon((5, 4), &polygon), "5,4 should be in (interior)");

        // Outside tiles should be out
        assert!(!point_in_polygon((1, 1), &polygon), "1,1 should be out");
        assert!(!point_in_polygon((5, 1), &polygon), "5,1 should be out");
    }
}
