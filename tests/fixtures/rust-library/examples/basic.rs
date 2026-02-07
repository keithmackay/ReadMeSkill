use fastcache::Cache;
use std::time::Duration;

fn main() {
    let cache = Cache::new(1000, Duration::from_secs(60));
    cache.insert("key", "value");
    println!("{:?}", cache.get(&"key"));
}
