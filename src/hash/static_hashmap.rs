use crate::hash::mod_hash::mod_hash;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::from(self.to_be_bytes())
    }
}


pub struct StaticHashMap<K: ToBytes, V> {
    size: u32,
    heads: Vec<Option<Vec<(K, V)>>>,
}

impl<K: ToBytes, V> StaticHashMap<K, V> {
    pub fn new(size: u32) -> Self {
        let mut heads = Vec::with_capacity(size as usize);
        for _ in 0..size {
            heads.push(None);
        }
        Self {
            size,
            heads,
        }
    }

    pub fn hash(&self, key: &K) -> usize {
        mod_hash(key.to_bytes(), self.size) as usize
    }

    pub fn put(&mut self, key: K, value: V) {
        let hash = self.hash(&key);
        match &mut self.heads[hash] {
            None => {
                let mut vec = Vec::new();
                vec.push((key, value));
                self.heads[hash] = Some(vec);
            }
            Some(vec) => {
                if let Some(pair) = vec.iter().enumerate().find(|pair| pair.1.0.to_bytes() == key.to_bytes()) {
                    vec.remove(pair.0);
                }
                vec.push((key, value));
            }
        };
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        let hash = self.hash(&key);
        if let Some(vec) = &self.heads[hash] {
            vec.iter().find(|pair| pair.0.to_bytes() == key.to_bytes()).map(|pair| &pair.1)
        } else {
            None
        }
    }
}

#[test]
pub fn static_hashmap_test() {
    let mut map = StaticHashMap::new(101);
    map.put(10, "Hello, World!");
    map.put(20, "Bye!");
    assert_eq!(map.get(10), Some(&"Hello, World!"));
    assert_eq!(map.get(20), Some(&"Bye!"));
    assert_eq!(map.get(30), None);
}