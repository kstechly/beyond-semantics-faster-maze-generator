use crate::types::{Maze, Solution, ReasoningEvent};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A* node for priority queue
#[derive(Copy, Clone, Eq, PartialEq)]
struct AStarNode {
    x: u16,
    y: u16,
    g_score: u16,
    f_score: u16,
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap based on f_score
        other.f_score.cmp(&self.f_score)
            .then_with(|| other.g_score.cmp(&self.g_score))
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Manhattan distance heuristic
#[inline(always)]
fn manhattan_distance(x1: u16, y1: u16, x2: u16, y2: u16) -> u16 {
    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as u16
}

/// A* pathfinding with reasoning trace
pub fn solve(maze: &Maze) -> Solution {
    let mut reasoning: Vec<ReasoningEvent> = Vec::with_capacity(1000);
    let mut open_set = BinaryHeap::with_capacity(256);
    
    // Use flat arrays for better cache locality
    let total_cells = maze.rows * maze.cols;
    let mut g_scores = vec![u16::MAX; total_cells];
    let mut came_from = vec![u32::MAX; total_cells];
    let mut closed_set = vec![false; total_cells];
    
    // Convert coordinates to u16 for efficiency
    let start_x = maze.start.0 as u16;
    let start_y = maze.start.1 as u16;
    let goal_x = maze.goal.0 as u16;
    let goal_y = maze.goal.1 as u16;
    let _cols = maze.cols as u16;
    
    // Initialize start node
    let start_idx = (start_y as usize) * maze.cols + (start_x as usize);
    let start_h = manhattan_distance(start_x, start_y, goal_x, goal_y);
    g_scores[start_idx] = 0;
    open_set.push(AStarNode {
        x: start_x,
        y: start_y,
        g_score: 0,
        f_score: start_h,
    });
    
    const DIRECTIONS: [(i16, i16); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    
    while let Some(current_node) = open_set.pop() {
        let x = current_node.x;
        let y = current_node.y;
        let current_idx = (y as usize) * maze.cols + (x as usize);
        
        // Skip if already processed
        if closed_set[current_idx] {
            continue;
        }
        
        let g_score = current_node.g_score;
        let h_score = manhattan_distance(x, y, goal_x, goal_y);
        
        // Record close event
        reasoning.push(ReasoningEvent::Close { 
            x: x, 
            y: y, 
            g: g_score, 
            h: h_score 
        });
        
        // Found goal
        if x == goal_x && y == goal_y {
            break;
        }
        
        closed_set[current_idx] = true;
        
        // Explore neighbors
        for &(dx, dy) in &DIRECTIONS {
            let nx = x as i16 + dx;
            let ny = y as i16 + dy;
            
            // Check bounds
            if nx < 0 || nx >= maze.cols as i16 || ny < 0 || ny >= maze.rows as i16 {
                continue;
            }
            
            let nx = nx as u16;
            let ny = ny as u16;
            let neighbor_idx = (ny as usize) * maze.cols + (nx as usize);
            
            // Skip walls and closed nodes
            if !maze.get_cell(nx as usize, ny as usize) || closed_set[neighbor_idx] {
                continue;
            }
            
            let tentative_g = g_score + 1;
            
            // Update if this is a better path
            if tentative_g < g_scores[neighbor_idx] {
                came_from[neighbor_idx] = current_idx as u32;
                g_scores[neighbor_idx] = tentative_g;
                
                let h = manhattan_distance(nx, ny, goal_x, goal_y);
                let f = tentative_g + h;
                
                // Record create event
                reasoning.push(ReasoningEvent::Create { 
                    x: nx, 
                    y: ny, 
                    g: tentative_g, 
                    h 
                });
                
                open_set.push(AStarNode {
                    x: nx,
                    y: ny,
                    g_score: tentative_g,
                    f_score: f,
                });
            }
        }
    }
    
    // Reconstruct path
    let mut path = Vec::with_capacity(100);
    let goal_idx = (goal_y as usize) * maze.cols + (goal_x as usize);
    let mut current_idx = goal_idx;
    
    // Check if goal was reached
    if came_from[goal_idx] != u32::MAX || current_idx == start_idx {
        while current_idx != start_idx {
            let x = (current_idx % maze.cols) as usize;
            let y = (current_idx / maze.cols) as usize;
            path.push((x, y));
            
            let prev_idx = came_from[current_idx];
            if prev_idx == u32::MAX {
                // No path found
                path.clear();
                break;
            }
            current_idx = prev_idx as usize;
        }
        
        if !path.is_empty() {
            path.push(maze.start);
            path.reverse();
        }
    }
    
    Solution { path, reasoning }
}