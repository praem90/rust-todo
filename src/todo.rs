use std::collections::HashMap

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn insert (&mut self, key: String) {
        self.map.add(key, true);
    }

    fn complete (&mut self, key: String) {

    }

    fn get (&mut self) {
        self.map
    }
}
