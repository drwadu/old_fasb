use crate::utils::Facets;
use lru::{DefaultHasher, LruCache};
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    pub static ref CACHE: Arc<Mutex<Cache>> = Arc::new(Mutex::new(Cache::new(1_000)));
}

#[derive(Debug)]
pub struct Cache {
    pub inclusive_facets: LruCache<Vec<i32>, Facets>,
    pub max_as_facets: LruCache<String, Vec<String>>,
    pub min_as_facets: LruCache<String, Vec<String>>,
    pub max_fc_facets: LruCache<String, Vec<String>>,
    pub min_fc_facets: LruCache<String, Vec<String>>,
    pub as_counts: LruCache<String, usize>,
}
impl Cache {
    pub fn new(capacity: usize) -> Self {
        Self {
            inclusive_facets: LruCache::with_hasher(capacity, DefaultHasher::default()),
            max_as_facets: LruCache::with_hasher(capacity, DefaultHasher::default()),
            min_as_facets: LruCache::with_hasher(capacity, DefaultHasher::default()),
            max_fc_facets: LruCache::with_hasher(capacity, DefaultHasher::default()),
            min_fc_facets: LruCache::with_hasher(capacity, DefaultHasher::default()),
            as_counts: LruCache::with_hasher(capacity, DefaultHasher::default()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        Cache::new(1_000);
    }

    #[test]
    fn cache() {
        assert!(CACHE.lock().is_ok());
    }
}
