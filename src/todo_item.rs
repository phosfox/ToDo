pub struct TodoItem {
    pub name: String,
    pub done: bool, 
}

impl TodoItem {
    pub fn new(name: String, done: bool) -> TodoItem {
        TodoItem { name, done }
    }
 
    pub fn change_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn complete(&mut self) {
        self.done = true;
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() {
        let name = String::from("test");
        let done = false;

        let item = TodoItem::new(name.clone(), done);
        let got_name = item.name;
        let got_done = item.done;

        assert_eq!(name, got_name);
        assert_eq!(done, got_done);
    }

    #[test]
    fn test_complete() {
        let name = String::from("test");
        let done = false;
        let mut item = TodoItem::new(name.clone(), done);
        item.complete();
        assert_eq!(item.done, true);
    }

    #[test]
    fn test_change_name() {
        let name = String::from("test");
        let done = false;
        let mut item = TodoItem::new(name.clone(), done);
        item.change_name(String::from("changed"));
        assert_eq!(item.name, "changed");
    }
}
