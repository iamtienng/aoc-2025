use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

type Point = (i64, i64);

fn parse_pair(s: &str) -> Point {
    let mut it = s.trim().split(',');
    let a = it
        .next()
        .expect("missing x")
        .trim()
        .parse::<i64>()
        .expect("bad x");
    let b = it
        .next()
        .expect("missing y")
        .trim()
        .parse::<i64>()
        .expect("bad y");
    (a, b)
}

// --- Part One Solution ---

pub fn part_one(pts: &[Point]) -> u128 {
    let n = pts.len();
    let mut best: u128 = 0;

    // Brute-force check all pairs of red tiles
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = pts[i];
            let (x2, y2) = pts[j];

            // Calculate dimensions (inclusive)
            let dx = (x1 - x2).abs();
            let dy = (y1 - y2).abs();

            let w = (dx as u128) + 1;
            let h = (dy as u128) + 1;

            let area = w.saturating_mul(h);
            if area > best {
                best = area;
            }
        }
    }
    best
}

// --- Part Two Helper Functions ---

// Ray Casting Algorithm (Parity) to check if a point is strictly inside the polygon.
// Casts a ray to x=+infinity and counts vertical segment crossings.
fn is_inside_polygon(test_pt: Point, pts: &[Point]) -> bool {
    let (x_test, y_test) = test_pt;
    let n = pts.len();
    let mut intersections = 0;

    for i in 0..n {
        let p1 = pts[i];
        let p2 = pts[(i + 1) % n];

        // Only consider vertical segments (p1.0 == p2.0) that are to the right of x_test
        if p1.0 == p2.0 && p1.0 > x_test {
            let y_min = min(p1.1, p2.1);
            let y_max = max(p1.1, p2.1);

            // If the segment crosses the horizontal ray y = y_test.
            // Using strict inequality for the start point and inclusive for the end point
            // correctly handles vertices without double counting.
            if y_test > y_min && y_test <= y_max {
                intersections += 1;
            }
        }
    }

    // If the number of intersections is odd, the point is inside.
    intersections % 2 != 0
}

// Collects all boundary points (Red and Green) into a HashSet for O(1) lookup.
fn get_boundary_points(red_pts: &[Point]) -> HashSet<Point> {
    let mut boundary_pts: HashSet<Point> = HashSet::new();
    let n = red_pts.len();

    for i in 0..n {
        let p1 = red_pts[i];
        let p2 = red_pts[(i + 1) % n];

        let mut curr_x = p1.0;
        let mut curr_y = p1.1;

        // Determine step direction
        let dx_step = if p1.0 < p2.0 {
            1
        } else if p1.0 > p2.0 {
            -1
        } else {
            0
        };
        let dy_step = if p1.1 < p2.1 {
            1
        } else if p1.1 > p2.1 {
            -1
        } else {
            0
        };

        // Iterate along the segment, including the start point
        loop {
            boundary_pts.insert((curr_x, curr_y));
            if curr_x == p2.0 && curr_y == p2.1 {
                break;
            }
            curr_x += dx_step;
            curr_y += dy_step;
        }
    }
    boundary_pts
}

// Checks if the interior of rectangle R defined by c1 and c2 is entirely allowed (Red or Green).
fn is_rectangle_valid(
    c1: Point,
    c2: Point,
    red_pts: &[Point],
    boundary_pts: &HashSet<Point>,
) -> bool {
    let (x1, y1) = c1;
    let (x2, y2) = c2;

    let x_min = min(x1, x2);
    let x_max = max(x1, x2);
    let y_min = min(y1, y2);
    let y_max = max(y1, y2);

    // Check 1: Non-Red Corners must be Allowed (Boundary OR Interior)
    let c3 = (x1, y2);
    let c4 = (x2, y1);

    // An Allowed tile is either on the boundary (boundary_pts) or strictly inside the polygon.
    let is_allowed = |p: Point| boundary_pts.contains(&p) || is_inside_polygon(p, red_pts);

    if !is_allowed(c3) || !is_allowed(c4) {
        return false;
    }

    // Edge case: 1xN or Nx1 rectangles. If the corners are valid, the whole rectangle is valid.
    if x_min == x_max || y_min == y_max {
        return true;
    }

    // Check 2: Interior Containment
    // Use an interior tile (x_min + 1, y_min + 1) for the ray casting check.
    let x_test = x_min + 1;
    let y_test = y_min + 1;

    if !is_inside_polygon((x_test, y_test), red_pts) {
        // If an interior point is outside, the rectangle is invalid.
        return false;
    }

    // Check 3: Boundary Crossing
    // Ensure no polygon segment strictly intersects the open interior of the rectangle.
    for i in 0..red_pts.len() {
        let p1 = red_pts[i];
        let p2 = red_pts[(i + 1) % red_pts.len()];

        let px_min = min(p1.0, p2.0);
        let px_max = max(p1.0, p2.0);
        let py_min = min(p1.1, p2.1);
        let py_max = max(p1.1, p2.1);

        // Horizontal Segment (y is constant)
        if p1.1 == p2.1 {
            let y_seg = p1.1;

            // Check if segment is strictly between the rectangle's y-rows (y_min < y_seg < y_max)
            if y_seg > y_min && y_seg < y_max {
                // Check for overlap in the x-range (must be strictly inside the rectangle's x-columns)
                let overlap_x_min = max(px_min, x_min + 1);
                let overlap_x_max = min(px_max, x_max - 1);

                if overlap_x_min <= overlap_x_max {
                    return false;
                }
            }
        }
        // Vertical Segment (x is constant)
        else if p1.0 == p2.0 {
            let x_seg = p1.0;

            // Check if segment is strictly between the rectangle's x-columns (x_min < x_seg < x_max)
            if x_seg > x_min && x_seg < x_max {
                // Check for overlap in the y-range (must be strictly inside the rectangle's y-rows)
                let overlap_y_min = max(py_min, y_min + 1);
                let overlap_y_max = min(py_max, y_max - 1);

                if overlap_y_min <= overlap_y_max {
                    return false;
                }
            }
        }
    }

    true
}

// --- Part Two Solution ---

pub fn part_two(red_pts: &[Point]) -> u128 {
    let n = red_pts.len();
    if n < 2 {
        return 1;
    }

    // 1. Pre-calculate the entire set of boundary points
    let boundary_pts = get_boundary_points(red_pts);

    // 2. Iterate and Check for Largest Valid Rectangle
    let mut best_area: u128 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let c1 = red_pts[i];
            let c2 = red_pts[j];

            if is_rectangle_valid(c1, c2, red_pts, &boundary_pts) {
                // Rectangle is valid. Calculate area.
                let w = (c1.0 - c2.0).abs() as u128 + 1;
                let h = (c1.1 - c2.1).abs() as u128 + 1;
                let area = w.saturating_mul(h);

                if area > best_area {
                    best_area = area;
                }
            }
        }
    }
    best_area
}

fn main() -> io::Result<()> {
    // Read input data
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);
    let mut pts: Vec<Point> = Vec::new();
    for line_res in reader.lines() {
        let line = line_res?;
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        pts.push(parse_pair(s));
    }

    // Solve Part One
    let best_one = part_one(&pts);
    let mut out_one = File::create("../output_part_one.txt")?;
    writeln!(out_one, "{}", best_one)?;

    // Solve Part Two
    let best_two = part_two(&pts);
    let mut out_two = File::create("../output_part_two.txt")?;
    writeln!(out_two, "{}", best_two)?;

    Ok(())
}
