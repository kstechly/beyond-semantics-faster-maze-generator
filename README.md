# Beyond Semantics -- Faster Data Generation Utilities

See [here](https://github.com/kstechly/beyond-semantics-maze-visualizer) for background. This is just a faster reimplementation of the data generation pipeline (though it does not replicate bit-for-bit the same datasets as in [the paper](https://arxiv.org/abs/2505.13775).

Note that this implementation does not have a flag to distinguish between training and test data -- either split after generation or use two different seeds.

## Prerequisites
- Rust 1.70 or later

## Basic Usage
Generate 100000 mazes using DFS generator and A* solver:
```bash
./target/release/maze_gen_fast --generator dfs --solver astar --count 100000 --seed 12345
```
Generate 50000 50x50 mazes using Wilson's algorithm:
```bash
./target/release/maze_gen_fast --generator wilson --solver astar --count 50000 --rows 50 --cols 50 --seed 41
```

Use different generators:
```bash
# Kruskal's algorithm
./target/release/maze_gen_fast -g kruskal -s astar -c 1000000 --seed 37

# Searchformer style (random walls validated by pathfinding)
./target/release/maze_gen_fast -g searchformer -s astar -c 1000000 --seed 999

# Drunkard's walk with custom coverage
./target/release/maze_gen_fast -g drunkards-walk -s astar -c 1000000 --param coverage=0.7 --seed 666
```

Output to custom file:
```bash
./target/release/maze_gen_fast -g dfs -s astar -c 1000000 --output mazes.jsonl --seed 54321
```

List generator parameters:
```bash
# List parameters for all generators
./target/release/maze_gen_fast --list-params

# List parameters for a specific generator
./target/release/maze_gen_fast --list-params drunkards-walk
```

### Output Format
The tool outputs mazes in JSONL format (one JSON object per line). This should be compatible with the main pipeline, though it does contain additional fields. Each line contains:
- `idx`: Instance ID
- `text`: Maze representation with start/goal positions, walls, reasoning trace, and solution path
- `generator`: Algorithm used to generate the maze
- `solver`: Algorithm used to solve the maze
- `seed`: Random seed used
- `rows`: Maze height
- `cols`: Maze width