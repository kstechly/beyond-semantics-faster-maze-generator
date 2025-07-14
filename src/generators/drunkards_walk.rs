use crate::types::Maze;
use crate::parameters::GeneratorParams;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;

/// Drunkard's Walk maze generation
/// Creates organic cave-like structures by random walk
pub fn generate(
    rng: &mut Xoshiro256PlusPlus,
    rows: usize,
    cols: usize,
    params: &GeneratorParams,
) -> Maze {
    // Get coverage parameter with validation
    let coverage = params.get("coverage", 0.5).clamp(0.01, 1.0);
    
    // Calculate target cells to carve (at least 2 for start/goal)
    let total_cells = rows * cols;
    let target = ((total_cells as f64 * coverage) as usize).max(2);
    
    // Initialize maze with all walls
    let mut maze = Maze::new(rows, cols);
    
    // Start at random position
    let mut x = rng.gen_range(0..cols);
    let mut y = rng.gen_range(0..rows);
    
    // Carve starting position
    maze.set_cell(x, y, true);
    let mut carved = 1;
    
    // Direction constants for orthogonal movement
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    
    // Pre-allocate vector for valid directions
    let mut valid_dirs = Vec::with_capacity(4);
    
    // Random walk until target reached
    while carved < target {
        // Collect valid directions (staying in bounds)
        valid_dirs.clear();
        for &(dx, dy) in &DIRECTIONS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < cols as i32 && ny >= 0 && ny < rows as i32 {
                valid_dirs.push((dx, dy));
            }
        }
        
        // Move in random valid direction
        if valid_dirs.is_empty() {
            // This should never happen with proper bounds checking
            break;
        }
        
        let &(dx, dy) = &valid_dirs[rng.gen_range(0..valid_dirs.len())];
        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
        
        // Carve if unvisited
        if !maze.get_cell(x, y) {
            maze.set_cell(x, y, true);
            carved += 1;
        }
    }
    
    // Collect all floor cells for start/goal selection
    let mut floors = Vec::with_capacity(carved);
    for y in 0..rows {
        for x in 0..cols {
            if maze.get_cell(x, y) {
                floors.push((x, y));
            }
        }
    }
    
    // Edge case: ensure we have at least 2 cells
    if floors.len() < 2 {
        // This should be impossible with target >= 2, but be defensive
        // Carve one more cell if needed
        for y in 0..rows {
            for x in 0..cols {
                if !maze.get_cell(x, y) {
                    maze.set_cell(x, y, true);
                    floors.push((x, y));
                    if floors.len() >= 2 {
                        break;
                    }
                }
            }
            if floors.len() >= 2 {
                break;
            }
        }
    }
    
    // Pick random distinct start and goal
    let start_idx = rng.gen_range(0..floors.len());
    let (start_x, start_y) = floors[start_idx];
    
    let mut goal_idx = rng.gen_range(0..floors.len());
    while goal_idx == start_idx && floors.len() > 1 {
        goal_idx = rng.gen_range(0..floors.len());
    }
    let (goal_x, goal_y) = floors[goal_idx];
    
    maze.start = (start_x, start_y);
    maze.goal = (goal_x, goal_y);
    maze
}