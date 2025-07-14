use crate::types::Maze;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;
use std::collections::{HashMap, HashSet};

/// Wilson's algorithm for maze generation (loop-erased random walk)
pub fn generate(
    rng: &mut Xoshiro256PlusPlus,
    rows: usize,
    cols: usize,
) -> Maze {
    // Initialize maze with all walls
    let mut maze = Maze::new(rows, cols);
    
    // Parity offset for rooms (alternates between 0 and 1)
    let offset = if rng.gen::<f64>() < 0.5 { 0 } else { 1 };
    
    // Build list of room coordinates (cells at odd positions)
    let mut rooms = Vec::new();
    for y in (offset..rows).step_by(2) {
        for x in (offset..cols).step_by(2) {
            rooms.push((x, y));
        }
    }
    
    // Seed maze with one random room
    let mut in_maze = HashSet::new();
    let first_idx = rng.gen_range(0..rooms.len());
    let (sx0, sy0) = rooms[first_idx];
    maze.set_cell(sx0, sy0, true);
    in_maze.insert((sx0, sy0));
    
    // Directions for two-step jumps
    let dirs = [(2i32, 0i32), (-2, 0), (0, 2), (0, -2)];
    
    // Loop-erased random walks to carve all rooms
    while in_maze.len() < rooms.len() {
        // Pick a random start outside the maze
        let mut root;
        loop {
            let idx = rng.gen_range(0..rooms.len());
            root = rooms[idx];
            if !in_maze.contains(&root) {
                break;
            }
        }
        
        // Perform loop-erased walk
        let mut path = vec![root];
        let mut index_map: HashMap<(usize, usize), usize> = HashMap::new();
        index_map.insert(root, 0);
        
        loop {
            let (cx, cy) = path[path.len() - 1];
            let (dx, dy) = dirs[rng.gen_range(0..dirs.len())];
            
            let nx = cx as i32 + dx;
            let ny = cy as i32 + dy;
            
            // Check bounds
            if nx < 0 || nx >= cols as i32 || ny < 0 || ny >= rows as i32 {
                continue;
            }
            
            let nx = nx as usize;
            let ny = ny as usize;
            let next = (nx, ny);
            
            // Check if we hit the existing maze
            if in_maze.contains(&next) {
                path.push(next);
                break;
            }
            
            // Check for loops
            if let Some(&idx) = index_map.get(&next) {
                // Erase loop: remove entries after the first occurrence
                path.truncate(idx + 1);
                
                // Rebuild index map for the trimmed path
                index_map.clear();
                for (j, &pos) in path.iter().enumerate() {
                    index_map.insert(pos, j);
                }
            } else {
                // Extend path
                index_map.insert(next, path.len());
                path.push(next);
            }
        }
        
        // Carve the path into the maze
        for i in 0..path.len() {
            let (cx, cy) = path[i];
            if !in_maze.contains(&(cx, cy)) {
                in_maze.insert((cx, cy));
                maze.set_cell(cx, cy, true);
            }
            
            // Carve the wall between consecutive path cells
            if i > 0 {
                let (px, py) = path[i - 1];
                let wx = (px + cx) / 2;
                let wy = (py + cy) / 2;
                maze.set_cell(wx, wy, true);
            }
        }
    }
    
    // Pick random distinct start and goal from floor cells
    let mut floors = Vec::new();
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