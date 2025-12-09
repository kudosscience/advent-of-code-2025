use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "input.txt" };
    let input = fs::read_to_string(filename).expect("Failed to read input file");

    let red_points: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    // Part 1: Find largest rectangle using any two red tiles as opposite corners
    let mut max_area_part1: i64 = 0;
    for i in 0..red_points.len() {
        for j in (i + 1)..red_points.len() {
            let (x1, y1) = red_points[i];
            let (x2, y2) = red_points[j];
            if x1 != x2 && y1 != y2 {
                let width = (x2 - x1).abs() + 1;
                let height = (y2 - y1).abs() + 1;
                let area = width * height;
                if area > max_area_part1 {
                    max_area_part1 = area;
                }
            }
        }
    }
    println!("Part 1 - Largest rectangle area: {}", max_area_part1);

    // Part 2: Build the polygon boundary segments
    // Store horizontal and vertical segments separately for efficient checking
    let mut h_segments: Vec<(i64, i64, i64)> = Vec::new(); // (y, x_min, x_max)
    let mut v_segments: Vec<(i64, i64, i64)> = Vec::new(); // (x, y_min, y_max)

    let red_set: HashSet<(i64, i64)> = red_points.iter().cloned().collect();

    for i in 0..red_points.len() {
        let (x1, y1) = red_points[i];
        let (x2, y2) = red_points[(i + 1) % red_points.len()];

        if x1 == x2 {
            // Vertical segment
            let (min_y, max_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            v_segments.push((x1, min_y, max_y));
        } else if y1 == y2 {
            // Horizontal segment
            let (min_x, max_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            h_segments.push((y1, min_x, max_x));
        }
    }

    // For a point to be inside or on boundary of the polygon, use ray casting
    // Count how many vertical segments are to the right of the point
    // If odd, point is inside

    let is_on_boundary = |x: i64, y: i64| -> bool {
        // Check horizontal segments
        for &(seg_y, x_min, x_max) in &h_segments {
            if y == seg_y && x >= x_min && x <= x_max {
                return true;
            }
        }
        // Check vertical segments
        for &(seg_x, y_min, y_max) in &v_segments {
            if x == seg_x && y >= y_min && y <= y_max {
                return true;
            }
        }
        false
    };

    let is_inside = |x: i64, y: i64| -> bool {
        // Ray casting: count crossings to the right
        let mut crossings = 0;
        for &(seg_x, y_min, y_max) in &v_segments {
            if seg_x > x && y > y_min && y <= y_max {
                crossings += 1;
            }
        }
        crossings % 2 == 1
    };

    let is_red_or_green = |x: i64, y: i64| -> bool {
        is_on_boundary(x, y) || is_inside(x, y)
    };

    // For a rectangle to be valid, all four corners and all edges must be inside
    // But actually we need ALL tiles inside to be red or green
    // For efficiency: check if the rectangle's boundary is fully within the polygon
    // and then use the property that if boundary is inside a simple polygon, interior is too

    // Actually, for a rectilinear polygon, we can check more efficiently:
    // The rectangle is valid if all its corners are inside/on boundary AND
    // no polygon edge crosses through the interior of the rectangle

    let rect_valid = |left: i64, right: i64, top: i64, bottom: i64| -> bool {
        // Check all four corners
        let corners = [(left, top), (right, top), (left, bottom), (right, bottom)];
        for &(cx, cy) in &corners {
            if !is_red_or_green(cx, cy) {
                return false;
            }
        }

        // Check if any vertical segment of the polygon crosses the rectangle interior
        // A vertical segment at x=seg_x from y_min to y_max crosses if:
        // - seg_x is strictly between left and right
        // - the segment overlaps with [top, bottom] range
        for &(seg_x, y_min, y_max) in &v_segments {
            if seg_x > left && seg_x < right {
                // Check if this segment crosses through the rectangle
                if y_min < bottom && y_max > top {
                    return false;
                }
            }
        }

        // Check if any horizontal segment of the polygon crosses the rectangle interior
        for &(seg_y, x_min, x_max) in &h_segments {
            if seg_y > top && seg_y < bottom {
                // Check if this segment crosses through the rectangle
                if x_min < right && x_max > left {
                    return false;
                }
            }
        }

        true
    };

    // Part 2: Find largest rectangle where corners are red and entire rectangle is red/green
    let mut max_area_part2: i64 = 0;

    for i in 0..red_points.len() {
        for j in (i + 1)..red_points.len() {
            let (x1, y1) = red_points[i];
            let (x2, y2) = red_points[j];

            if x1 == x2 || y1 == y2 {
                continue;
            }

            let left = x1.min(x2);
            let right = x1.max(x2);
            let top = y1.min(y2);
            let bottom = y1.max(y2);

            let width = right - left + 1;
            let height = bottom - top + 1;
            let area = width * height;

            // Skip if can't beat current best
            if area <= max_area_part2 {
                continue;
            }

            if rect_valid(left, right, top, bottom) {
                max_area_part2 = area;
            }
        }
    }

    println!("Part 2 - Largest rectangle area: {}", max_area_part2);
}

