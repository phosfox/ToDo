extern crate csv;

use crate::todo_item::TodoItem;
use std::io;
use std::process;
use std::str::FromStr;
use std::num;

enum ParseBoolError {
    IoError(io::Error)
}
impl From<io::Error> for ParseBoolError {
    fn from(error: io::Error) -> Self {
        ParseBoolError::IoError(error)
    }
    
}

pub fn write_todos(todos: &Vec<TodoItem>, mut to: impl std::io::Write){
    for todo in todos.iter() {
        writeln!(to, "{},{}", todo.name, todo.done).unwrap();
    }
}

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

pub fn parse_csv(path: &std::path::Path) -> Result<Vec<TodoItem>, io::Error> {
    let mut reader = match csv::ReaderBuilder::new()
    .has_headers(false)
    .from_path(path) {
        Err(err) => return Err(From::from(err)),
        Ok(reader) => reader,
    };

    let mut todos: Vec<TodoItem> = vec![];

    for result in reader.records() {
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                let name: String = match record.get(0) {
                    Some(name) => name.to_string(),
                    None => panic!("name could not be parsed"),
                };
                
                let done: bool = match record.get(1) {
                    Some(done) => FromStr::from_str(done)?,
                    None => panic!("done could not be parsed"),
                };

                todos.push(TodoItem::new(name, done));
            }
        }
    }

    return Ok(todos);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        //let prop_todos = vec![TodoItem::new(String::from("test1"), false), TodoItem::new(String::from("test2"), false), TodoItem::new(String::from("test3"), false)];

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
    #[test]
    fn test_write_todos(){
        let todo = TodoItem::new(String::from("test"), false);
        let todo2 = TodoItem::new(String::from("test2"), true);
        let mut todos: Vec<TodoItem> = vec![];
        todos.push(todo);
        todos.push(todo2);

        let mut result = Vec::new();
        write_todos(&todos, &mut result);
        assert_eq!(result, b"test,false\ntest2,true\n");
    }
}
