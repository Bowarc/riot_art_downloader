use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ChampionList {
    pub r#type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, Champion>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Champion {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: Info,
    pub image: Image,
    pub tags: Vec<String>,
    pub partype: String,
    pub stats: Stats,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Info {
    pub attack: f32,
    pub defense: f32,
    pub magic: f32,
    pub difficulty: f32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Stats {
    hp: f32,
    hpperlevel: f32,
    mp: f32,
    mpperlevel: f32,
    movespeed: f32,
    armor: f32,
    armorperlevel: f32,
    spellblock: f32,
    spellblockperlevel: f32,
    attackrange: f32,
    hpregen: f32,
    hpregenperlevel: f32,
    mpregen: f32,
    mpregenperlevel: f32,
    crit: f32,
    critperlevel: f32,
    attackdamage: f32,
    attackdamageperlevel: f32,
    attackspeedperlevel: f32,
    attackspeed: f32,
}
