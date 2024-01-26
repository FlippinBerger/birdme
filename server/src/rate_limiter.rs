use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct RateLimiter {
    map: RwLock<HashMap<String, DateTime<Utc>>>,
    duration: i64,
}

impl RateLimiter {
    pub fn new(duration: i64) -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
            duration,
        }
    }

    // can_request returns true when the current IP isn't hitting the set
    // rate limit
    pub fn can_request(&self, ip: String) -> bool {
        let now = chrono::Utc::now();

        match self.map.read().expect("reading from the map").get(&ip) {
            Some(t) => {
                let upper_time_limit = *t + chrono::Duration::seconds(self.duration);
                if now.le(&upper_time_limit) {
                    return false;
                }
            }
            None => {}
        }

        let mut lock = self.map.write().expect("locking the hash map to write");
        lock.insert(ip, now);
        true
    }
}
