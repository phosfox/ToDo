use crate::todo_item::TodoItem;

pub fn write_todos(todos: Vec<TodoItem>, mut to: impl std::io::Write){
    for (i, todo) in todos.iter().enumerate() {
        writeln!(to, "{},{},{}", i+1, todo.name, todo.done).unwrap();
    }
}

pub fn print_todos(todos: Vec<TodoItem>) {
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: {}", i+1, todo.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_todos(){
        let todo = TodoItem::new(String::from("test"), false);
        let todo2 = TodoItem::new(String::from("test2"), true);
        let mut todos: Vec<TodoItem> = vec![];
        todos.push(todo);
        todos.push(todo2);

        let mut result = Vec::new();
        write_todos(todos, &mut result);
        assert_eq!(result, b"1,test,false\n2,test2,true\n");
    }
}
