use std::collections::HashMap;
use crate::GeneratorType;

/// Parameters for generators
#[derive(Debug, Clone)]
pub struct GeneratorParams {
    params: HashMap<String, f64>,
}

impl GeneratorParams {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }
    
    pub fn get(&self, key: &str, default: f64) -> f64 {
        self.params.get(key).copied().unwrap_or(default)
    }
    
    pub fn from_vec(pairs: Vec<(String, String)>) -> Result<Self, String> {
        let mut params = HashMap::new();
        for (key, value) in pairs {
            let val = value.parse::<f64>()
                .map_err(|_| format!("Invalid value for parameter '{}': '{}' (must be a number)", key, value))?;
            params.insert(key, val);
        }
        Ok(Self { params })
    }
}

impl Default for GeneratorParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameter information for help display
#[derive(Debug, Clone)]
pub struct ParamInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub default: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

/// Get parameter descriptions for a generator
pub fn get_generator_params(generator: GeneratorType) -> Vec<ParamInfo> {
    match generator {
        GeneratorType::DrunkardsWalk => vec![
            ParamInfo {
                name: "coverage",
                description: "Fraction of cells to carve (0.0 to 1.0)",
                default: 0.5,
                min: Some(0.01),
                max: Some(1.0),
            },
        ],
        GeneratorType::Searchformer => vec![
            ParamInfo {
                name: "min_path_length",
                description: "Minimum required path length from start to goal",
                default: 10.0,
                min: Some(1.0),
                max: None,
            },
            ParamInfo {
                name: "wall_sample_rate",
                description: "Fraction of walls to randomly remove",
                default: 0.3,
                min: Some(0.0),
                max: Some(1.0),
            },
        ],
        // Generators without parameters
        GeneratorType::Wilson | GeneratorType::Dfs | GeneratorType::Kruskal => vec![],
    }
}

/// Print parameter help for a generator
pub fn print_param_help(generator: GeneratorType) {
    let params = get_generator_params(generator);
    
    if params.is_empty() {
        println!("Generator '{}' has no configurable parameters.", format!("{:?}", generator).to_lowercase());
        return;
    }
    
    println!("Parameters for '{}' generator:", format!("{:?}", generator).to_lowercase());
    println!();
    
    for param in params {
        println!("  --param {}=<value>", param.name);
        println!("    {}", param.description);
        println!("    Default: {}", param.default);
        if let (Some(min), Some(max)) = (param.min, param.max) {
            println!("    Range: {} to {}", min, max);
        } else if let Some(min) = param.min {
            println!("    Minimum: {}", min);
        } else if let Some(max) = param.max {
            println!("    Maximum: {}", max);
        }
        println!();
    }
}

/// Print help for all generators
pub fn print_all_params_help() {
    println!("Generator Parameters:");
    println!("====================");
    println!();
    
    for generator in [
        GeneratorType::Wilson,
        GeneratorType::Dfs,
        GeneratorType::Kruskal,
        GeneratorType::DrunkardsWalk,
        GeneratorType::Searchformer,
    ] {
        let params = get_generator_params(generator);
        let gen_name = format!("{:?}", generator).to_lowercase();
        
        if params.is_empty() {
            println!("{}: No parameters", gen_name);
        } else {
            println!("{}: {} parameter(s)", gen_name, params.len());
            for param in params {
                println!("  - {}: {} (default: {})", param.name, param.description, param.default);
            }
        }
        println!();
    }
}