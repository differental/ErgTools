// ErgTools - Rust-based web app & CLI application to easily calculate rowing splits and analyse performance.
// Copyright (C) 2025 Brian Chen (differental)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
