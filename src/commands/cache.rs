use clap::{Clap, AppSettings};
use crate::data::config::Config;

use remove_dir_all::*;
use owo_colors::*;
use walkdir::WalkDir;

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Cache {
    /// Clear the cache
    #[clap(subcommand)]
    pub op: CacheOperation
}

#[derive(Clap, Debug, Clone)]
pub enum CacheOperation {
    /// Clear the cache
    Clear,
    /// Lists versions for each cached package
    List
}

pub fn execute_cache_operation(operation: Cache)
{
    match operation.op {
        CacheOperation::Clear => clear(),
        CacheOperation::List => list()
    }
}

fn clear()
{
    let config = Config::read_combine();
    let path = config.cache.unwrap();
    remove_dir_contents(path).expect("Failed to remove cached folders");
}

fn list()
{
    let config = Config::read_combine();
    let path = config.cache.unwrap();

    for dir in WalkDir::new(&path).max_depth(2).min_depth(1) {
        let unwrapped = dir.unwrap();
        if unwrapped.depth() == 1 {
            println!("package {}:", unwrapped.file_name().to_string_lossy().bright_red());
        } else {
            println!(" - {}", unwrapped.file_name().to_string_lossy().bright_green());
        }
    }
}