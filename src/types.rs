// Shared types
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Time,
    Distance,
}

#[derive(Serialize)]
pub struct SplitResult {
    pub time: String,
    pub distance: String,
    pub pace: String,
    pub watts: String,
}

#[derive(Debug, Deserialize)]
pub struct Concept2DataPoint {
    // Distance is in .1m. i.e., 1m -> 10
    #[serde(rename = "d")]
    pub distance_dm: u32,
    // Pace is in distance over time??? It's larger when slower
    //#[serde(rename = "p")]
    //pace: u32,
    // Heart rate in bpm, unused atm
    //#[serde(rename = "hr")]
    //heart_rate: u32,
    // Time is in .1s. i.e., 1s -> 10. Intervals of 2s
    #[serde(rename = "t")]
    pub time_ds: u32,
}
