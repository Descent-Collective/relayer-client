use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_unix_timestamp() -> u64 {
    // Get the current time as a SystemTime
    let current_time = SystemTime::now();

    // Calculate the Unix timestamp
    let unix_timestamp = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    println!("Current Unix timestamp: {}", unix_timestamp);

    unix_timestamp
}
