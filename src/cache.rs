use lru::{DefaultHasher, LruCache};
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    pub static ref CACHE: Arc<Mutex<Cache>> = Arc::new(Mutex::new(Cache::new(1_000)));
}

#[derive(Debug)]
pub struct Cache {
    pub answer_set_counts: LruCache<String, usize>,
    pub max_fc_facets: LruCache<String, Vec<String>>,
    pub min_fc_facets: LruCache<String, Vec<String>>,
}
impl Cache {
    pub fn new(capacity: usize) -> Self {
        Cache {
            answer_set_counts: LruCache::with_hasher(capacity, DefaultHasher::default()),
            max_fc_facets: LruCache::with_hasher(capacity, DefaultHasher::default()),
            min_fc_facets: LruCache::with_hasher(capacity, DefaultHasher::default()),
        }
    }
}
