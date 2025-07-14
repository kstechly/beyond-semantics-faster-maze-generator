use crate::types::Maze;
use crate::solvers::astar;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_xoshiro::Xoshiro256PlusPlus;

/// SearchFormer Style maze generation
/// Randomly samples walls (30-50% density) then validates with A*
pub fn generate(
    rng: &mut Xoshiro256PlusPlus,
    rows: usize,
    cols: usize,
) -> Maze {
    let total = rows * cols;
    let base = total / 10;
    let min_walls = base * 3;  // 30% walls
    let max_walls = base * 5;  // 50% walls
    
    // Create indices [0..total-1]
    let mut indices: Vec<usize> = (0..total).collect();
    
    loop {  // Retry loop for entire maze generation
        // Shuffle indices
        indices.shuffle(rng);
        
        // Pick random number of walls between 30-50%
        let num_walls = rng.gen_range(min_walls..=max_walls);
        
        // First num_walls indices are walls, rest are passages
        let passages = &indices[num_walls..];
        
        // Create maze with all walls first
        let mut maze = Maze::new(rows, cols);
        
        // Carve passages
        for &idx in passages {
            let x = idx % cols;
            let y = idx / cols;
            maze.set_cell(x, y, true);
        }
        
        // Try up to 100 start/goal placements
        let mut free_cells = passages.to_vec();
        
        for _ in 0..100 {
            // Shuffle before picking to actually try different start/goal combinations
            free_cells.shuffle(rng);
            
            if free_cells.len() < 2 {
                break;  // Not enough free cells, retry maze
            }
            
            let start_idx = free_cells[0];
            let goal_idx = free_cells[1];
            
            let start_x = start_idx % cols;
            let start_y = start_idx / cols;
            let goal_x = goal_idx % cols;
            let goal_y = goal_idx / cols;
            
            // Set temporary start/goal
            maze.start = (start_x, start_y);
            maze.goal = (goal_x, goal_y);
            
            // Run A* to validate - we need the actual path length
            let solution = astar::solve(&maze);
            
            // Check if path exists and is long enough
            if !solution.path.is_empty() && 
               solution.path.len() >= rows.max(cols) {
                // Success! Return this maze
                return maze;
            }
        }
        
        // Failed to find valid start/goal after 100 attempts
        // Loop will retry with new random walls
    }
}