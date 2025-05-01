// Shared types

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Time,
    Distance,
}

#[derive(Debug, Deserialize)]
pub struct Concept2DataPoint {
    // Distance is in .1m. i.e., 1m -> 10
    #[serde(rename = "d")]
    distance: u32,
    // Pace is in distance over time??? It's larger when slower
    #[serde(rename = "p")]
    pace: u32,
    // Time is in .1s. i.e., 1s -> 10. Intervals of 2s
    #[serde(rename = "t")]
    time: u32,
}
