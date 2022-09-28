#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    capacity: usize,
    keys: HashMap<K, V>,
    priority: HashMap<K, usize>,
    tree: BTreeMap<usize, K>,
    cur_priority: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Clone + Hash + Ord,
{
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("haha! capacity makes brrrrrrrr")
        }
        Self {
            capacity: (capacity),
            keys: (HashMap::new()),
            priority: (HashMap::new()),
            cur_priority: (0),
            tree: (BTreeMap::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.priority.len()
    }

    pub fn is_empty(&self) -> bool {
        self.priority.is_empty()
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.keys.get(key)?;

        self.upd_priority(key.clone());
        Some(self.keys.get(key).unwrap())
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.upd_priority(key.clone());
        self.keys.get_mut(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let upd_key = self.keys.get_mut(&key);

        if upd_key.is_some() {
            self.upd_priority(key.clone());
            self.keys.insert(key, value)
        } else {
            if self.keys.len() < self.capacity {
                self.keys.insert(key.clone(), value);
                self.priority.insert(key.clone(), self.cur_priority);
            } else {
                let lru = self.tree.keys().next().unwrap();
                let old_key = self.tree.remove(&lru.clone()).unwrap();
                self.keys.remove(&old_key);
                self.priority.remove(&old_key);
                self.keys.insert(key.clone(), value);
                self.priority.insert(key.clone(), self.cur_priority);
            }
            self.tree.insert(self.cur_priority, key.clone());
            self.cur_priority += 1;

            None
        }
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.priority.clear();
        self.tree.clear();
        self.cur_priority = 0;
    }

    pub fn upd_priority(&mut self, key: K) {
        let cur_key_priority = self.priority.get(&key);

        if cur_key_priority.is_none() {
            return;
        }

        self.tree.remove(cur_key_priority.unwrap());
        self.tree.insert(self.cur_priority, key.clone());

        let upd_priority = self.priority.entry(&key).and_modify(|x| * x = self.cur_priority);
        self.cur_priority += 1;
    }
}
