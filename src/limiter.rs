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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_same_client_after_bucket_is_exhausted() {
        let limiter = RateLimiter::new();

        for _ in 0..100 {
            assert!(limiter.allow("127.0.0.1").is_ok());
        }

        let result = limiter.allow("127.0.0.1");
        assert!(matches!(result, Err(BucketError::NotEnoughTokens { .. })));
    }

    #[test]
    fn tracks_each_client_in_a_separate_bucket() {
        let limiter = RateLimiter::new();

        for _ in 0..100 {
            assert!(limiter.allow("127.0.0.1").is_ok());
        }

        assert!(matches!(
            limiter.allow("127.0.0.1"),
            Err(BucketError::NotEnoughTokens { .. })
        ));

        assert!(limiter.allow("127.0.0.2").is_ok());
    }

    #[test]
    fn rejects_invalid_batch_size() {
        let limiter = RateLimiter::new();

        let result = limiter.allow_n("127.0.0.1", 0.0);
        assert!(matches!(result, Err(BucketError::InvalidTokenAmount(0.0))));
    }
}
