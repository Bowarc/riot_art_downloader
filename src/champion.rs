use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct ChampionList {
    #[serde(rename = "type")]
    pub _type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, Champion>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Info {
    pub attack: f32,
    pub defense: f32,
    pub magic: f32,
    pub difficulty: f32,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct DetailedChampionList {
    #[serde(rename = "type")]
    pub _type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, DetailedChampion>,
}

// #[derive(Serialize, Deserialize, Default, Clone, Debug)]
// pub struct DetailedChampion {
//     #[serde(rename = "@champ_name")]
//     pub champ_name: DetailedChampion1,
// }

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct DetailedChampion {
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub image: Image,
    pub skins: Vec<Skin>,
    pub lore: String,
    pub blurb: String,
    pub allytips: Vec<String>,
    pub enemytips: Vec<String>,
    pub tags: Vec<String>,
    pub info: Info,
    pub stats: Stats,
    pub spells: Vec<Spell>,
    pub passive: Passive,
    pub recommended: Vec<String>, // ???
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Skin {
    pub id: String,
    pub num: f32,
    pub name: String,
    pub chromas: bool,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Spell {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tooltip: String,
    pub leveltip: Option<LevelTip>,
    pub maxrank: f32,
    pub cooldown: Vec<f32>,
    #[serde(rename = "cooldownBurn")]
    pub cooldown_burn: String,
    pub cost: Vec<f32>,
    #[serde(rename = "costBurn")]
    pub cost_burn: String,
    pub datavalues: HashMap<i32, String>, // ???
    pub effect: Vec<Option<Vec<f32>>>,
    #[serde(rename = "effectBurn")]
    pub effect_burn: Vec<Option<String>>,
    pub vars: Vec<String>,
    #[serde(rename = "costType")]
    pub cost_type: String,
    pub maxammo: String,
    pub range: Vec<f32>,
    pub image: Image,
    pub resource: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Passive {
    pub name: String,
    pub description: String,
    pub image: Image,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct LevelTip {
    pub label: Vec<String>,
    pub effect: Vec<String>,
}
