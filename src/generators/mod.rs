pub mod wilson;
pub mod dfs;
pub mod kruskal;
pub mod drunkards_walk;
pub mod searchformer;

use crate::types::Maze;
use crate::GeneratorType;
use crate::parameters::GeneratorParams;
use rand_xoshiro::Xoshiro256PlusPlus;

pub fn generate_maze(
    generator: GeneratorType,
    rng: &mut Xoshiro256PlusPlus,
    rows: usize,
    cols: usize,
    params: &GeneratorParams,
) -> Maze {
    match generator {
        GeneratorType::Wilson => wilson::generate(rng, rows, cols),
        GeneratorType::Dfs => dfs::generate(rng, rows, cols),
        GeneratorType::Kruskal => kruskal::generate(rng, rows, cols),
        GeneratorType::DrunkardsWalk => drunkards_walk::generate(rng, rows, cols, params),
        GeneratorType::Searchformer => searchformer::generate(rng, rows, cols),
    }
}