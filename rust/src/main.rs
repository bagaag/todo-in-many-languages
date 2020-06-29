mod todo;

use std::env;
use crate::todo::{Items};

// where to save the list
static DATAFILE: &str = ".todo";


// main program flow
fn main() {
    // create and populate items struct
    let mut items = Items::new();
    items.load(DATAFILE);
    // parse cli args
    let args: Vec<String> = env::args().collect();
    let mut cmd = "list";
    if args.len() > 1 {
        cmd = &args[1];
    }
    // act on given command
    if cmd == "list" {
        items.print_list();
    }
    else if cmd == "add" && args.len() > 2 {
        let the_rest = args[2..].join(" ").to_string();
        items.add(the_rest);
        items.print_list();
        items.save(DATAFILE);
    }
    else if cmd == "edit" && args.len() > 3 {
        let ix: u32 = args[2].parse().unwrap();
        let the_rest = args[3..].join(" ").to_string();
        items.edit_at(ix, the_rest);
        items.print_list();
        items.save(DATAFILE);
    }
    else if cmd == "complete" && args.len() == 3 {
        let ix:u32 = args[2].to_string().parse().unwrap();
        items.complete_at(ix);
        items.print_list();
        items.save(DATAFILE);
    }
    else if cmd == "completed" {
        items.print_completed();
    }
    else if cmd == "clear" {
        let count = items.clear();
        println!("{} completed items cleared.", count);
        items.save(DATAFILE);
    }
    else if cmd == "help" {
        let help: String = 
"Usage: todo <cmd> <args>

Commands:
  list                      Displays active list items.
  add <description>         Adds an item to the list.
  edit <num> <description>  Replaces a numbered item in the list.
  complete <num>            Completes the numbered item.
  completed                 Displays completed items.
  clear                     Deletes completed items.
  help                      Displays this message.".to_string();
        println!("{}", help);
    }
    else { 
        println!("Sorry, {} is not a valid command. Try 'help'.", cmd);
    }
}


