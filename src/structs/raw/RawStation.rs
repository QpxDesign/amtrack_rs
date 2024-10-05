use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RawStation {
    pub code: Option<String>,
    pub tz: Option<String>,
    pub bus: Option<bool>,
    pub scharr: Option<String>,
    pub schdep: Option<String>,
    pub schcmnt: Option<String>,
    pub autoarr: Option<bool>,
    pub autodep: Option<bool>,
    pub estarr: Option<String>,
    pub estdep: Option<String>,
    pub estarrcmnt: Option<String>,
    pub estdepcmnt: Option<String>,
    pub postdep: Option<String>,
    pub postarr: Option<String>,
    pub postcmnt: Option<String>,
}
