use std::string::String;
use std::sync::RwLock;

/**
This is the hash table structure given by the assignment
This is, in fact, a linked list, don't ask me I'm just following instruction I've already axed the code once because I made a hash table once
The linked list is will hold an order based on the hash value that will be generated from the name string
Order should be upheld in the insert method
*/
struct hash_struct{
    hash: u32,
    name: String,
    salary: u32,
    next: Option<Box<hash_struct>>,
}

impl hash_struct {
    fn new(hash: u32, name: String, salary: u32) -> Self {
        hash_struct {
            hash,
            name,
            salary,
            next: None,
        }
    }

    fn insert(&mut self, new_hash: hash_struct) {
        match self.next {
            Some(ref mut next_node) => {
                if new_hash.hash < next_node.hash {
                    // Insert between `self` and `next_node`
                    let mut boxed = Box::new(new_hash);
                    boxed.next = self.next.take();
                    self.next = Some(boxed);
                } else {
                    // Recur down the list
                    next_node.insert(new_hash);
                }
            }
            None => {
                // End of list → append
                self.next = Some(Box::new(new_hash));
            }
        }
    }

    fn delete(&mut self, hash: u32) -> bool {
        // Check if next node exists
        if let Some(ref mut next_node) = self.next {
            // If the next node is the one to delete
            if next_node.hash == hash {
                // Remove it by taking ownership of its `next`
                let next_next = next_node.next.take();
                self.next = next_next;
                return true;
            } else {
                // Continue searching further down the list
                return next_node.delete(hash);
            }
        }

        // Reached end of list, nothing deleted
        false
    }


    pub fn update(&mut self, hash: u32, new_salary: u32) -> bool {
        if let Some(node) = self.search(hash) {
            node.salary = new_salary;
            true
        } else {
            false
        }
    }
    pub fn search(&mut self, hash: u32) -> Option<&mut hash_struct> {
        if self.hash == hash {
            return Some(self);
        }

        if let Some(ref mut next) = self.next {
            next.search(hash)
        } else {
            None
        }
    }
    fn print(& self) {
        println!("Hash: {}", self.hash);
        println!("Name: {}", self.name);
        println!("Salary: {}", self.salary);
        println!("---");

        if let Some(ref next) = self.next {
            next.print();
        }
    }
}


// static cv_ordering: u32
static WRITER_LOCK: RwLock<Option<hash_struct>> = RwLock::new(None);
static READER_LOCK: RwLock<Option<hash_struct>> = RwLock::new(None);

fn main() {

    let hash:hash_struct = hash_struct::new(0, String::from("hello"), 0);

}

fn jenkins_one_at_a_time_hash(key: String) -> u32 {
    let mut i: usize = 0;
    let mut hash: u32 = 0;
    while i != key.len(){
        hash += key.as_bytes()[i] as u32;
        hash += hash << 10;
        hash ^= hash >> 6;
        i += 1;
    }
    hash += hash << 3;
    hash ^= hash >> 11;
    hash += hash << 15;
    return hash;
}



fn insert(name: String, salary: u32, priority: u32) {

}

fn delete( name: String, priority: u32) {

}

fn update( name: String, salary: u32, priority: u32) {

}
fn search( name: String, priority: u32) {

}
fn print( priority: u32) {

}



