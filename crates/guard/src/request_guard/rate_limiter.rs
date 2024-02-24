use chrono::{DateTime, Utc};
use rocket::request::FromRequest;
use std::{collections::HashMap, hash::Hash, sync::Arc};
use tokio::sync::Mutex;

pub struct RateLimiter<K> {
    pub max_requests: u32,
    pub duration: u32,
    pub map: Arc<Mutex<HashMap<K, Vec<DateTime<Utc>>>>>,
}

#[derive(Debug, Clone)]
pub struct RateLimit {
    pub max_requests: u32,
    pub remaining_requests: u32,
    pub reset_time: DateTime<Utc>,
}

impl<K> RateLimiter<K> {
    #[inline]
    pub fn new(max_requests: u32, duration: u32) -> Self {
        RateLimiter {
            max_requests,
            duration,
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn increment(&self, key: K) -> RateLimit
    where
        K: Eq + Hash,
    {
        let mut map = self.map.lock().await;
        let mut requests = map.entry(key).or_insert_with(Vec::new);
        let now = Utc::now();
        requests.retain(|&time| time > now - chrono::Duration::seconds(self.duration as i64));
        let remaining_requests = self.max_requests.saturating_sub(requests.len() as u32);
        let reset_time = if remaining_requests == 0 {
            requests
                .first()
                .map(|&time| time + chrono::Duration::seconds(self.duration as i64))
        } else {
            None
        };
        if remaining_requests > 0 {
            requests.push(now);
        }

        RateLimit {
            max_requests: self.max_requests,
            remaining_requests,
            reset_time: reset_time.unwrap_or(now + chrono::Duration::seconds(self.duration as i64)),
        }
    }
}

impl RateLimit {
    #[inline]
    pub const fn available(&self) -> bool {
        self.remaining_requests != 0
    }
}
