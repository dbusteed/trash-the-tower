use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TowerData {
    pub kind: String,
    pub x: f32,
    pub y: f32,
    pub h: f32,
    pub w: f32,
}

#[derive(Deserialize, Debug)]
pub struct TargetData {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug)]
pub struct LevelData {
    pub tower: Vec<TowerData>,
    pub targets: Vec<TargetData>
}

pub fn read_levels() -> Vec<LevelData> {
    let json = std::fs::read_to_string("./assets/levels.json").expect("");
    let data: Vec<LevelData> = serde_json::from_str(&json).unwrap();
    data
}