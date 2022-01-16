#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Library {
    pub actors: Vec<Actor>,
    pub actors_deleted: Vec<Deleted>,
    pub actor_references: Vec<ActorReference>,
    pub actor_references_deleted: Vec<Deleted>,
    pub images: Vec<Image>,
    pub images_deleted: Vec<Deleted>,
    pub labels: Vec<Label>,
    pub labels_deleted: Vec<Deleted>,
    pub labelled_items: Vec<LabelledItem>,
    pub labelled_items_deleted: Vec<Deleted>,
    pub markers: Vec<Marker>,
    pub markers_deleted: Vec<Deleted>,
    pub scenes: Vec<Scene>,
    pub scenes_deleted: Vec<Deleted>,
    pub studios: Vec<Studio>,
    pub studios_deleted: Vec<Deleted>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Deleted {
    pub _id: String,
    #[serde(rename = "$$deleted")]
    pub deleted: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActorReference {
    pub _id: String,
    pub actor: String,
    pub item: String,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Actor {
    pub _id: String,
    pub addedOn: u64,
    pub aliases: Vec<String>,
    pub altThumbnail: Option<String>,
    pub avatar: Option<String>,
    pub bornOn: Option<i64>,
    pub bookmark: Option<u64>,
    pub customFields: HashMap<String, String>,
    pub description: Option<String>,
    pub favorite: bool,
    pub hero: Option<String>,
    pub name: Option<String>,
    pub nationality: Option<String>,
    pub rating: Option<f64>,
    pub thumbnail: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Image {
    pub _id: String,
    pub addedOn: u64,
    pub album: Option<String>,
    pub bookmark: Option<String>,
    pub color: Option<String>,
    pub customFields: HashMap<String, String>,
    pub favorite: bool,
    pub hash: Option<String>,
    pub meta: serde_json::Value,
    pub name: String,
    pub path: String,
    pub rating: f64,
    pub scene: Option<String>,
    pub studio: Option<String>,
    pub thumbPath: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Label {
    pub _id: String,
    pub addedOn: u64,
    pub aliases: Vec<String>,
    pub name: String,
    pub thumbnail: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabelledItem {
    pub _id: String,
    pub item: String,
    pub label: String,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Marker {
    pub _id: String,
    pub addedOn: u64,
    pub bookmark: Option<u64>,
    pub customFields: HashMap<String, String>,
    pub favorite: bool,
    pub name: String,
    pub rating: f64,
    pub scene: String,
    pub thumbnail: Option<String>,
    pub time: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Scene {
    pub _id: String,
    pub addedOn: u64,
    pub album: Option<String>,
    pub bookmark: Option<u64>,
    pub customFields: HashMap<String, String>,
    pub description: Option<String>,
    pub favorite: bool,
    pub meta: serde_json::Value,
    pub name: String,
    pub path: String,
    pub preview: Option<String>,
    pub processed: bool,
    pub rating: u8,
    pub releasedate: Option<u64>,
    pub streamLinks: Vec<String>,
    pub studio: Option<String>,
    pub thumbnail: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Studio {
    pub _id: String,
    pub addedOn: u64,
    pub aliases: Option<Vec<String>>,
    pub bookmark: Option<String>,
    pub customFields: HashMap<String, String>,
    pub description: Option<String>,
    pub favorite: bool,
    pub name: String,
    pub parent: Option<String>,
    pub rating: f64,
    pub thumbnail: Option<String>,
}
