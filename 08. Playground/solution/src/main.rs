use std::fs;

// --- Data Structures and Helper Functions ---

// A simple structure to hold 3D coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

// A structure to represent a connection (edge) between two junction boxes
struct Edge {
    u: usize,   // index of the first point
    v: usize,   // index of the second point
    dist2: u64, // squared distance
}

/// Calculates the squared Euclidean distance between two points: (dx^2 + dy^2 + dz^2).
/// Using squared distance is sufficient for sorting and avoids floating-point issues.
fn dist2(p1: Point, p2: Point) -> u64 {
    // Calculate the absolute difference for each coordinate.
    let dx = (p1.x as i64).abs_diff(p2.x as i64) as u128;
    let dy = (p1.y as i64).abs_diff(p2.y as i64) as u128;
    let dz = (p1.z as i64).abs_diff(p2.z as i64) as u128;

    // Sum of squares. The result fits comfortably within u64.
    (dx * dx + dy * dy + dz * dz) as u64
}

/// Disjoint Set Union (DSU) structure to manage circuits and track component size/count.
struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_circuits: usize,
}

impl Dsu {
    /// Creates a new DSU structure with `n` elements.
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_circuits: n, // Initially, n circuits
        }
    }

    /// Finds the representative (root) of the set containing element `i` with path compression.
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        // Path compression
        let root = self.find(self.parent[i]);
        self.parent[i] = root;
        root
    }

    /// Unites the sets containing elements `i` and `j` by size (rank).
    /// Returns `true` if a union successfully occurred, decreasing `num_circuits`.
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            // Union by size
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.num_circuits -= 1; // A successful merge reduces the count by one
            return true;
        }
        return false; // No merge: they were already in the same circuit
    }

    /// Returns a list of all distinct circuit sizes.
    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let mut sizes = Vec::new();
        // The size is only accurate and tracked at the root of each set.
        for i in 0..self.parent.len() {
            if self.parent[i] == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

/// Parses the input string into a vector of Point structs.
fn parse_input(input_content: &str) -> Vec<Point> {
    input_content
        .lines()
        .filter_map(|line| {
            let coords: Vec<&str> = line.trim().split(',').collect();
            if coords.len() == 3 {
                if let (Ok(x), Ok(y), Ok(z)) = (
                    coords[0].parse::<i64>(),
                    coords[1].parse::<i64>(),
                    coords[2].parse::<i64>(),
                ) {
                    Some(Point { x, y, z })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

// --- Part One Solution ---

/// Connects the 1000 shortest pairs of junction boxes and multiplies the sizes of the three largest resulting circuits.
pub fn part_one(points: &Vec<Point>) -> u64 {
    let n = points.len();
    if n < 3 {
        return 0;
    }

    // 1. Generate all possible edges (pairs) and calculate squared distance
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                u: i,
                v: j,
                dist2: dist2(points[i], points[j]),
            });
        }
    }

    // 2. Sort edges by distance (ascending)
    edges.sort_by_key(|e| e.dist2);

    // 3. Initialize DSU structure
    let mut dsu = Dsu::new(n);

    // 4. Process the 1000 shortest connections (edges)
    let num_connections_to_make = 1000;

    for i in 0..num_connections_to_make {
        if i >= edges.len() {
            break;
        }
        let edge = &edges[i];
        dsu.union(edge.u, edge.v);
    }

    // 5. Get the sizes of all resulting circuits and sort them
    let mut circuit_sizes = dsu.get_circuit_sizes();
    circuit_sizes.sort_by(|a, b| b.cmp(a)); // Descending order

    // 6. Multiply the sizes of the three largest circuits
    let s1 = *circuit_sizes.get(0).unwrap_or(&0) as u64;
    let s2 = *circuit_sizes.get(1).unwrap_or(&0) as u64;
    let s3 = *circuit_sizes.get(2).unwrap_or(&0) as u64;

    s1 * s2 * s3
}

// --- Part Two Solution ---

/// Continues connecting the closest pairs until all junction boxes are in a single circuit.
/// Returns the product of the X coordinates of the last two connected junction boxes.
pub fn part_two(points: &Vec<Point>) -> u64 {
    let n = points.len();
    if n <= 1 {
        return 0;
    }

    // 1. Generate all possible edges (pairs) and calculate squared distance
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push(Edge {
                u: i,
                v: j,
                dist2: dist2(points[i], points[j]),
            });
        }
    }

    // 2. Sort edges by distance (ascending)
    edges.sort_by_key(|e| e.dist2);

    // 3. Initialize DSU structure
    let mut dsu = Dsu::new(n);

    // 4. Iterate through sorted edges, performing unions until only one circuit remains
    for edge in &edges {
        let u = edge.u;
        let v = edge.v;

        // Attempt to merge the circuits
        let merged = dsu.union(u, v);

        // Check for the stopping condition: 1 circuit remains after a successful merge.
        if merged && dsu.num_circuits == 1 {
            // This is the LAST connection required.
            let p1 = points[u];
            let p2 = points[v];

            // Multiply the X coordinates of the two connected junction boxes
            return (p1.x as u64) * (p2.x as u64);
        }
    }

    // Should only be reached if n=0 or n=1
    0
}

// --- Main Function ---

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "../input.txt";
    let input_content = fs::read_to_string(file_path)?;
    let points = parse_input(&input_content);

    // Part One
    let result_one = part_one(&points);
    fs::write("../output_part_one.txt", result_one.to_string())?;

    // Part Two
    let result_two = part_two(&points);
    fs::write("../output_part_two.txt", result_two.to_string())?;

    Ok(())
}
