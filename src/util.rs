extern crate csv;

use crate::todo_item::TodoItem;
use std::io::Error;

pub fn todos_to_csv(todos: &Vec<TodoItem>) -> String {
    let mut todo_as_strings = Vec::new();
    for todo in todos.iter() {
        todo_as_strings.push(format!("{},{}", todo.name, todo.done));
    }
    return todo_as_strings.join("\n");
}

pub fn print_todos(todos: &Vec<TodoItem>) {
    for (i, todo) in todos.iter().enumerate() {
        println!("{}: {}", i+1, todo.to_string());
    }
}

pub fn parse_csv(path: &std::path::Path) -> Result<Vec<TodoItem>, Error> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    let mut todos: Vec<TodoItem> = vec![];

    for result in reader.records() {
        let record = result?;

        let name: String = record[0].to_string();
        let done: bool =  record[1].parse().expect("could not parse input");

        todos.push(TodoItem::new(name, done));        
    }
    return Ok(todos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        let path = std::path::Path::new("./tests/props/parse_prob.csv");
        
        let todos = match parse_csv(&path){
            Err(err) => panic!("test failed because: {}", err),
            Ok(todos) => todos,
        };
        assert_eq!(todos.len(), 3);
    }

    #[test]
    fn test_todos_to_csv() {
        let todo = TodoItem::new(String::from("test"), false);
        let todo2 = TodoItem::new(String::from("test2"), true);
        let mut todos: Vec<TodoItem> = vec![];
        todos.push(todo);
        todos.push(todo2);

        let csv = todos_to_csv(&todos);
        assert_eq!(csv, "test,false\ntest2,true")
    }
}
