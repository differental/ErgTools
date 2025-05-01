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

/// (known_distance, target_distance, split_times:Vec<u32>) -> Vec<(dist: u32, time: f64)>
/// Note that this does not follow concept2's splits tables for "distance intervals".
///     Concept2 distances are cumulative and times are not. Here they are both not cumulative.
/// E.g., Return data: [(1000, 220.0), (1000, 215.0), (500, 100.0)]
pub fn process_distance_splits(known_distance: u32, target_distance: u32, split_times: Vec<f64>) -> Vec<(u32, f64)> {
    let total_distance = known_distance * split_times.len() as u32;
    let new_split_len = (total_distance as f64 / target_distance as f64).ceil() as u32;
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
        new_split_times.push((target_distance, curr_measured_time - curr_measured_distance as f64 / latest_speed));   
        curr_measured_time = curr_measured_distance as f64 / latest_speed;
    }
    new_split_times
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

    #[test]
    fn test_process_distance_splits() {
        // known > target & incomplete
        let known_distance = 500;
        let target_distance = 400;
        let split_times = vec![120.0, 100.0, 110.0, 105.0, 100.0];
        let result = process_distance_splits(known_distance, target_distance, split_times);
        assert_eq!(result,vec![(400, 96.0), (400, 84.0), (400, 84.0), (400, 87.0), (400, 84.0), (400, 80.0), (100, 20.0)]);

        // known < target & incomplete
        let known_distance = 500;
        let target_distance = 1000;
        let split_times = vec![120.0, 100.0, 110.0, 105.0, 100.0];
        let result = process_distance_splits(known_distance, target_distance, split_times);
        assert_eq!(result,vec![(1000, 220.0), (1000, 215.0), (500, 100.0)]);
    }
}