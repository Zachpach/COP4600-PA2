use std::string::String;
use std::sync::RwLock;
use std::fs::File;
use std::io::{self, BufRead};

/**
This is the hash table structure given by the assignment
This is, in fact, a linked list, don't ask me I'm just following instruction I've already axed the code once because I made a hash table once
The linked list is will hold an order based on the hash value that will be generated from the name string
Order should be upheld in the insert method
*/
struct hash_struct{
    hash: u32,
    name: String, // Key
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

fn parse_line(line: String) -> Option<(String, String, u32, u32)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Split and trim fields
    let parts: Vec<String> = trimmed.split(',').map(|s| s.trim().to_string()).collect();

    if parts.is_empty() {
        eprintln!("Empty command: {}", line);
        return None;
    }

    // normalize command
    let command = parts[0].to_lowercase();

    match command.as_str() {
        "insert" => {
            if parts.len() != 4 {
                eprintln!("Invalid format for INSERT: {}", line);
                return None;
            }

            let salary = match parts[2].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid salary in INSERT: {}", line);
                    return None;
                }
            };

            let priority = match parts[3].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid priority in INSERT: {}", line);
                    return None;
                }
            };

            Some((command, parts[1].clone(), salary, priority))
        }

        "delete" => {
            if parts.len() != 3 {
                eprintln!("Invalid format for DELETE: {}", line);
                return None;
            }

            let priority = match parts[2].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid priority in DELETE: {}", line);
                    return None;
                }
            };

            Some((command, parts[1].clone(), 0, priority))
        }

        "update" => {
            if parts.len() != 3 {
                eprintln!("Invalid format for UPDATE: {}", line);
                return None;
            }

            let salary = match parts[2].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid salary in UPDATE: {}", line);
                    return None;
                }
            };

            Some((command, parts[1].clone(), salary, 0))
        }

        "search" => {
            if parts.len() != 3 {
                eprintln!("Invalid format for SEARCH: {}", line);
                return None;
            }

            let priority = match parts[2].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid priority in SEARCH: {}", line);
                    return None;
                }
            };

            Some((command, parts[1].clone(), 0, priority))
        }

        "print" => {
            if parts.len() != 2 {
                eprintln!("Invalid format for PRINT: {}", line);
                return None;
            }

            let priority = match parts[1].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid priority in PRINT: {}", line);
                    return None;
                }
            };

            Some((command, String::new(), 0, priority))
        }

        _ => {
            eprintln!("Unknown command: {}", command);
            None
        }
    }
}

// static cv_ordering: u32
static WRITER_LOCK: RwLock<Option<hash_struct>> = RwLock::new(None);
static READER_LOCK: RwLock<Option<hash_struct>> = RwLock::new(None);

fn main() {
    let file = File::open("commands.txt")?;
    let reader = io::BufReader::new(file);

    let hash:hash_struct = hash_struct::new(0, String::from("hello"), 0);
    
    /*  for i in 0..input.len() {
            let out: Option<(String, String, u32, u32)> = parse_line(input[i].clone());
        println!("Line{}: {:?}", i, out);
    }*/
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



