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

use crate::types::Concept2DataPoint;

/// (known_time, target_time, split_distances:Vec<u32>) -> Vec<(time: f64, dist: u32)>
/// Note that this does not follow concept2's splits tables for "time intervals".
///     Concept2 times are cumulative and distances are not. Here they are both not cumulative.
/// E.g., Return data: [(240.0, 300), (240.0, 300), (240.0, 300)]
pub fn process_time_splits(
    known_time: f64,
    target_time: f64,
    split_distances: Vec<u32>,
) -> Vec<(f64, u32)> {
    let total_time = known_time * split_distances.len() as f64;
    let new_split_len = (total_time / target_time).ceil() as u32;

    if new_split_len > 100 {
        // guard
        return vec![];
    }

    let mut curr_measured_time = known_time;
    let mut curr_measured_distance = split_distances[0] as f64;
    let mut latest_speed = split_distances[0] as f64 / known_time;
    let mut idx = 0;

    let mut new_split_distances = Vec::<(f64, u32)>::new();

    'outer: for _ in 0..new_split_len {
        while curr_measured_time < target_time {
            idx += 1;
            if idx == split_distances.len() {
                // Final incomplete section
                new_split_distances.push((curr_measured_time, curr_measured_distance as u32));

                break 'outer;
            }
            curr_measured_time += known_time;
            curr_measured_distance += split_distances[idx] as f64;
            latest_speed = split_distances[idx] as f64 / known_time;
        }
        curr_measured_time -= target_time;
        new_split_distances.push((
            target_time,
            (curr_measured_distance - curr_measured_time * latest_speed) as u32,
        ));
        curr_measured_distance = curr_measured_time * latest_speed;
    }
    new_split_distances
}

/// (known_distance, target_distance, split_times:Vec<u32>) -> Vec<(dist: u32, time: f64)>
/// Note that this does not follow concept2's splits tables for "distance intervals".
///     Concept2 distances are cumulative and times are not. Here they are both not cumulative.
/// E.g., Return data: [(1000, 220.0), (1000, 215.0), (500, 100.0)]
pub fn process_distance_splits(
    known_distance: u32,
    target_distance: u32,
    split_times: Vec<f64>,
) -> Vec<(u32, f64)> {
    let total_distance = known_distance * split_times.len() as u32;
    let new_split_len = (total_distance as f64 / target_distance as f64).ceil() as u32;

    if new_split_len > 100 {
        // guard
        return vec![];
    }

    let mut curr_measured_distance = known_distance;
    let mut curr_measured_time = split_times[0];
    let mut latest_speed = known_distance as f64 / split_times[0];
    let mut idx = 0;

    let mut new_split_times = Vec::<(u32, f64)>::new();

    'outer: for _ in 0..new_split_len {
        while curr_measured_distance < target_distance {
            idx += 1;
            if idx == split_times.len() {
                // Final incomplete section
                new_split_times.push((curr_measured_distance, curr_measured_time));

                break 'outer;
            }
            curr_measured_distance += known_distance;
            curr_measured_time += split_times[idx];
            latest_speed = known_distance as f64 / split_times[idx];
        }
        curr_measured_distance -= target_distance;
        new_split_times.push((
            target_distance,
            curr_measured_time - curr_measured_distance as f64 / latest_speed,
        ));
        curr_measured_time = curr_measured_distance as f64 / latest_speed;
    }
    new_split_times
}

pub fn process_concept2_time(data: Vec<Concept2DataPoint>, target_time: f64) -> Vec<(f64, u32)> {
    if target_time < 30.0 {
        // guard
        return vec![];
    }

    let mut last_datapoint_time_ds = 0.0;
    let mut last_datapoint_distance_dm = 0.0;
    let mut cumulative_split_distance_dm = 0.0;

    let mut next_split_time_ds = target_time * 10.0;

    let mut result = Vec::<(f64, u32)>::new();

    for item in data {
        if item.time_ds as f64 >= next_split_time_ds {
            let new_dist_dm = last_datapoint_distance_dm
                + (item.distance_dm as f64 - last_datapoint_distance_dm)
                    / (item.time_ds as f64 - last_datapoint_time_ds)
                    * (next_split_time_ds - last_datapoint_time_ds);

            result.push((
                target_time,
                ((new_dist_dm - cumulative_split_distance_dm) / 10.0) as u32,
            ));

            if result.len() > 100 {
                // guard
                return vec![];
            }

            cumulative_split_distance_dm = new_dist_dm;
            next_split_time_ds += target_time * 10.0;
        } else {
            last_datapoint_time_ds = item.time_ds as f64;
            last_datapoint_distance_dm = item.distance_dm as f64;
        }
    }

    // Remaining bits
    // (last_datapoint_time_ds - (next_split_time_ds - target_time * 10.0)) / 10.0,
    if next_split_time_ds > last_datapoint_time_ds {
        result.push((
            target_time - (next_split_time_ds - last_datapoint_time_ds) / 10.0,
            ((last_datapoint_distance_dm - cumulative_split_distance_dm) / 10.0) as u32,
        ));
    }

    result
}

pub fn process_concept2_distance(
    data: Vec<Concept2DataPoint>,
    target_distance: u32,
) -> Vec<(u32, f64)> {
    if target_distance < 50 {
        // guard
        return vec![];
    }

    let mut last_datapoint_time_ds = 0.0;
    let mut last_datapoint_distance_dm = 0.0;
    let mut cumulative_split_time_ds = 0.0;

    let mut next_split_distance_dm = target_distance as f64 * 10.0;

    let mut result = Vec::<(u32, f64)>::new();

    for item in data {
        if item.distance_dm as f64 >= next_split_distance_dm {
            let new_time_ds = last_datapoint_time_ds
                + (item.time_ds as f64 - last_datapoint_time_ds)
                    / (item.distance_dm as f64 - last_datapoint_distance_dm)
                    * (next_split_distance_dm - last_datapoint_distance_dm);

            result.push((
                target_distance,
                (new_time_ds - cumulative_split_time_ds) / 10.0,
            ));

            if result.len() > 100 {
                // guard
                return vec![];
            }

            cumulative_split_time_ds = new_time_ds;
            next_split_distance_dm += target_distance as f64 * 10.0;
        } else {
            last_datapoint_time_ds = item.time_ds as f64;
            last_datapoint_distance_dm = item.distance_dm as f64;
        }
    }

    // Remaining bits
    // ((last_datapoint_distance_dm - (next_split_distance_dm - target_distance as f64 * 10.0)) / 10.0) as u32
    if next_split_distance_dm > last_datapoint_distance_dm {
        result.push((
            target_distance - ((next_split_distance_dm - last_datapoint_distance_dm) / 10.0) as u32,
            (last_datapoint_time_ds - cumulative_split_time_ds) / 10.0,
        ));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_time_splits() {
        // known > target & incomplete
        let known_time = 300.0; // 5min
        let target_time = 240.0; // 4min
        let split_distances = vec![400, 0, 400, 0, 400];
        let result = process_time_splits(known_time, target_time, split_distances);
        assert_eq!(
            result,
            vec![
                (240.0, 320),
                (240.0, 80),
                (240.0, 160),
                (240.0, 240),
                (240.0, 0),
                (240.0, 320),
                (60.0, 80)
            ]
        );

        // known < target & incomplete
        let known_time = 300.0; // 5min
        let target_time = 600.0; // 10min
        let split_distances = vec![400, 200, 400, 200, 400];
        let result = process_time_splits(known_time, target_time, split_distances);
        assert_eq!(result, vec![(600.0, 600), (600.0, 600), (300.0, 400)]);
    }

    #[test]
    fn test_process_distance_splits() {
        // known > target & incomplete
        let known_distance = 500;
        let target_distance = 400;
        let split_times = vec![120.0, 100.0, 110.0, 105.0, 100.0];
        let result = process_distance_splits(known_distance, target_distance, split_times);
        assert_eq!(
            result,
            vec![
                (400, 96.0),
                (400, 84.0),
                (400, 84.0),
                (400, 87.0),
                (400, 84.0),
                (400, 80.0),
                (100, 20.0)
            ]
        );

        // known < target & incomplete
        let known_distance = 500;
        let target_distance = 1000;
        let split_times = vec![120.0, 100.0, 110.0, 105.0, 100.0];
        let result = process_distance_splits(known_distance, target_distance, split_times);
        assert_eq!(result, vec![(1000, 220.0), (1000, 215.0), (500, 100.0)]);
    }
}
