extern crate walkdir;

mod code;

use code::*;
use trash;
use walkdir::WalkDir;
use std::path::Path;

pub(crate) const PATH_TO_DATA_FOLDER : &str = "C:/Users/kiril/OneDrive/Documents/JoyCleaner";
pub(crate) const CSV_PATHS : &str = "/paths.xlsx";

fn main() {
    println!("Clean started!\n");
    let caches: Vec<ProgramCachePath> = get_cache_paths();
    let mut total_size = 0u64;
    for cache in caches {
        total_size += remove_cache(&cache);
    }

    println!("\nFinished!\nTotal cleaned: {}mb", total_size);
    loop {}
}

fn remove_cache(cache: &ProgramCachePath) -> u64 {
    let mut path : String = cache.root.clone();
    if cache.cache.len() > 0 {
        path = (path + "/" + &cache.cache).replace("\\", "/");
    }
    let size = bytes_to_mb(dir_size(&path));
    match trash::delete(&path) {
        Ok(_) => println!("Cache at path {} ({}mb) was removed!", &path, size),
        Err(err) => println!("Can not remove cache at path: {} ({})", &path, err),
    };

    size
}

fn dir_size<P: AsRef<Path>>(path: &P) -> u64 {
    WalkDir::new(path)
        .min_depth(1)
        .max_depth(100)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len())
}

fn bytes_to_mb(bytes: u64) -> u64 {
    bytes / (1024 * 1025)
}