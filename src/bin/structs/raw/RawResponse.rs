use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RawResponse {
    pub features: Option<Vec<RawFeature>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawFeature {
    pub geometry: Option<RawGeometry>,
    pub id: Option<i64>,
    pub properties: Option<RawProperty>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawGeometry {
    pub coordinates: Option<Vec<f64>>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RawProperty {
    pub OBJECTID: Option<i64>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub DestCode: Option<String>,
    pub EventCode: Option<String>,
    pub Heading: Option<String>,
    pub TrainNum: Option<String>,
    pub Velocity: Option<String>,
    pub OrigCode: Option<String>,
    pub RouteName: Option<String>,
    pub OrigSchDep: Option<String>,
    pub OriginTZ: Option<String>,
    pub TrainState: Option<String>,
    pub updated_at: Option<String>,

    pub Station1: Option<String>,
    pub Station2: Option<String>,
    pub Station3: Option<String>,
    pub Station4: Option<String>,
    pub Station5: Option<String>,
    pub Station6: Option<String>,
    pub Station7: Option<String>,
    pub Station8: Option<String>,
    pub Station9: Option<String>,
    pub Station10: Option<String>,
    pub Station11: Option<String>,
    pub Station12: Option<String>,
    pub Station13: Option<String>,
    pub Station14: Option<String>,
    pub Station15: Option<String>,
    pub Station16: Option<String>,
    pub Station17: Option<String>,
    pub Station18: Option<String>,
    pub Station19: Option<String>,
    pub Station20: Option<String>,
    pub Station21: Option<String>,
    pub Station22: Option<String>,
    pub Station23: Option<String>,
    pub Station24: Option<String>,
    pub Station25: Option<String>,
    pub Station26: Option<String>,
    pub Station27: Option<String>,
    pub Station28: Option<String>,
    pub Station29: Option<String>,
    pub Station30: Option<String>,
    pub Station31: Option<String>,
    pub Station32: Option<String>,
    pub Station33: Option<String>,
    pub Station34: Option<String>,
    pub Station35: Option<String>,
    pub Station36: Option<String>,
    pub Station37: Option<String>,
    pub Station38: Option<String>,
    pub Station39: Option<String>,
    pub Station40: Option<String>,
    pub Station41: Option<String>,
    pub Station42: Option<String>,
    pub Station43: Option<String>,
    pub Station44: Option<String>,
    pub Station45: Option<String>,
    pub Station46: Option<String>,
}
