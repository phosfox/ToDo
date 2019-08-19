use crate::todo_item::TodoItem;

pub fn write_todos(todos: &Vec<TodoItem>, mut to: impl std::io::Write){
    for (i, todo) in todos.iter().enumerate() {
        writeln!(to, "{},{},{}", i+1, todo.name, todo.done).unwrap();
    }
}

pub fn todos_to_csv(todos: &Vec<TodoItem>) -> String {
    let mut todo_as_strings = Vec::new();
    for (i, todo) in todos.iter().enumerate() {
        todo_as_strings.push(format!("{},{},{}", i+1, todo.name, todo.done));
    }
    return todo_as_strings.join("\n");
}

pub fn print_todos(todos: &Vec<TodoItem>) {
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: {}", i+1, todo.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todos_to_csv() {
        let todo = TodoItem::new(String::from("test"), false);
        let todo2 = TodoItem::new(String::from("test2"), true);
        let mut todos: Vec<TodoItem> = vec![];
        todos.push(todo);
        todos.push(todo2);

        let csv = todos_to_csv(&todos);
        assert_eq!(csv, "1,test,false\n2,test2,true")
    }
    #[test]
    fn test_write_todos(){
        let todo = TodoItem::new(String::from("test"), false);
        let todo2 = TodoItem::new(String::from("test2"), true);
        let mut todos: Vec<TodoItem> = vec![];
        todos.push(todo);
        todos.push(todo2);

        let mut result = Vec::new();
        write_todos(&todos, &mut result);
        assert_eq!(result, b"1,test,false\n2,test2,true\n");
    }
}
