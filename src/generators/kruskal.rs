use crate::types::Maze;
use rand::seq::SliceRandom;
use rand::Rng;
use rand_xoshiro::Xoshiro256PlusPlus;

/// Efficient Union-Find structure with path compression and union-by-rank
struct UnionFind {
    parent: Vec<u16>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size as u16).collect(),
            rank: vec![0; size],
        }
    }
    
    #[inline(always)]
    fn find(&mut self, x: usize) -> u16 {
        let x_u16 = x as u16;
        if self.parent[x] != x_u16 {
            self.parent[x] = self.find(self.parent[x] as usize);
        }
        self.parent[x]
    }
    
    /// Returns true if the sets were successfully unified (were in different sets)
    #[inline(always)]
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x) as usize;
        let root_y = self.find(y) as usize;
        
        if root_x == root_y {
            return false;
        }
        
        // Union by rank
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => {
                self.parent[root_x] = root_y as u16;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_y] = root_x as u16;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x as u16;
                self.rank[root_x] += 1;
            }
        }
        
        true
    }
}

/// Edge between two rooms with packed wall position
#[derive(Clone, Copy)]
struct Edge {
    room1: u16,
    room2: u16,
    wall_pos: u32,  // High 16 bits = y, Low 16 bits = x
}

/// Pack wall coordinates into a single u32
#[inline(always)]
fn pack_wall_pos(x: usize, y: usize) -> u32 {
    ((y as u32) << 16) | (x as u32)
}

/// Unpack wall coordinates from u32
#[inline(always)]
fn unpack_wall_pos(packed: u32) -> (usize, usize) {
    ((packed & 0xFFFF) as usize, (packed >> 16) as usize)
}

/// Kruskal's algorithm for maze generation
pub fn generate(
    rng: &mut Xoshiro256PlusPlus,
    rows: usize,
    cols: usize,
) -> Maze {
    // Initialize maze with all walls
    let mut maze = Maze::new(rows, cols);
    
    // Random parity offset (0 or 1)
    let offset = if rng.gen::<f64>() < 0.5 { 0 } else { 1 };
    
    // Pre-calculate room dimensions
    let room_rows = (rows - offset + 1) / 2;
    let room_cols = (cols - offset + 1) / 2;
    let num_rooms = room_rows * room_cols;
    
    // Reserve capacity for edges (each room can have at most 2 edges: right and down)
    let mut edges = Vec::with_capacity(num_rooms * 2);
    
    // Build rooms and edges in a single pass
    let mut room_id = 0u16;
    for room_y in 0..room_rows {
        for room_x in 0..room_cols {
            let x = offset + room_x * 2;
            let y = offset + room_y * 2;
            
            // Mark room as floor
            maze.set_cell(x, y, true);
            
            // Check right neighbor
            if room_x + 1 < room_cols {
                let neighbor_id = room_id + 1;
                let wall_x = x + 1;
                let wall_y = y;
                edges.push(Edge {
                    room1: room_id,
                    room2: neighbor_id,
                    wall_pos: pack_wall_pos(wall_x, wall_y),
                });
            }
            
            // Check down neighbor
            if room_y + 1 < room_rows {
                let neighbor_id = room_id + room_cols as u16;
                let wall_x = x;
                let wall_y = y + 1;
                edges.push(Edge {
                    room1: room_id,
                    room2: neighbor_id,
                    wall_pos: pack_wall_pos(wall_x, wall_y),
                });
            }
            
            room_id += 1;
        }
    }
    
    // Shuffle edges using Fisher-Yates
    edges.shuffle(rng);
    
    // Initialize Union-Find
    let mut uf = UnionFind::new(num_rooms);
    
    // Process edges and carve walls
    for edge in edges {
        if uf.union(edge.room1 as usize, edge.room2 as usize) {
            let (wall_x, wall_y) = unpack_wall_pos(edge.wall_pos);
            maze.set_cell(wall_x, wall_y, true);
        }
    }
    
    // Pick random distinct start and goal from floor cells
    // Reuse the same pattern as Wilson's and DFS for consistency
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