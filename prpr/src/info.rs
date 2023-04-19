use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
pub enum ChartFormat {
    Rpe = 0,
    Pec,
    Pgr,
    Pbc,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct ChartInfo {
    pub id: Option<i32>,

    pub name: String,
    pub difficulty: f32,
    pub level: String,
    pub charter: String,
    pub composer: String,
    pub illustrator: String,

    pub chart: String,
    pub format: Option<ChartFormat>,
    pub music: String,
    pub illustration: String,

    pub preview_start: f32,
    pub preview_end: Option<f32>,
    pub aspect_ratio: f32,
    pub background_dim: f32,
    pub line_length: f32,
    pub offset: f32,
    pub tip: Option<String>,
    pub tags: Vec<String>,

    pub intro: String,

    pub hold_partial_cover: bool,
}

impl Default for ChartInfo {
    fn default() -> Self {
        Self {
            id: None,

            name: "UK".to_string(),
            difficulty: 10.,
            level: "UK Lv.10".to_string(),
            charter: "UK".to_string(),
            composer: "UK".to_string(),
            illustrator: "UK".to_string(),

            chart: "chart.json".to_string(),
            format: None,
            music: "song.mp3".to_string(),
            illustration: "background.png".to_string(),

            preview_start: 0.,
            preview_end: None,
            aspect_ratio: 16. / 9.,
            background_dim: 0.6,
            line_length: 6.,
            offset: 0.,
            tip: None,
            tags: Vec::new(),

            intro: String::new(),

            hold_partial_cover: false,
        }
    }
}
