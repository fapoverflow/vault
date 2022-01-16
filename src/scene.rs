use crate::model::Scene;
use crate::util;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn remove_missing(fs_root: &str, mnt_root: &str, lib_root: &str) -> Result<(), Box<dyn Error>> {
    let lib = util::read_library(&Path::new(lib_root))?;
    let mut new_scenes = Vec::new();
    for scene in lib.scenes {
        let fs_path = Path::new(fs_root).join(Path::new(&scene.path).strip_prefix(mnt_root)?);
        if fs_path.exists() {
            new_scenes.push(scene);
            continue;
        }
        println!("missing: {}", fs_path.display());
    }
    write(&Path::new(lib_root).join("scenes2.db"), &new_scenes)?;
    Ok(())
}

pub fn write(path: &Path, scenes: &Vec<Scene>) -> Result<(), Box<dyn Error>> {
    let payload = scenes
        .into_iter()
        .map(|scene| format!("{}", serde_json::to_string(scene).unwrap()))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(fs::write(path, payload)?)
}
