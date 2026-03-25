# rate-limiter

A minimal rate-limiter that uses token bucket algorithm and lru caching for storing user related buckets.
By defualt user can make 100 requests per minute.

## Start server

```bash
cargo run
```

## Run loop script

```bash
for i in {1..120}; do curl -i http://127.0.0.1:3000; done
```
