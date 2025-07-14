pub mod astar;

use crate::types::{Maze, Solution};
use crate::SolverType;

pub fn solve_maze(solver: SolverType, maze: &Maze) -> Solution {
    match solver {
        SolverType::AStar => astar::solve(maze),
    }
}