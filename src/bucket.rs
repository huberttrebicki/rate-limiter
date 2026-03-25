use std::{error::Error, fmt, time::Instant};

#[derive(Debug)]
pub enum BucketError {
    InvalidTokenAmount(f64),
    InvalidMaxTokens(f64),
    InvalidRefillRate(f64),
    NotEnoughTokens { requested: f64, available: f64 },
}

impl fmt::Display for BucketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BucketError::InvalidTokenAmount(n) => {
                write!(f, "token amount must be > 0, got {n}")
            }
            BucketError::InvalidMaxTokens(n) => {
                write!(f, "max_tokens must be > 0 and finite, got {n}")
            }
            BucketError::InvalidRefillRate(n) => {
                write!(f, "refill_rate must be > 0 and finite, got {n}")
            }
            BucketError::NotEnoughTokens {
                requested,
                available,
            } => {
                write!(
                    f,
                    "not enough tokens: requested {requested}, available {available}"
                )
            }
        }
    }
}

impl Error for BucketError {}

#[derive(Debug)]
pub struct Bucket {
    max_tokens: f64,
    remaining_tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

impl Bucket {
    pub fn new() -> Self {
        Self {
            max_tokens: 100.0,
            remaining_tokens: 100.0,
            refill_rate: 100.0 / 60.0,
            last_refill: Instant::now(),
        }
    }

    //Meant for special types of users with different limits.
    pub fn with_limit(max_tokens: f64, refill_rate: f64) -> Result<Self, BucketError> {
        if !max_tokens.is_finite() || max_tokens <= 0.0 {
            return Err(BucketError::InvalidMaxTokens(max_tokens));
        }

        if !refill_rate.is_finite() || refill_rate <= 0.0 {
            return Err(BucketError::InvalidRefillRate(refill_rate));
        }
        Ok(Self {
            max_tokens,
            remaining_tokens: max_tokens,
            refill_rate,
            last_refill: Instant::now(),
        })
    }

    pub fn try_consume(&mut self) -> Result<(), BucketError> {
        self.try_consume_n(1.0)
    }

    pub fn try_consume_n(&mut self, n: f64) -> Result<(), BucketError> {
        if !n.is_finite() || n <= 0.0 {
            return Err(BucketError::InvalidTokenAmount(n));
        }

        self.refill();

        if self.remaining_tokens >= n {
            self.remaining_tokens -= n;
            Ok(())
        } else {
            Err(BucketError::NotEnoughTokens {
                requested: n,
                available: self.remaining_tokens,
            })
        }
    }

    pub fn available_tokens(&mut self) -> f64 {
        self.refill();
        self.remaining_tokens
    }

    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();
        let tokens_to_add = self.refill_rate * elapsed;
        self.remaining_tokens = f64::min(self.remaining_tokens + tokens_to_add, self.max_tokens);
        self.last_refill = Instant::now();
    }
}
