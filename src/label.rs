use crate::model::LabelledItem;
use crate::{scene, util};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn scene_studios(lib_root: &str) -> Result<(), Box<dyn Error>> {
    let mut lib = util::read_library(&Path::new(lib_root))?;
    for scene in &mut lib.scenes {
        for studio in &lib.studios {
            if Some(&studio._id) == scene.studio.as_ref() {
                continue;
            }
            if util::scene_matches_studio(&scene, &studio) {
                println!("Scene {} missing studio {}", scene.path, studio.name);
                scene.studio = Some(studio._id.clone());
            }
        }
    }
    scene::write(&Path::new(lib_root).join("scenes.db"), &lib.scenes)?;
    Ok(())
}

pub fn remove_missing(lib_root: &str) -> Result<(), Box<dyn Error>> {
    let lib = util::read_library(&Path::new(lib_root))?;
    let scenes: HashMap<_, _> = lib.scenes.iter().map(|s| (s._id.as_str(), s)).collect();
    let mut new_labelled_items = Vec::new();
    for item in lib.labelled_items {
        match item.r#type.as_str() {
            "scene" => {
                if !scenes.contains_key(item.item.as_str()) {
                    continue;
                }
            }
            s => println!("Unhandled LabelledItem type: {}", s),
        }
        new_labelled_items.push(item);
    }
    write(
        &Path::new(lib_root).join("labelled_items.db"),
        &new_labelled_items,
    )?;
    Ok(())
}

pub fn write(path: &Path, lis: &Vec<LabelledItem>) -> Result<(), Box<dyn Error>> {
    let payload = lis
        .into_iter()
        .map(|li| format!("{}", serde_json::to_string(li).unwrap()))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(fs::write(path, payload)?)
}
