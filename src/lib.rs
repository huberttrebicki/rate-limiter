use std::time::Instant;

#[derive(Debug)]
pub struct Bucket {
    max_tokens: f64,
    remaining_tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            max_tokens: 100.0,
            remaining_tokens: 100.0,
            refill_rate: 1.67,
            last_refill: Instant::now(),
        }
    }
}
