extern crate core;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::string::String;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/*
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
                    println!(
                        "Inserted {},{},{}",
                        new_hash.hash, new_hash.name, new_hash.salary
                    );
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
                println!(
                    "Inserted {},{},{}",
                    new_hash.hash, new_hash.name, new_hash.salary
                );
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
                println!("Deleted record for {}", next_node.to_string());
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
            let before = self.to_string();
            self.salary = new_salary;
            let after = self.to_string();

            println!("Updated record {} from {} to {}", self.hash, before, after);

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
    fn print(&self) -> String{
        let mut output = String::new();
        if let Some(ref next) = self.next {
            output.push_str(&next.to_string());
            output.push_str("\n");
            output.push_str(&next.print());
        }
        output
    }

    pub fn to_string(&self) -> String {
        return format!("{},{},{}", self.hash, self.name, self.salary);
    }
}

struct HashStructWrapper {
    // head: hash_struct,
    OUTPUT_BUFF: Option<Mutex<Vec<String>>>,
    head: RwLock<hash_struct>,
    // cv: global lock for printing for log and waiting
    lock: Mutex<u32>,
    cvar: Condvar,
}
impl HashStructWrapper {
    pub fn new() -> Self {
        return HashStructWrapper {
            OUTPUT_BUFF: Some(Mutex::new(Vec::new())),
            head: RwLock::new(hash_struct::new(0, "head".parse().unwrap(), 0)),
            lock: Mutex::new(0),
            cvar: Condvar::new(),
        };
    }

    //When called, pass the thread number and the event string
    fn log_event(&self, event: String) {
        let timestamp = current_timestamp();
        let mutex = self.OUTPUT_BUFF.as_ref().unwrap();

        // Lock the Mutex to gain exclusive access
        let mut data = mutex.lock().unwrap();

        // Convert the string slice to an owned String before pushing
        data.push(format!("{}: THREAD {} ", timestamp, event.to_string()));
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

            let priority = match parts[3].parse::<u32>() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Invalid priority in UPDATE: {}", line);
                    return None;
                }
            };

            Some((command, parts[1].clone(), salary, priority))
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

fn write_log_to_file(hash_structure: &HashStructWrapper) -> Result<(), Box<dyn std::error::Error>> {
    let mutex = hash_structure.OUTPUT_BUFF.as_ref().unwrap();
    let data = mutex.lock().unwrap();

    let mut file = File::create("hash.log")?;
    for entry in data.iter() {
        writeln!(file, "{}", entry)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("commands.txt")?;
    let mut reader = io::BufReader::new(file);
    let mut threads = vec![];
    let hash_struct = Arc::new(HashStructWrapper::new());

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

    for command in reader.lines() {
        let hs_clone = Arc::clone(&hash_struct);

        let current_thread = std::thread::spawn(move || {
            thread_op(&hs_clone, command.unwrap().to_string());
        });

        threads.push(current_thread)
    }

    // shut down threads
    for thread in threads {
        thread.join().unwrap();
    }

    // write log to file
    write_log_to_file(&hash_struct)?;
    hash_struct.log_event("Final Table:".to_string());
    hash_struct.log_event(hash_struct.head.read().unwrap().print());

    Ok(())
}

fn jenkins_one_at_a_time_hash(key: String) -> u32 {
    let mut i: usize = 0;
    let mut hash: u32 = 0;
    while i != key.len() {
        hash = hash.wrapping_add(key.as_bytes()[i] as u32);
        hash = hash.wrapping_add(hash << 10);
        hash ^= hash >> 6;
        i += 1;
    }
    hash = hash.wrapping_add(hash << 3);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 15);
    return hash;
}

fn thread_op(hash_structure: &HashStructWrapper, command: String) {
    let parse = parse_line(command.clone());

    if parse.is_none() {
        return;
    }

    let parse = parse.unwrap();

    // control ordering run command once it the correct time
    // println!("Thread Command: {}", command);

    match parse.0.as_str() {
        "insert" => insert(hash_structure, parse.1, parse.2, parse.3),
        "delete" => delete(hash_structure, parse.1, parse.3),
        "update" => update(hash_structure, parse.1, parse.2, parse.3),
        "search" => search(hash_structure, parse.1, parse.3),
        "print" => print(hash_structure, parse.3),
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn insert(hash_structure: &HashStructWrapper, name: String, salary: u32, priority: u32) {
    // Wait for the thread to start up.
    //get information on lock
    let mut can_start = hash_structure.lock.lock().unwrap();
    let hash = jenkins_one_at_a_time_hash(name.clone());

    //if the lock isn't acquired, or this thread hasn't started, wait
    // As long as the value inside the `Mutex<bool>` is `false`, we wait.

    //Thread wait loop
    hash_structure.log_event(format!("{} WAITING FOR MY TURN", priority.clone()));
    while *can_start != priority {
        can_start = hash_structure.cvar.wait(can_start).unwrap();
    }
    //Thread awakened
    hash_structure.log_event(format!("{} AWAKENED FOR WORK", priority.clone()));

    //Write lock acquired
    hash_structure.log_event(format!("{} INSERT,{},{},{}", priority.clone(), hash, name, salary));
    hash_structure.log_event(format!("{} WRITE LOCK ACQUIRED", priority.clone()));
    hash_structure
        .head
        .write()
        .unwrap()
        .insert(hash_struct::new(hash, name.clone(), salary));

    //Write lock released
    hash_structure.log_event(format!("{} WRITE LOCK RELEASED", priority.clone()));

    // let mut can_start = hash_structure.lock.lock().unwrap();
    *can_start += 1;
    hash_structure.cvar.notify_all();
}

fn delete(hash_structure: &HashStructWrapper, name: String, priority: u32) {
    // Wait for the thread to start up.
    //get information on lock
    let mut can_start = hash_structure.lock.lock().unwrap();
    let hash = jenkins_one_at_a_time_hash(name.clone());

    //if the lock isn't acquired, or this thread hasn't started, wait
    // As long as the value inside the `Mutex<bool>` is `false`, we wait.
    //Thread wait loop
    hash_structure.log_event(format!("{} WAITING FOR MY TURN", priority.clone()));
    while *can_start != priority {
        can_start = hash_structure.cvar.wait(can_start).unwrap();
    }
    //Thread awakened
    hash_structure.log_event(format!("{} AWAKENED FOR WORK", priority.clone()));
    hash_structure.log_event(format!("{} DELETE,{},{}", priority.clone(), hash, name));
    //Write lock acquired
    hash_structure.log_event(format!("{} WRITE LOCK ACQUIRED", priority.clone()));

    if !hash_structure.head.write().unwrap().delete(hash) {
        println!("{} not found.", name);
    }

    //Write lock released
    hash_structure.log_event(format!("{} WRITE LOCK RELEASED", priority.clone()));

    // let mut can_start = hash_structure.lock.lock().unwrap();

    *can_start += 1;
    hash_structure.cvar.notify_all();
}

fn update(hash_structure: &HashStructWrapper, name: String, salary: u32, priority: u32) {
    // Wait for the thread to start up.
    //get information on lock
    let mut can_start = hash_structure.lock.lock().unwrap();
    let hash = jenkins_one_at_a_time_hash(name.clone());

    //if the lock isn't acquired, or this thread hasn't started, wait
    // As long as the value inside the `Mutex<bool>` is `false`, we wait.
    //Thread wait loop
    hash_structure.log_event(format!("{} WAITING FOR MY TURN", priority.clone()));
    while *can_start != priority {
        can_start = hash_structure.cvar.wait(can_start).unwrap();
    }
    //Thread awakened
    hash_structure.log_event(format!("{} AWAKENED FOR WORK", priority.clone()));
    hash_structure.log_event(format!("{} UPDATE,{},{},{}", priority.clone(), hash, name, salary));
    //Write lock acquired
    hash_structure.log_event(format!("{} WRITE LOCK ACQUIRED", priority.clone()));
    if !hash_structure.head.write().unwrap().update(hash, salary) {
        println!("Update failed. Entry {} not found.", hash);
    }

    //Write lock released
    hash_structure.log_event(format!("{} WRITE LOCK RELEASED", priority.clone()));

    // let mut can_start = hash_structure.lock.lock().unwrap();
    *can_start += 1;
    hash_structure.cvar.notify_all();
}

fn search(hash_structure: &HashStructWrapper, name: String, priority: u32) {
    // Wait for the thread to start up.
    //get information on lock
    let mut can_start = hash_structure.lock.lock().unwrap();
    let hash = jenkins_one_at_a_time_hash(name.clone());

    //if the lock isn't acquired, or this thread hasn't started, wait
    // As long as the value inside the `Mutex<bool>` is `false`, we wait.
    //Thread wait loop
    hash_structure.log_event(format!("{} WAITING FOR MY TURN", priority.clone()));
    while *can_start != priority {
        can_start = hash_structure.cvar.wait(can_start).unwrap();
    }
    hash_structure.log_event(format!("{} SEARCH,{},{}", priority.clone(), hash, name));
    //Thread awakened
    hash_structure.log_event(format!("{} AWAKENED FOR WORK", priority.clone()));

    //Read lock acquired
    hash_structure.log_event(format!("{} READ LOCK ACQUIRED", priority.clone()));
    let mut l = hash_structure.head.read().unwrap();
    let s = l.search(hash);

    if s.is_some() {
        println!("Found: {}", s.unwrap().to_string());
    } else {
        println!("{} not found.", name.to_string());
    }

    //Read lock released
    hash_structure.log_event(format!("{} READ LOCK RELEASED", priority.clone()));
    // let mut can_start = hash_structure.lock.lock().unwrap();

    *can_start += 1;
    hash_structure.cvar.notify_all();
}
fn print(hash_structure: &HashStructWrapper, priority: u32) {
    // Wait for the thread to start up.
    //get information on lock
    let mut can_start = hash_structure.lock.lock().unwrap();

    //if the lock isn't acquired, or this thread hasn't started, wait
    // As long as the value inside the `Mutex<bool>` is `false`, we wait.

    //Thread wait loop
    hash_structure.log_event(format!("{} WAITING FOR MY TURN", priority.clone()));
    while *can_start != priority {
        can_start = hash_structure.cvar.wait(can_start).unwrap();
    }
    //Thread awakened
    hash_structure.log_event(format!("{} AWAKENED FOR WORK", priority.clone()));
    hash_structure.log_event(format!("{} PRINT", priority.clone()));
    //Read lock acquired
    hash_structure.log_event(format!("{} READ LOCK ACQUIRED", priority.clone()));
    println!("Current Database:");
    print!("{}", hash_structure.head.read().unwrap().print());
    // let mut can_start = hash_structure.lock.lock().unwrap();
    //Read lock released
    hash_structure.log_event(format!("{} READ LOCK RELEASED", priority.clone()));

    *can_start += 1;
    hash_structure.cvar.notify_all();
}
