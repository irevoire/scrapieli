use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Output {
    pub hierarchy_radio_lvl0: Option<String>,
    pub hierarchy_radio_lvl1: Option<String>,
    pub hierarchy_radio_lvl2: Option<String>,
    pub hierarchy_radio_lvl3: Option<String>,
    pub hierarchy_radio_lvl4: Option<String>,
    pub hierarchy_radio_lvl5: Option<String>,
    pub hierarchy_lvl0: Option<String>,
    pub hierarchy_lvl1: Option<String>,
    pub hierarchy_lvl2: Option<String>,
    pub hierarchy_lvl3: Option<String>,
    pub hierarchy_lvl4: Option<String>,
    pub hierarchy_lvl5: Option<String>,
    pub hierarchy_lvl6: Option<String>,
    pub content: String,
    #[serde(rename = "objectID")]
    pub object_id: String,
    pub anchor: Option<String>,
    pub url: String,
}
