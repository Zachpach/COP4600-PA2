




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

    fn insert(&mut self, name: String, salary: u32, priority: u32) {

    }

    fn delete(&mut self, name: String, salary: u32, priority: u32) {

    }

    fn update(&mut self, name: String, salary: u32, priority: u32) {

    }
    fn search(&mut self, name: String, salary: u32, priority: u32) {

    }
    fn print(&mut self, name: String, salary: u32, priority: u32) {

    }
}


fn main() {

}





