extern crate clap;

use clap::{Arg, App};
mod todo_item;

use todo_item::TodoItem;

fn main(){
    let matches = App::new("My Super Program")
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

// You can check the value provided by positional arguments, or option arguments
    if let Some(a) = matches.value_of("add") {
        let item = TodoItem::new(a.to_string(), false);
        println!("New Todo item added. \nName: {} Done: {}", item.name , item.done);
    }

    if let Some(l) = matches.value_of("list") {
         println!("Value for list: {}", l);
    }
    // Continued program logic goes here...
}

