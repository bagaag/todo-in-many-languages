//use std::io;

use chrono::{DateTime, Utc, Datelike};
use json::{object};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// where to save the list
static DATAFILE: &str = ".todo";

// A to do list item
#[derive(Debug)]
struct Item {
    description: String,
    completed: Option<DateTime<Utc>>
}
impl Item {
    fn complete(&mut self) {
        let now: DateTime<Utc> = Utc::now();
        self.completed = Some(now);
    }

}

// A to do list
#[derive(Debug)]
struct Items {
    items: Vec<Item>
}
impl Items {
    fn new() -> Items {
        Items { items: Vec::new() }
    }

    fn all(&self) -> &[Item] {
        self.items.as_ref()
    }

    fn add(&mut self, desc: String) {
        self.items.push( 
            Item { 
                description: desc, 
                completed: Option::None 
            } 
        );
    }
    
    fn at(&self, target: u32) -> Option<&Item> {
        let mut ix = 0;
        for item in self.items.iter() {
            if item.completed.is_none() {
                ix = ix + 1;
                if ix == target {
                    return Some(item);
                }
            }
        }
        None
    }

    fn edit_at(&mut self, target: u32, desc: String ) -> Option<&Item> {
        let mut ix = 0;
        for mut item in self.items.iter_mut() {
            if item.completed.is_none() {
                ix = ix + 1;
                if ix == target {
                    item.description = desc;
                    return self.at(target);
                }
            }
        }
        None
    }

    fn complete_at(&mut self, target: u32 ) -> Option<&Item> {
        let mut ix = 0;
        for item in self.items.iter_mut() {
            if item.completed.is_none() {
                ix = ix + 1;
                if ix == target {
                    item.complete();
                    return self.at(target);
                }
            }
        }
        None
    }

    fn to_json(&self) -> String {
        let mut data = object!{ items: [] };
        for item in self.all() {
            let item = object!{
                description: &item.description[..],
                completed: match item.completed {
                    Some(date_value) => json::JsonValue::String(date_value.to_string()),
                    None => json::Null
                }
            };
            data["items"].push(item).expect("Sorry, you have too much to do.");
        }
        data.dump()
    }

    fn save(&self) {
        let path = Path::new(DATAFILE);
		let display = path.display();
		let mut file = match File::create(&path) {
			Err(why) => panic!("Error: Couldn't create file to save list at {}: {}", display, why),
			Ok(file) => file,
    	};
		match file.write_all(self.to_json().as_bytes()) {
			Err(why) => panic!("Error: Couldn't write list to file at {}: {}", display, why),
			Ok(_) => (),
		}
    }

    fn load(&self, filename: &str) {
        let path = Path::new(DATAFILE);
		let display = path.display();
		let mut file = match File::open(&path) {
			Err(why) => panic!("couldn't open {}: {}", display, why),
			Ok(file) => file
		};
	    let mut s = String::new();
    	match file.read_to_string(&mut s) {
			Err(why) => panic!("couldn't read {}: {}", display, why),
			Ok(_) => ()
	    }
    }
}

fn date_to_string(d: &DateTime<Utc>) -> String {
    format!("{}-{:02}-{:02}", d.year(), d.month(), d.day())
}

fn print_list(list: &[Item]) {
    let mut ix = 1;
    for item in list {
        println!("{:>2}. {}", ix, item.description);
        ix = ix + 1;
    }
}

fn print_completed(list: &[Item]) {
    for item in list {
        if item.completed.is_some() {
            let dt: DateTime<Utc> = item.completed.unwrap();
            let ds = date_to_string(&dt);
            println!("{} - {}", ds, item.description);
        }
    }
}

/*
fn test() {
    let mut items = Items::new();
    items.add("Hey now.".to_string());
    items.add("Hey now, a GEEEEin.".to_string());
    print_list(items.all());
    let item = items.edit_at(1, "Nope.".to_string());
    if item.is_some() {
        println!("Item 1: {}", item.unwrap().description);
    } else {
        println!("Item 1 not found :(");
    }
    items.complete_at(1);    
    items.complete_at(1);
    print_completed(items.all());
    let json = items.to_json();
    println!("{}", json);
	items.save();
}
*/

fn main() {
    let mut items = Items::new();
    items.load();
    let args: Vec<String> = env::args().collect();
    let mut cmd = "list";
    if args.len() > 1 {
        cmd = &args[1];
    }
    if cmd == "list" {
        println!("list");
    }
    else { 
        println!("Sorry, {} is not a valid command. Try 'help'.", cmd);
    }
}


