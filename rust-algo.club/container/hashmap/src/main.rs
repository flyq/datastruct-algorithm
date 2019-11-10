use std::hash::{Hash, Hasher};


fn main() {
    println!("Hello, world!");
}

struct Car {
  brand: String,
}

impl Hash for Car {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.brand.hash(state);
    }
}

fn make_hash<X>(x: &X, len: usize) -> Option<usize>
    where X: Hash + ?Sized,                   // 1
{
    if len == 0 { return None; }              // 2
    let mut hasher = DefaultHasher::new();    // 3
    x.hash(&mut hasher);
    Some(hasher.finish() as usize % len)
}

pub struct HashMap<K, V> where K: Hash + Eq {
    buckets: Vec<(K, V)>,
}

type Bucket<K, V> = Vec<(K, V)>;              // 1

pub struct HashMap<K, V> where K: Hash + Eq {
   buckets: Vec<Bucket<K, V>>,                // 2
   len: usize,                                // 3
}


impl<K, V> HashMap<K, V> where K: Hash + Eq {
    pub fn new() -> Self {
        Default::default()
    }
    /// ...
}

impl<K, V> Default for HashMap<K, V> 
    where K: Hash + Eq 
{
    fn default() -> Self { 
        Self { buckets: Vec::<Bucket<K, V>>::new(), len: 0 }
    }
}

pub fn with_capacity(cap: usize) -> Self {
    let mut buckets: Vec<Bucket<K, V>> =  Vec::new();
    for _ in 0..cap {
        buckets.push(Bucket::new());
    }
    Self { buckets, len: 0 }
}

pub fn get(&self, key: &K) -> Option<&V> {
    let index = self.make_hash(key)?;
    self.buckets.get(index).and_then(|bucket|
        bucket.iter()
            .find(|(k, _)| *k == *key)
            .map(|(_, v)| v)
    )
}

pub fn get<Q>(&self, q: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized
{
    let index = self.make_hash(q)?;
    self.buckets.get(index).and_then(|bucket|
        bucket.iter()
            .find(|(k, _)| q == k.borrow())
            .map(|(_, v)| v)
    )
}

pub fn remove<Q>(&mut self, q: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized
{
    let index = self.make_hash(q)?;                     // 1
    self.buckets.get_mut(index).and_then(|bucket| {     // 2
        bucket.iter_mut()
            .position(|(k, _)| q == (*k).borrow())
            .map(|index| bucket.swap_remove(index).1)
    }).map(|v| {                                        // 3
        self.len -= 1; // Length decreases by one.
        v
    })
}


pub fn insert(&mut self, key: K, value: V) -> Option<V> {
    self.try_resize();                                      // 1
    let index = self                                        // 2
        .make_hash(&key)
        .expect("Failed to make a hash while insertion");
    let bucket = self.buckets
        .get_mut(index)
        .expect(&format!("Failed to get bucket[{}] while insetion", index));
    match bucket.iter_mut().find(|(k, _)| *k == key) {      // 3
        Some((_ , v)) =>  Some(mem::replace(v, value)),     // 3.1
        None => {
            bucket.push((key , value));                     // 3.2
            self.len += 1;
            None
        }
    }                                                       // 4
}

fn try_resize(&mut self) {
    let entry_count = self.len();                               // 1
    let capacity = self.bucket_count();

    // Initialization.
    if capacity == 0 {                                          // 2
        self.buckets.push(Bucket::new());
        return
    }

    if entry_count as f64 / capacity as f64 > LOAD_FACTOR {     // 3
        // Resize. Rehash. Reallocate!
        let mut new_map = Self::with_capacity(capacity << 1);   // 4
        self.buckets.iter_mut()                                 // 5
            .flat_map(|bucket| mem::replace(bucket, vec![]))
            .for_each(|(k, v)| { new_map.insert(k, v); });
        *self = new_map;                                        // 6
    }
}


fn iter() -> std::slice::Iter<(&k, &v)> {
    self.buckets.iter_mut()
        .flat_map(|b| b)
        .map(|(k, v)| (k, v))
}

pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
    self.buckets.iter()
        .flat_map(|b| b)
        .map(|(k, v)| (k, v))
}
