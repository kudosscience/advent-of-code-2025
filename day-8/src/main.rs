use std::fs;

/// Represents a 3D point (junction box position)
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn from_line(line: &str) -> Option<Point> {
        let parts: Vec<&str> = line.trim().split(',').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Point {
            x: parts[0].parse().ok()?,
            y: parts[1].parse().ok()?,
            z: parts[2].parse().ok()?,
        })
    }

    /// Calculate squared Euclidean distance to avoid floating point
    fn distance_squared(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

/// Union-Find (Disjoint Set Union) data structure for tracking circuits
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
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    /// Union two sets. Returns true if they were in different sets.
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in the same circuit
        }

        // Union by rank
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
            self.rank[root_x] += 1;
        }
        true
    }

    /// Get sizes of all unique circuits
    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = Vec::new();
        for i in 0..n {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

/// Represents a pair of junction boxes with their distance
#[derive(Debug)]
struct Pair {
    i: usize,
    j: usize,
    distance_sq: i64,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    // Parse all junction box positions
    let points: Vec<Point> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| Point::from_line(line))
        .collect();

    let n = points.len();
    println!("Number of junction boxes: {}", n);

    // Calculate all pairwise distances
    let mut pairs: Vec<Pair> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            pairs.push(Pair {
                i,
                j,
                distance_sq: points[i].distance_squared(&points[j]),
            });
        }
    }

    // Sort pairs by distance (ascending)
    pairs.sort_by_key(|p| p.distance_sq);

    // Initialize Union-Find structure (moved to parts below)

    // Part 1: Connect the 1000 closest pairs
    const CONNECTIONS_TO_MAKE: usize = 1000;
    let mut uf_part1 = UnionFind::new(n);
    
    for pair in pairs.iter().take(CONNECTIONS_TO_MAKE) {
        uf_part1.union(pair.i, pair.j);
    }

    // Get all circuit sizes for Part 1
    let mut circuit_sizes = uf_part1.get_circuit_sizes();
    circuit_sizes.sort_by(|a, b| b.cmp(a));

    println!("=== Part 1 ===");
    println!("Number of circuits: {}", circuit_sizes.len());
    println!("Top circuit sizes: {:?}", &circuit_sizes[..circuit_sizes.len().min(10)]);

    let result_part1: usize = circuit_sizes.iter().take(3).product();
    println!("Product of three largest circuit sizes: {}", result_part1);

    // Part 2: Continue connecting until all junction boxes are in one circuit
    // We need to find the connection that reduces circuit count to 1
    println!("\n=== Part 2 ===");
    
    let mut uf = UnionFind::new(n);
    let mut num_circuits = n; // Start with n individual circuits
    let mut last_connecting_pair: Option<&Pair> = None;

    for pair in pairs.iter() {
        // Only count actual merges (when two different circuits are connected)
        if uf.union(pair.i, pair.j) {
            num_circuits -= 1;
            
            if num_circuits == 1 {
                last_connecting_pair = Some(pair);
                break;
            }
        }
    }

    if let Some(pair) = last_connecting_pair {
        let p1 = &points[pair.i];
        let p2 = &points[pair.j];
        println!(
            "Last connection: ({},{},{}) and ({},{},{})",
            p1.x, p1.y, p1.z, p2.x, p2.y, p2.z
        );
        let result_part2 = p1.x * p2.x;
        println!("Product of X coordinates: {}", result_part2);
    } else {
        println!("Could not find a connection that unifies all circuits!");
    }
}
