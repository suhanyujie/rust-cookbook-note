//! 通过 [indexmap](https://github.com/bluss/indexmap) 实现简单的 KV 数据库
//! 为了防止 data race，将 IndexMap 用 Arc 进行包装

use super::util::HandyRwLock;
use indexmap::IndexMap;
use std::sync::{Arc, RwLock};

struct KVStore {
    inner: Arc<RwLock<IndexMap<Vec<u8>, Vec<u8>>>>,
}

impl KVStore {
    fn new() -> Self {
        KVStore::from_map(IndexMap::new())
    }

    fn from_map(m: IndexMap<Vec<u8>, Vec<u8>>) -> Self {
        KVStore {
            inner: Arc::new(RwLock::new(m)),
        }
    }

    fn set(&mut self, k: Vec<u8>, v: Vec<u8>) -> Option<Vec<u8>> {
        self.inner.wl().insert(k, v)
    }

    fn get(&self, k: &[u8]) -> Option<Vec<u8>> {
        self.inner.rl().get(k).map(|v| v.clone())
    }

    fn delete(&mut self, k: &[u8]) -> Option<Vec<u8>> {
        self.inner.wl().remove(k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store1() {
        let mut st = KVStore::new();
        let cache_key: Vec<u8> = "org_1001_info".as_bytes().into();
        st.set(cache_key.clone(), "hello org".as_bytes().into());
        assert_eq!(st.get(&cache_key), Some("hello org".as_bytes().into()));
        assert!(false);
    }

    #[test]
    fn test_store_delete() {
        let mut st = KVStore::new();
        let cache_key: Vec<u8> = "org_1001_info".as_bytes().into();
        st.set(cache_key.clone(), "hello org".as_bytes().into());
        assert_eq!(st.delete(&cache_key), Some("hello org".as_bytes().into()));
        assert_eq!(st.get(&cache_key), None);
    }
}
