extern crate clap;

use clap::{App, Arg};
mod todo_item;
mod util;

use std::path;
use todo_item::TodoItem;

fn main() -> std::io::Result<()> {
     let matches = App::new("Simple ToDo App")
          .version("0.01")
          .author("phosfox")
          .about("Simple Todo App")
          .arg(Arg::with_name("add")
               .short("a")
               .long("add")
               .value_name("INPUT")
               .help("Adds a todo item")
               .takes_value(true))
          .arg(Arg::with_name("list")
               .short("l")
               .long("list")
               .help("lists all todo items"))
          .get_matches();

     let path = path::Path::new(".todos");
     let mut todos: Vec<TodoItem> = vec![];

     if path.is_file() {
          todos = match util::parse_csv(&path) {
               Ok(todos) => todos,
               Err(err) => {
                    println!("could not parse todos-file: {}", err);
                    std::process::exit(1);
               }
          };
     }

     if let Some(a) = matches.value_of("add") {
          let item = TodoItem::new(a.to_string(), false);
          println!(
               "New Todo item added. \nName: {} Done: {}",
               item.name, item.done
          );
          todos.push(item);
     }


     if matches.is_present("list") {
          util::print_todos(&todos);
     }

     let csv_string = util::todos_to_csv(&todos);
     std::fs::write(path, csv_string)?;
     Ok(())
}
