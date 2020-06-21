//use std::io;
use chrono::{DateTime, Utc};
use json::{object};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// where to save the list
static DATAFILE: &str = ".todo";

// A to do list item
struct Item {
    description: String,
    completed: Option<DateTime<Utc>>
}

// A to do list
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
            data["items"].push(item);
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
			Ok(_) => println!("Saved list at {}.", display),
		}
    }
}

fn main() {
    println!("WIP");
    let mut items = Items::new();
    items.add(String::from("Hey now."));
    let json = items.to_json();
    println!("{}", json);
	items.save();
}
