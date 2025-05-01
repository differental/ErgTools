use chrono::{Duration, NaiveTime};

const MIDNIGHT: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

pub fn parse_time(s: &str) -> Option<f64> {
    NaiveTime::parse_from_str(s.trim(), "%H:%M:%S%.f")
        .ok()
        .map(|t| Duration::from(t.signed_duration_since(MIDNIGHT)).as_seconds_f64())
}

pub fn format_time(secs: f64) -> String {
    let total_seconds = secs.floor() as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = secs % 60.0;

    format!("{:01}:{:02}:{:04.1}", hours, minutes, seconds)
}

/// (known_time, target_time, split_distances:Vec<u32>) -> Vec<(time: f64, dist: u32)>
/// Note that this does not follow concept2's splits tables for "time intervals".
///     Concept2 times are cumulative and distances are not. Here they are both not cumulative.
/// E.g., Return data: [(240.0, 300), (240.0, 300), (240.0, 300)]
pub fn process_time_splits(known_time: f64, target_time: f64, split_distances: Vec<u32>) -> Vec<(f64, u32)> {
    let total_time = known_time * split_distances.len() as f64;
    let new_split_len = (total_time / target_time).ceil() as u32;
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
        new_split_distances.push((target_time, (curr_measured_distance - curr_measured_time * latest_speed) as u32));   
        curr_measured_distance = curr_measured_time * latest_speed;
    }
    new_split_distances
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
        assert_eq!(result, vec![(240.0, 320), (240.0, 80), (240.0, 160), (240.0, 240), (240.0, 0), (240.0, 320), (60.0, 80)]);

        // known < target & incomplete
        let known_time = 300.0; // 5min
        let target_time = 600.0; // 10min
        let split_distances = vec![400, 200, 400, 200, 400];
        let result = process_time_splits(known_time, target_time, split_distances);
        assert_eq!(result, vec![(600.0, 600), (600.0, 600), (300.0, 400)]);
    }
}