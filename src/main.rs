extern crate clap;

use clap::{Arg, App};
mod todo_item;
mod util;

use todo_item::TodoItem;
use std::path;

fn main(){
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
          println!("exists");
          //todos = util::parse_csv(path);
     } else {
          let file = match std::fs::File::create(&path) {
               Err(why) => panic!("unable to create {}: {}", path.display(), why),
               Ok(file) => file,
          };

          let csv_string = util::todos_to_csv(&todos);
          std::fs::write(path, csv_string).expect("Unable to write file");
          println!("created");
     }

    todos.push(TodoItem::new(String::from("Placeholder"), false));

    if let Some(a) = matches.value_of("add") {
        let item = TodoItem::new(a.to_string(), false);
        println!("New Todo item added. \nName: {} Done: {}", item.name , item.done);
        todos.push(item);
    }


    println!("{:?}", matches.args.values());

    if matches.is_present("list"){
         util::print_todos(&todos);
    }
}

