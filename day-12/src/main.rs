use std::collections::HashSet;
use std::fs;

type Shape = Vec<(i32, i32)>;

/// Parse a shape from lines of # and .
fn parse_shape(lines: &[&str]) -> Shape {
    let mut shape = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                shape.push((x as i32, y as i32));
            }
        }
    }
    // Normalize shape to start at (0, 0)
    normalize_shape(&mut shape);
    shape
}

/// Normalize a shape so its minimum x and y are 0
fn normalize_shape(shape: &mut Shape) {
    if shape.is_empty() {
        return;
    }
    let min_x = shape.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = shape.iter().map(|(_, y)| *y).min().unwrap();
    for (x, y) in shape.iter_mut() {
        *x -= min_x;
        *y -= min_y;
    }
    shape.sort();
}

/// Generate all rotations and reflections of a shape
fn get_all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut orientations = HashSet::new();
    let mut current = shape.clone();
    
    // 4 rotations
    for _ in 0..4 {
        let mut normalized = current.clone();
        normalize_shape(&mut normalized);
        orientations.insert(normalized);
        
        // Also add horizontal flip
        let mut flipped: Shape = current.iter().map(|(x, y)| (-*x, *y)).collect();
        normalize_shape(&mut flipped);
        orientations.insert(flipped);
        
        // Rotate 90 degrees clockwise: (x, y) -> (y, -x)
        current = current.iter().map(|(x, y)| (*y, -*x)).collect();
    }
    
    orientations.into_iter().collect()
}

/// Check if a shape can be placed at position (px, py) on the grid
fn can_place(grid: &[Vec<bool>], shape: &Shape, px: i32, py: i32, width: usize, height: usize) -> bool {
    for &(sx, sy) in shape {
        let x = px + sx;
        let y = py + sy;
        if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
            return false;
        }
        if grid[y as usize][x as usize] {
            return false;
        }
    }
    true
}

/// Place a shape on the grid
fn place_shape(grid: &mut [Vec<bool>], shape: &Shape, px: i32, py: i32) {
    for &(sx, sy) in shape {
        let x = (px + sx) as usize;
        let y = (py + sy) as usize;
        grid[y][x] = true;
    }
}

/// Remove a shape from the grid
fn remove_shape(grid: &mut [Vec<bool>], shape: &Shape, px: i32, py: i32) {
    for &(sx, sy) in shape {
        let x = (px + sx) as usize;
        let y = (py + sy) as usize;
        grid[y][x] = false;
    }
}

/// Try to solve the puzzle using backtracking
/// Simple approach: just try all positions for each piece
fn solve(
    grid: &mut Vec<Vec<bool>>,
    pieces: &mut Vec<Vec<Shape>>, // Each element is the list of orientations for that piece
    width: usize,
    height: usize,
) -> bool {
    if pieces.is_empty() {
        return true; // All pieces placed
    }
    
    // Take the first remaining piece
    let orientations = pieces.remove(0);
    
    // Try each orientation
    for orientation in &orientations {
        // Calculate bounds for placement
        let max_x = orientation.iter().map(|(x, _)| *x).max().unwrap();
        let max_y = orientation.iter().map(|(_, y)| *y).max().unwrap();
        
        // Try all valid positions
        for py in 0..=(height as i32 - max_y - 1) {
            for px in 0..=(width as i32 - max_x - 1) {
                if can_place(grid, orientation, px, py, width, height) {
                    place_shape(grid, orientation, px, py);
                    
                    if solve(grid, pieces, width, height) {
                        return true;
                    }
                    
                    remove_shape(grid, orientation, px, py);
                }
            }
        }
    }
    
    // Put the piece back
    pieces.insert(0, orientations);
    false
}

/// Check if a region can fit all the required pieces
fn can_fit_region(shapes: &[Shape], width: usize, height: usize, counts: &[usize]) -> bool {
    // Build list of pieces to place (with all their orientations)
    let mut pieces: Vec<Vec<Shape>> = Vec::new();
    
    for (shape_idx, &count) in counts.iter().enumerate() {
        if shape_idx >= shapes.len() || count == 0 {
            continue;
        }
        let orientations = get_all_orientations(&shapes[shape_idx]);
        for _ in 0..count {
            pieces.push(orientations.clone());
        }
    }
    
    if pieces.is_empty() {
        return true;
    }
    
    // Calculate total cells needed
    let total_cells_needed: usize = counts
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            if i < shapes.len() {
                c * shapes[i].len()
            } else {
                0
            }
        })
        .sum();
    
    if total_cells_needed > width * height {
        return false;
    }
    
    // Sort pieces by size (larger first) for better pruning
    pieces.sort_by(|a, b| {
        let size_a = a[0].len();
        let size_b = b[0].len();
        size_b.cmp(&size_a)
    });
    
    let mut grid = vec![vec![false; width]; height];
    solve(&mut grid, &mut pieces, width, height)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "input.txt" };
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    
    // Parse input - handle the specific format
    let mut shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    
    let mut current_shape_lines: Vec<&str> = Vec::new();
    let mut parsing_shapes = true;
    
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        
        // Check if this is a region line (starts with dimensions like "12x5:")
        if line.contains('x') && line.contains(':') {
            let first_part = line.split(':').next().unwrap().trim();
            if first_part.contains('x') {
                let dims: Vec<&str> = first_part.split('x').collect();
                if dims.len() == 2 && dims[0].chars().all(|c| c.is_ascii_digit()) && dims[1].chars().all(|c| c.is_ascii_digit()) {
                    // This is a region line
                    if !current_shape_lines.is_empty() {
                        shapes.push(parse_shape(&current_shape_lines));
                        current_shape_lines.clear();
                    }
                    parsing_shapes = false;
                    
                    let counts_part = line.split(':').nth(1).unwrap();
                    let width: usize = dims[0].parse().unwrap();
                    let height: usize = dims[1].parse().unwrap();
                    let counts: Vec<usize> = counts_part
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    regions.push((width, height, counts));
                    continue;
                }
            }
        }
        
        if parsing_shapes {
            // Check if this is a shape header (like "0:")
            if line.ends_with(':') && line[..line.len()-1].chars().all(|c| c.is_ascii_digit()) {
                if !current_shape_lines.is_empty() {
                    shapes.push(parse_shape(&current_shape_lines));
                    current_shape_lines.clear();
                }
            } else if line.chars().all(|c| c == '#' || c == '.') {
                current_shape_lines.push(line);
            }
        }
    }
    
    // Don't forget the last shape
    if !current_shape_lines.is_empty() {
        shapes.push(parse_shape(&current_shape_lines));
    }
    
    println!("Parsed {} shapes and {} regions", shapes.len(), regions.len());
    
    // Count how many regions can fit all their presents
    let mut count = 0;
    for (i, (width, height, counts)) in regions.iter().enumerate() {
        if can_fit_region(&shapes, *width, *height, counts) {
            count += 1;
        }
        if (i + 1) % 100 == 0 {
            eprintln!("Processed {} regions...", i + 1);
        }
    }
    
    println!("Answer: {}", count);
}
