use std::{
    num::NonZeroUsize,
    sync::{Arc, Mutex},
};

use lru::LruCache;

use crate::{Bucket, BucketError};

const DEFAULT_CACHE_SIZE: usize = 10_000;

#[derive(Clone)]
pub struct RateLimiter {
    buckets: Arc<Mutex<LruCache<String, Bucket>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        let capacity =
            NonZeroUsize::new(DEFAULT_CACHE_SIZE).expect("default capacity must be non-zero");
        Self::with_capacity(capacity)
    }

    pub fn with_capacity(capacity: NonZeroUsize) -> Self {
        Self {
            buckets: Arc::new(Mutex::new(LruCache::new(capacity))),
        }
    }

    //Meant for batch requests or so...
    pub fn allow_n(&self, ip_address: &str, n: f64) -> Result<(), BucketError> {
        let mut buckets = self
            .buckets
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if buckets.get(ip_address).is_none() {
            buckets.put(ip_address.to_owned(), Bucket::new());
        }

        let client = buckets
            .get_mut(ip_address)
            .expect("client should exist after insertion");

        client.try_consume_n(n)
    }

    pub fn allow(&self, ip_address: &str) -> Result<(), BucketError> {
        self.allow_n(ip_address, 1.0)
    }
}
