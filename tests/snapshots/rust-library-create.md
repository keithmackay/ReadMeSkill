# fastcache

A high-performance, thread-safe caching library with TTL support. Uses parking_lot for low-contention locking and hashbrown for fast hash maps.

## Highlights

- **Thread-safe** — Safe for concurrent access from multiple threads
- **TTL support** — Entries expire automatically after a configurable duration
- **LRU eviction** — Bounded cache size with least-recently-used eviction
- **Zero-copy reads** — Clone-on-read for cached values
- **Benchmarked** — Criterion benchmarks for insert and get operations

## Getting Started

Add `fastcache` to your `Cargo.toml`:

```toml
[dependencies]
fastcache = "0.2.0"
```

## Usage

```rust
use fastcache::Cache;
use std::time::Duration;

let cache = Cache::new(1000, Duration::from_secs(60));
cache.insert("key", "value");

if let Some(value) = cache.get(&"key") {
    println!("Got: {}", value);
}

cache.remove(&"key");
```

## API Reference

### `Cache::new(max_size: usize, ttl: Duration) -> Cache<K, V>`

Create a cache with a maximum number of entries and a time-to-live for each entry.

### `cache.get(key: &K) -> Option<V>`

Retrieve a cached value. Returns `None` if the key is missing or expired.

### `cache.insert(key: K, value: V)`

Insert or update a cache entry. Resets the TTL.

### `cache.remove(key: &K) -> Option<V>`

Remove and return a cached value.

### `cache.len() -> usize`

Number of entries currently in the cache.

### `cache.clear()`

Remove all entries.

## Development

```bash
git clone <repo-url>
cd fastcache
cargo test
```

| Command | Description |
|---------|-------------|
| `cargo test` | Run test suite |
| `cargo bench` | Run Criterion benchmarks |
| `cargo doc --open` | Generate and view API docs |

## Contributing

Contributions are welcome. Fork the repo, create a feature branch, and open a pull request.

## License

[MIT](LICENSE)
