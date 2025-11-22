use std::fs::File;
use std::io::{self, BufRead};
use std::string::String;
use std::sync::{Arc, RwLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

static mut OUTPUT_BUFF: Option<Mutex<Vec<String>>> = None;
/**
This is the hash table structure given by the assignment
This is, in fact, a linked list, don't ask me I'm just following instruction I've already axed the code once because I made a hash table once
The linked list is will hold an order based on the hash value that will be generated from the name string
Order should be upheld in the insert method
*/
struct hash_struct {
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

    fn insert(&mut self, new_hash: hash_struct) -> bool {
        match self.next {
            Some(ref mut next_node) => {
                if new_hash.hash < next_node.hash {
                    // Insert between `self` and `next_node`
                    let mut boxed = Box::new(new_hash);
                    boxed.next = self.next.take();
                    self.next = Some(boxed);
                    return true;
                } else if new_hash.hash == next_node.hash {
                    return false;
                } else {
                    // Recur down the list
                    return next_node.insert(new_hash);
                }
            }
            None => {
                // End of list → append
                self.next = Some(Box::new(new_hash));
                return true;
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
        if self.hash == hash {
            self.salary = new_salary;
            return true;
        }

        if let Some(ref mut next) = self.next {
            next.update(hash, new_salary)
        } else {
            false
        }
    }

    pub fn search(&self, hash: u32) -> Option<&hash_struct> {
        if self.hash == hash {
            return Some(self);
        }

        if let Some(ref next) = self.next {
            next.search(hash)
        } else {
            None
        }
    }
    fn print(&self) {
        if let Some(ref next) = self.next {
            println!("{}", next.to_string());
            next.print();
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}, {}, {}", self.hash, self.name, self.salary);
    }
}

//When called, pass the thread number and the event string
pub fn log_event(event: String) {
    let timestamp = current_timestamp();
    unsafe {
        let mutex = OUTPUT_BUFF.as_ref().expect("Global string vector must be initialized.");
        
        // Lock the Mutex to gain exclusive access
        let mut data = mutex.lock().unwrap(); 
        
        println!("-> Adding '{}' to the global vector.", event);
        // Convert the string slice to an owned String before pushing
        data.push(format!("{}: THREAD {} ", timestamp, event.to_string()));
    }
}

struct HashStructWrapper {
    // head: hash_struct,
    head: RwLock<hash_struct>,
}
impl HashStructWrapper {
    pub fn new() -> Self {
        return HashStructWrapper {
            head: RwLock::new(hash_struct::new(0, "head".parse().unwrap(), 0)),
        };
    }
}

fn current_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

fn parse_thread_command(line: String) -> Option<usize> {
    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
    if parts.len() >= 2 && parts[0].to_lowercase() == "threads" {
        parts[1].parse::<usize>().ok()
    } else {
        None
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
            if parts.len() != 4 {
                eprintln!("Invalid format for DELETE: {}", line);
                return None;
            }

            let priority = match parts[3].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid priority in DELETE: {}", line);
                    return None;
                }
            };

            Some((command, parts[1].clone(), 0, priority))
        }

        "update" => {
            if parts.len() != 4 {
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
            if parts.len() != 4 {
                eprintln!("Invalid format for SEARCH: {}", line);
                return None;
            }

            let priority = match parts[3].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid priority in SEARCH: {}", line);
                    return None;
                }
            };

            Some((command, parts[1].clone(), 0, priority))
        }

        "print" => {
            if parts.len() != 4 {
                eprintln!("Invalid format for PRINT: {}", line);
                return None;
            }

            let priority = match parts[3].parse::<u32>() {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {

    unsafe {
        OUTPUT_BUFF = Some(Mutex::new(Vec::new()));
    }
    
    let file = File::open("commands.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut commands: Vec<(String, String, u32, u32)> = vec![];
    let mut threads = vec![];
    let hash_struct = Arc::new(HashStructWrapper::new());

    // test_elements(&hash_struct);

    // read file and place commands into the commands array
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    
    let max_threads = match parse_thread_command(first_line.trim().to_string()) {
        Some(n) if n > 0 => n,
        _ => {
            eprintln!("Failed to parse thread configuration or N is 0. Defaulting to 1.");
            1
        }
    };
    println!("Max concurrent threads set to: {}", max_threads);
    //  can get max num of threads from the max_threads var and seperately roll out the commands


    for line in reader.lines() {
        let line = line?;
        if let Some(parsed) = parse_line(line) {
            commands.push(parsed);
        }
    }


    for command in commands {
        let hs_clone = Arc::clone(&hash_struct);

        let current_thread = std::thread::spawn(move || {
            thread_op(&hs_clone, "command".to_string());
        });

        threads.push(current_thread)
    }

    // shut down threads
    for thread in threads {
        thread.join().unwrap();
    }


    Ok(())
}

fn jenkins_one_at_a_time_hash(key: String) -> u32 {
    let mut i: usize = 0;
    let mut hash: u32 = 0;
    while i != key.len() {
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

fn thread_op(hash_structure: &HashStructWrapper, command: String) {
    parse_line(command.clone());

    // control ordering run command once it the correct time
    println!("Thread Command: {}", command);

}

fn insert(hash_structure: &HashStructWrapper, name: String, salary: u32, priority: u32) {}

fn delete(hash_structure: &HashStructWrapper, name: String, priority: u32) {}

fn update(hash_structure: &HashStructWrapper, name: String, salary: u32, priority: u32) {}

fn search(hash_structure: &HashStructWrapper, name: String, priority: u32) {}
fn print(hash_structure: &HashStructWrapper, priority: u32) {}

fn test_elements(hash_struct_wrapper: &HashStructWrapper) {
    hash_struct_wrapper
        .head
        .write()
        .unwrap()
        .insert(hash_struct::new(5, "dave".parse().unwrap(), 23000));
    hash_struct_wrapper
        .head
        .write()
        .unwrap()
        .insert(hash_struct::new(3, "molly".parse().unwrap(), 45000));
    hash_struct_wrapper
        .head
        .write()
        .unwrap()
        .insert(hash_struct::new(6, "albert".parse().unwrap(), 15000));
    hash_struct_wrapper
        .head
        .write()
        .unwrap()
        .insert(hash_struct::new(5, "nick".parse().unwrap(), 68000));
    hash_struct_wrapper
        .head
        .write()
        .unwrap()
        .insert(hash_struct::new(13, "bob".parse().unwrap(), 41000));
    hash_struct_wrapper.head.read().unwrap().print();
    {
        let mut l = hash_struct_wrapper.head.read().unwrap();
        let s = l.search(6).unwrap().to_string();
        println!("found: {}", s.to_string());
    }
    hash_struct_wrapper.head.write().unwrap().delete(6);
    hash_struct_wrapper.head.write().unwrap().update(3, 53000);
    hash_struct_wrapper.head.read().unwrap().print();
}
