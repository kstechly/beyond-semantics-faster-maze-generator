use crate::types::Maze;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;

/// DFS (Depth-First Search / Recursive Backtracker) maze generation
pub fn generate(
    rng: &mut Xoshiro256PlusPlus,
    rows: usize,
    cols: usize,
) -> Maze {
    // Initialize maze with all walls
    let mut maze = Maze::new(rows, cols);
    
    // Parity offset for starting position (alternates between 0 and 1)
    let offset = if rng.gen::<f64>() < 0.5 { 0 } else { 1 };
    
    // Pick random starting cell at odd coordinates
    let start_x = offset + 2 * rng.gen_range(0..(cols - offset) / 2);
    let start_y = offset + 2 * rng.gen_range(0..(rows - offset) / 2);
    
    // Initialize stack for DFS with pre-allocation
    let mut stack = Vec::with_capacity(rows * cols / 4);
    stack.push((start_x, start_y));
    
    // Mark starting cell as floor
    maze.set_cell(start_x, start_y, true);
    
    // Directions for two-step jumps (to maintain wall structure)
    const DIRECTIONS: [(i32, i32); 4] = [(0, -2), (2, 0), (0, 2), (-2, 0)];
    
    // Pre-allocate neighbors vector
    let mut neighbors = Vec::with_capacity(4);
    
    // DFS loop
    while let Some((x, y)) = stack.last().copied() {
        // Find unvisited neighbors (2 cells away)
        neighbors.clear();
        
        for &(dx, dy) in &DIRECTIONS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            // Check bounds
            if nx >= 0 && nx < cols as i32 && ny >= 0 && ny < rows as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                
                // Check if unvisited (still a wall)
                if !maze.get_cell(nx, ny) {
                    neighbors.push((nx, ny));
                }
            }
        }
        
        if !neighbors.is_empty() {
            // Choose random unvisited neighbor
            let &(nx, ny) = &neighbors[rng.gen_range(0..neighbors.len())];
            
            // Carve path to neighbor
            maze.set_cell(nx, ny, true);
            
            // Carve connecting wall
            let wall_x = (x + nx) / 2;
            let wall_y = (y + ny) / 2;
            maze.set_cell(wall_x, wall_y, true);
            
            // Push neighbor onto stack
            stack.push((nx, ny));
        } else {
            // No unvisited neighbors, backtrack
            stack.pop();
        }
    }
    
    // Pick random distinct start and goal from floor cells
    let mut floors = Vec::with_capacity(rows * cols / 2);
    for y in 0..rows {
        for x in 0..cols {
            if maze.get_cell(x, y) {
                floors.push((x, y));
            }
        }
    }
    
    let start_idx = rng.gen_range(0..floors.len());
    let (start_x, start_y) = floors[start_idx];
    
    let mut goal_idx = rng.gen_range(0..floors.len());
    while goal_idx == start_idx {
        goal_idx = rng.gen_range(0..floors.len());
    }
    let (goal_x, goal_y) = floors[goal_idx];
    
    maze.start = (start_x, start_y);
    maze.goal = (goal_x, goal_y);
    maze
}