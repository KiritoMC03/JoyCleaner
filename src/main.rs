mod code;

use code::*;
use trash;

pub(crate) const PATH_TO_DATA_FOLDER : &str = "C:/Users/kiril/OneDrive/Documents/JoyCleaner";
pub(crate) const CSV_PATHS : &str = "/paths.xlsx";

fn main() {
    let caches: Vec<ProgramCachePath> = get_cache_paths();
    for cache in caches {
        remove_cache(&cache);
    }

    println!("Finished!");
}

fn remove_cache(cache: &ProgramCachePath) {
    let mut path : String = cache.root.clone();
    if cache.cache.len() > 0 {
        path = (path + "/" + &cache.cache).replace("\\", "/");
    }
    match trash::delete(&path) {
        Ok(_) => println!("Cache at path {} was removed!", &path),
        Err(err) => println!("Can not remove cache at path: {} ({})", &path, err),
    };
}