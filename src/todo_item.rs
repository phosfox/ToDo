mod {
    pub struct TodoItem {
    pub name: String,
    pub done: bool, 
}

impl TodoItem {
    pub fn new(name: String, done: bool) -> TodoItem {
        TodoItem { name: name, done: done }
    }
 
    pub fn change_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn complete(&mut self) {
        self.done = true;
    }
}
}
