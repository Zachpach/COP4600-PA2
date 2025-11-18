use std::collections::LinkedList;
use std::string::String;
use std::sync::Mutex;

struct BucketNode {
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

    fn to_string(&self) -> String {
        return format!(
            "Node [ hash: {}, name: {}, salary: {} ] ",
            self.hash, self.name, self.salary
        );
    }
}

struct Bucket {
    bucket: Mutex<LinkedList<BucketNode>>,
}
impl Bucket {
    fn new() -> Self {
        return Bucket {
            bucket: Mutex::new(LinkedList::new()),
        };
    }
    fn get(&self, p0: usize) -> &mut Bucket {
        todo!()
    }
    fn to_string(&self) -> String {
        let bucket = self.bucket.lock().unwrap();
        let mut s1 = String::from("Bucket: { ");
        for element in bucket.iter() {
            s1.push_str(element.to_string().as_str());
            s1.push_str(", ");
        }

        s1.push_str(" }");
        return s1;
    }
}
struct ConcurrentHashTable {
    buckets: Vec<Bucket>,
}

impl ConcurrentHashTable {
    fn new(size: usize) -> ConcurrentHashTable {
        println!("buckets: {}", size);
        let mut temp:Vec<Bucket> = Vec::with_capacity(size);
        println!("buckets: {}", temp.len());

        for i in 0..size {
            let bucket = Bucket { bucket: Mutex::new(LinkedList::new()), };
            temp.push(bucket)
        }

        return ConcurrentHashTable {
            buckets: temp,
        };
    }
    fn insert(&mut self, bucket: Bucket) {}
    fn delete(&mut self, bucket: &Bucket) {}
    fn search_by_hash(&self, hash: usize) -> &mut Bucket {
        return self.buckets[hash].get(hash);
    }
    fn search_by_name(&self, name: String) -> &mut Bucket {
        let hash = jenkins_one_at_a_time_hash(name.clone());
        return self.search_by_hash(hash);
    }

    fn to_string(&self) -> String {
        let mut s1 = String::from("HashTable: { \n");
        println!("buckets: {}", self.buckets.len());
        for element in self.buckets.iter() {
            println!("1");
            s1.push_str(element.to_string().as_str());
            s1.push_str(", \n");
        }

        s1.push_str(" }");
        return s1;
    }
}

fn main() {
    let table = ConcurrentHashTable::new(10);
    println!("{:?}", table.to_string());
}

fn jenkins_one_at_a_time_hash(key: String) -> usize {
    let mut i: usize = 0;
    let mut hash: usize = 0;
    while i != key.len() {
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
