use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use bytes::Bytes;
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub(crate) struct Kv {
    shared: Arc<Mutex<Shared>>,
}

#[derive(Debug, Clone)]
struct Shared {
    data: HashMap<String, Bytes>,
    pub_sub: HashMap<String, broadcast::Sender<Bytes>>,
}

impl Kv {
    pub(crate) fn new() -> Kv {
        Kv {
            shared: Arc::new(Mutex::new(Shared {
                data: HashMap::new(),
                pub_sub: HashMap::new(),
            })),
        }
    }

    pub(crate) fn get(&self, key: String) -> Option<Bytes> {
        let shared = self.shared.lock().unwrap();
        shared.data.get(&key).map(|data| data.clone())
    }

    pub(crate) fn set(&self, key: String, value: Bytes, _expire: Option<Duration>) {
        let mut shared = self.shared.lock().unwrap();
        shared.data.insert(key, value);
    }

    pub(crate) fn subscribe(&self, key: String) -> broadcast::Receiver<Bytes> {
        use std::collections::hash_map::Entry;

        let mut shared = self.shared.lock().unwrap();
        match shared.pub_sub.entry(key) {
            Entry::Occupied(e) => e.get().subscribe(),
            Entry::Vacant(e) => {
                let (tx, rx) = broadcast::channel(1028);
                e.insert(tx);
                rx
            }
        }
    }

    pub(crate) fn publish(&self, key: &str, value: Bytes) -> usize {
        let shared = self.shared.lock().unwrap();

        shared
            .pub_sub
            .get(key)
            .map(|tx| tx.send(value).unwrap_or(0))
            .unwrap_or(0)
    }
}
