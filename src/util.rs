use crate::todo_item::TodoItem;

pub fn write_todos(todos: Vec<TodoItem>, mut to: impl std::io::Write){
    for todo in todos.iter() {
        writeln!(to, "{},{}", todo.name, todo.done).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_todos(){
        let todo = TodoItem::new(String::from("test"), false);
        let mut todos: Vec<TodoItem> = vec![];
        todos.push(todo);

        let mut result = Vec::new();
        write_todos(todos, &mut result);
        assert_eq!(result, b"test,false\n");
    }
}
