use std::error::Error;

/// Parse time in either h:mm:ss.s or mm:ss.s
pub fn parse_time(s: &str) -> Result<f64, Box<dyn Error>> {
    let parts: Vec<&str> = s.trim().split(':').collect();

    let total_seconds = match parts.len() {
        2 => {
            let minutes: f64 = parts[0].parse()?;
            let seconds: f64 = parts[1].parse()?;
            minutes * 60.0 + seconds
        }
        3 => {
            let hours: f64 = parts[0].parse()?;
            let minutes: f64 = parts[1].parse()?;
            let seconds: f64 = parts[2].parse()?;
            hours * 3600.0 + minutes * 60.0 + seconds
        }
        _ => return Err("Time format incorrect".into()),
    };

    Ok(total_seconds)
}

pub fn format_time(secs: f64, force_long: bool) -> String {
    let total_seconds = secs.floor() as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = secs % 60.0;

    if force_long || hours > 0 {
        format!("{:01}:{:02}:{:04.1}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:04.1}", minutes, seconds)
    }
}
