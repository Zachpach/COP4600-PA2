use std::collections::LinkedList;
use std::sync::Mutex;
use std::string::String;

struct BucketNode<> {
    hash: usize,
    name: String,
    salary: u32,
}
impl BucketNode {
    fn new(name: String, salary: u32) -> Self {
        BucketNode {
            hash: jenkins_one_at_a_time_hash(name.clone()),
            name,
            salary,
        }
    }

    fn equals(&self, other: &BucketNode) -> bool {
        return self.hash == other.hash && self.name == other.name;
    }
}

struct Bucket {
    bucket: Mutex<LinkedList<BucketNode>>,
}
impl Bucket {
    fn new() -> Self {
        return Bucket{
            bucket: Mutex::new(LinkedList::new()),
        };
    }
    fn get(&self, p0: usize) -> &mut Bucket {
        todo!()
    }
}
struct ConcurrentHashTable {
    buckets: Vec<Bucket>,
}

impl ConcurrentHashTable {
    fn new(size: usize) -> ConcurrentHashTable {
        return ConcurrentHashTable {
            buckets: Vec::with_capacity(size),
        };
    }
    fn insert(&mut self, bucket: Bucket) {

    }
    fn delete(&mut self, bucket: &Bucket) {

    }
    fn search_by_hash(&self, hash: usize) -> &mut Bucket {

        return self.buckets[hash].get(hash);
    }
    fn search_by_name(&self, name: String) -> &mut Bucket {
        let hash = jenkins_one_at_a_time_hash(name.clone());
        return self.search_by_hash(hash);
    }
}

fn main() {



}

fn jenkins_one_at_a_time_hash(key: String) -> usize {
    let mut i: usize = 0;
    let mut hash: usize = 0;
    while (i != key.len()) {
        hash += key.as_bytes()[i] as usize;
        hash += hash << 10;
        hash ^= hash >> 6;
        i += 1;
    }
    hash += hash << 3;
    hash ^= hash >> 11;
    hash += hash << 15;
    return hash;
}

fn update_salary() {}
