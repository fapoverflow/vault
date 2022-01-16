use crate::model::{
    Actor, ActorReference, Deleted, Image, Label, LabelledItem, Library, Marker, Scene, Studio,
};
use std::error::Error;
use std::path::Path;

pub fn vec_type_or_deleted_from_file<T: serde::de::DeserializeOwned>(
    path: &Path,
) -> Result<(Vec<T>, Vec<Deleted>), Box<dyn Error>> {
    let (v, d): (Vec<Option<T>>, Vec<Option<Deleted>>) = std::fs::read_to_string(Path::new(path))?
        .lines()
        .into_iter()
        .map(|l| {
            let err1: String;
            match serde_json::from_str::<T>(&l) {
                Ok(val) => return (Some(val), None),
                Err(e) => {
                    err1 = format!("{:?}", e);
                }
            };
            let err2: String;
            match serde_json::from_str::<Deleted>(&l) {
                Ok(deleted) => return (None, Some(deleted)),
                Err(e) => {
                    err2 = format!("{:?}", e);
                }
            };
            println!("Err1: {}", err1);
            println!("Err2: {}", err2);
            panic!("Couldn't parse: {} {:?}", &l, path)
        })
        .unzip();

    Ok((
        v.into_iter().flat_map(|e| e).collect(),
        d.into_iter().flat_map(|e| e).collect(),
    ))
}

pub fn read_library(path: &Path) -> Result<Library, Box<dyn Error>> {
    let (a, ad) = vec_type_or_deleted_from_file::<Actor>(&path.join("actors.db"))?;
    let (ar, ard) =
        vec_type_or_deleted_from_file::<ActorReference>(&path.join("actor_references.db"))?;
    let (i, id) = vec_type_or_deleted_from_file::<Image>(&path.join("images.db"))?;
    let (l, ld) = vec_type_or_deleted_from_file::<Label>(&path.join("labels.db"))?;
    let (li, lid) = vec_type_or_deleted_from_file::<LabelledItem>(&path.join("labelled_items.db"))?;
    let (m, md) = vec_type_or_deleted_from_file::<Marker>(&path.join("markers.db"))?;
    let (sc, scd) = vec_type_or_deleted_from_file::<Scene>(&path.join("scenes.db"))?;
    let (st, std) = vec_type_or_deleted_from_file::<Studio>(&path.join("studios.db"))?;
    return Ok(Library {
        actors: a,
        actors_deleted: ad,
        actor_references: ar,
        actor_references_deleted: ard,
        images: i,
        images_deleted: id,
        labels: l,
        labels_deleted: ld,
        labelled_items: li,
        labelled_items_deleted: lid,
        markers: m,
        markers_deleted: md,
        scenes: sc,
        scenes_deleted: scd,
        studios: st,
        studios_deleted: std,
    });
}

pub fn str_search_permutations(source: &str, term: &str) -> bool {
    let stal = source.to_ascii_lowercase();
    let ttal = term.to_ascii_lowercase();
    let ttalns: String = ttal.chars().filter(|c| char::is_alphanumeric(*c)).collect();
    stal.contains(&ttal) || stal.contains(&ttalns)
}

pub fn scene_matches_studio(scene: &Scene, studio: &Studio) -> bool {
    let mut std_names: Vec<&str> = Vec::new();
    std_names.push(&studio.name);
    if let Some(aliases) = &studio.aliases {
        for alias in aliases {
            std_names.push(&alias);
        }
    }
    for name in std_names {
        if str_search_permutations(&scene.path, &name) {
            return true;
        }
    }
    return false;
}
