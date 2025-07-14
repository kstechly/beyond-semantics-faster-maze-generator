mod generators;
mod parameters;
mod prng;
mod serializer;
mod solvers;
mod types;

use clap::{Parser, ValueEnum};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::mpsc::sync_channel;
use std::thread;
use std::time::Instant;

use crate::parameters::{GeneratorParams, print_param_help, print_all_params_help};
use crate::prng::create_instance_prng;
use crate::serializer::process_batch;
use crate::types::MazeResult;

#[derive(Clone, Copy, Debug, ValueEnum, Hash)]
pub enum GeneratorType {
    Dfs,
    Kruskal,
    Wilson,
    Searchformer,
    DrunkardsWalk,
}

#[derive(Clone, Copy, Debug, ValueEnum, Hash)]
pub enum SolverType {
    #[value(name = "astar")]
    AStar,
}

#[derive(Parser)]
#[command(name = "maze_gen_fast")]
#[command(about = "Parallel maze generation with hierarchical PRNG")]
struct Args {
    /// Generator algorithm
    #[arg(short, long, value_enum, required_unless_present = "list_params")]
    generator: Option<GeneratorType>,
    
    /// Solver algorithm
    #[arg(short, long, value_enum, required_unless_present = "list_params")]
    solver: Option<SolverType>,
    
    /// Master seed for PRNG
    #[arg(long, default_value = "42")]
    seed: u64,
    
    /// Number of mazes to generate
    #[arg(short, long, required_unless_present = "list_params")]
    count: Option<u64>,
    
    /// Maze height
    #[arg(long, default_value = "30")]
    rows: usize,
    
    /// Maze width
    #[arg(long, default_value = "30")]
    cols: usize,
    
    /// Output file
    #[arg(short, long, default_value = "output.jsonl")]
    output: String,
    
    /// Number of threads (defaults to all cores)
    #[arg(short, long)]
    threads: Option<usize>,
    
    /// Generator parameters as key=value pairs
    #[arg(long = "param", value_parser = parse_key_val::<String, String>)]
    params: Vec<(String, String)>,
    
    /// List parameters for a specific generator or all generators
    #[arg(long, value_name = "GENERATOR")]
    list_params: Option<Option<GeneratorType>>,
}

/// Parse key=value pairs
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Handle --list-params
    if let Some(maybe_generator) = args.list_params {
        match maybe_generator {
            Some(generator) => print_param_help(generator),
            None => print_all_params_help(),
        }
        return Ok(());
    }
    
    // Extract required args (safe because of required_unless_present)
    let generator = args.generator.expect("generator required");
    let solver = args.solver.expect("solver required");
    let count = args.count.expect("count required");
    
    // Parse generator parameters
    let generator_params = GeneratorParams::from_vec(args.params)?;
    
    // Set thread pool size if specified
    if let Some(threads) = args.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()?;
    }
    
    // Start timing
    let start_time = Instant::now();
    println!("Generating {} mazes...", count);
    
    // Create progress bar for writing only
    let writing_progress = ProgressBar::new(count);
    writing_progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.green/red} {pos}/{len} mazes written ({per_sec})")?
            .progress_chars("##-"),
    );
    
    // Create bounded channel with larger capacity for batches
    let (tx, rx) = sync_channel::<Vec<u8>>(100);
    
    
    // Batch size for processing
    const BATCH_SIZE: usize = 1000;
    
    // Writer thread
    let output_path = args.output.clone();
    let writer_handle = thread::spawn(move || -> Result<(), std::io::Error> {
        let file = File::create(&output_path)?;
        let mut writer = BufWriter::with_capacity(8 * 1024 * 1024, file);
        
        let mut total_written = 0u64;
        for batch_bytes in rx {
            writer.write_all(&batch_bytes)?;
            total_written += BATCH_SIZE as u64;
            
            // Update progress less frequently
            if total_written % 10_000 == 0 {
                writing_progress.set_position(total_written.min(writing_progress.length().unwrap_or(total_written)));
            }
            
            // Periodic flush
            if total_written % 10_000 == 0 {
                writer.flush()?;
            }
        }
        writer.flush()?;
        writing_progress.finish_with_message("All mazes written!");
        Ok(())
    });
    
    
    // Parallel generation
    let seed = args.seed;
    let rows = args.rows;
    let cols = args.cols;
    
    
    // Process mazes in batches
    (0..count)
        .step_by(BATCH_SIZE)
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each_with(tx, |tx, batch_start| {
            let batch_end = (batch_start + BATCH_SIZE as u64).min(count);
            let mut batch_results: Vec<MazeResult> = Vec::with_capacity(BATCH_SIZE);
            
            for instance_id in batch_start..batch_end {
                // Create instance PRNG
                let mut rng = create_instance_prng(seed, generator, solver, instance_id);
                
                // Generate maze
                let maze = generators::generate_maze(generator, &mut rng, rows, cols, &generator_params);
                
                // Solve maze
                let solution = solvers::solve_maze(solver, &maze);
                
                // Create result
                let result = MazeResult {
                    instance_id,
                    maze,
                    solution,
                    generator,
                    solver,
                    seed,
                };
                
                // Add to batch
                batch_results.push(result);
            }
            
            // Process and send entire batch as bytes
            let batch_bytes = process_batch(&batch_results);
            tx.send(batch_bytes).unwrap();
        });
    
    
    // Channel will be closed when all senders are dropped
    // Wait for writer to finish
    writer_handle.join().unwrap()?;
    
    let elapsed = start_time.elapsed();
    let rate = count as f64 / elapsed.as_secs_f64();
    
    println!("\nCompleted in {:.2}s", elapsed.as_secs_f64());
    println!("Generated {} mazes at {:.2} mazes/second", count, rate);
    
    Ok(())
}
