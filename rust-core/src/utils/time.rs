extern crate chrono;
use chrono::prelude::*;

pub fn current_time_with_timezone() -> String {
    let local_time = Utc::now().with_timezone(&Local);

    local_time.format("%Y-%m-%d %H:%M:%S %z").to_string()
}
