use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn insert (&mut self, key: String, value: bool) {
        self.map.insert(key, value);
    }

    fn complete (&mut self, task: String) {
        if let Some(value) = self.map.get_mut(&task) {
            *value = true;
        }

    }

    fn get (self) -> HashMap<String, bool> {
        self.map
    }

    fn done(&mut self, key: String) {
        self.map.remove(&key);
    }
}

fn main() {
    let mut todo = Todo {
        map: HashMap::new()
    };

    todo.insert("test".to_string(), false);
    todo.insert(String::from("Hellow"), true);

    todo.complete("test".to_string());
    todo.done("Hellow".to_string());
    let todos = todo.get();

    for (key, status) in  todos {
        println!("{}\t{}\n", key, if status {  "String" } else { "No" });
    }

    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

}

