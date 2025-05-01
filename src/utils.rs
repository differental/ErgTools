use std::error::Error;

pub fn parse_time(s: &str) -> Result<f64, Box<dyn Error>> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return Err("Time format incorrect".into());
    }

    let hours: f64 = parts[0].parse()?;
    let minutes: f64 = parts[1].parse()?;
    let seconds: f64 = parts[2].parse()?;

    Ok(hours * 3600.0 + minutes * 60.0 + seconds)
}

pub fn format_time(secs: f64) -> String {
    let total_seconds = secs.floor() as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = secs % 60.0;

    format!("{:01}:{:02}:{:04.1}", hours, minutes, seconds)
}
