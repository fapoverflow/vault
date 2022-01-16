#![forbid(unsafe_code)]
use std::env;
use std::error::Error;

mod label;
mod model;
mod scene;
mod tests;
mod util;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|e| e == "--help" || e == "-h") {
        println!("vault-utils <real root>:<mount root>:<library path> command");
        println!(" <real root>  - path to video folder on host.");
        println!(" <mount root> - path to video folder inside container.");
        println!(" <library>    - path to library folder on host.");
        println!(" Commands:");
        println!("  add_studios_to_series");
        println!("  remove_missing_scenes");
        println!("  remove_missing_labels");
        std::process::exit(1);
    }
    let mut path_map = args[1].splitn(3, ':');
    let fs_root = path_map.next().unwrap();
    let mnt_root = path_map.next().unwrap();
    let lib_root = path_map.next().unwrap();

    match args[2].as_str() {
        "add_studios_to_scenes" => label::scene_studios(lib_root),
        "remove_missing_scenes" => scene::remove_missing(fs_root, mnt_root, lib_root),
        "remove_missing_labels" => label::remove_missing(lib_root),
        _ => panic!("Invalid command"),
    }
}
