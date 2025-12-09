use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 { &args[1] } else { "input.txt" };
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    
    let points: Vec<(i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();
    
    let mut max_area: i64 = 0;
    
    // Check all pairs of points as opposite corners of a rectangle
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            
            // Calculate the area of the rectangle with these two points as opposite corners
            // The points must not be on the same row or column to form a valid rectangle
            // Area includes both corner tiles, so we add 1 to each dimension
            if x1 != x2 && y1 != y2 {
                let width = (x2 - x1).abs() + 1;
                let height = (y2 - y1).abs() + 1;
                let area = width * height;
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }
    
    println!("Largest rectangle area: {}", max_area);
}
