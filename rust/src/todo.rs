use chrono::{DateTime, Utc, Local, Datelike, Timelike};
use json::{object, JsonValue};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// A to do list item
#[derive(Debug)]
pub struct Item {
    description: String,
    completed: Option<DateTime<Utc>>
}
impl Item {
    pub fn complete(&mut self) {
        let now: DateTime<Utc> = Utc::now();
        self.completed = Some(now);
    }
}

// A to do list
#[derive(Debug)]
pub struct Items {
    items: Vec<Item>
}
impl Items {
    pub fn new() -> Items {
        Items { items: Vec::new() }
    }

    fn all(&self) -> &[Item] {
        self.items.as_ref()
    }

    pub fn add(&mut self, desc: String) {
        self.items.push( 
            Item { 
                description: desc, 
                completed: Option::None 
            } 
        );
    }
    
    pub fn at(&self, target: u32) -> Option<&Item> {
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

    pub fn edit_at(&mut self, target: u32, desc: String ) -> Option<&Item> {
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

    pub fn complete_at(&mut self, target: u32 ) -> Option<&Item> {
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
                    Some(date_value) => json::JsonValue::String(date_value.to_rfc3339()),
                    None => json::Null
                }
            };
            data["items"].push(item).expect("Sorry, you have too much to do.");
        }
        data.dump()
    }

    pub fn save(&self, filename: &str) {
        let path = Path::new(filename);
		let display = path.display();
		let mut file = match File::create(&path) {
			Err(why) => panic!("Error: Couldn't create file to save list at {}: {}", display, why),
			Ok(file) => file
    	};
		match file.write_all(self.to_json().as_bytes()) {
			Err(why) => panic!("Error: Couldn't write list to file at {}: {}", display, why),
			Ok(_) => ()
		};
    }

    // loads todo data from storage
    pub fn load(&mut self, filename: &str) {

        // open file
        let path = Path::new(filename);
        if !path.exists() {
            return;
        }
		let display = path.display();
		let file = match File::open(&path) {
			Err(why) => panic!("Error: Couldn't read file at {}: {}", display, why),
			Ok(file) => Some(file)
		};
        
        // no data file yet, move on
        if file.is_none() {
            return;
        }
        
        // read file into string
	    let mut s = String::new();
    	match file.unwrap().read_to_string(&mut s) {
			Err(why) => panic!("Error: couldn't read file at {}: {}", display, why),
			Ok(_) => ()
	    }
        
        // parse string into json
        let js_root = json::parse(&s).unwrap();
        
        // build struct from json
        let js_items = &js_root["items"];
        match js_items {
        
            // test and process js_items
            JsonValue::Array(js_items) => {
                for js_item in js_items.iter() {
            
                    // test and process an item
                    match js_item {
                        JsonValue::Object(js_item) => {
                    
                            // test and process item.completed
                            let js_completed = &js_item["completed"];
                            let completed = match js_completed {
                                 JsonValue::String(js_completed) => {
                                    let parse_result = DateTime::parse_from_rfc3339(&js_completed);
                                    match parse_result {
                                        Err(why) => {
                                            println!("Error reading stored data; invalid completed date: {}", why);
                                            None
                                        },
                                        Ok(fixed_offset) => {
                                            let utc: DateTime<Utc> = DateTime::from(fixed_offset);
                                            Some(utc)
                                        }
                                    }
                                },
                                _ => None
                            };

                            // test and process item.description
                            let js_desc = &js_item["description"];
                            let description = match js_desc {
                                JsonValue::String(js_desc) => {
                                    js_desc.to_string()
                                },
                                JsonValue::Short(js_desc) => {
                                    js_desc.to_string()
                                },
                                _ => {
                                    println!("Error reading stored data; expected item to have description but found {:#?}", js_desc);
                                    "".to_string()
                                }
                            };
                            // create the item and add to list
                            let item = Item { 
                                description: description,
                                completed: completed
                            };
                            self.items.push(item);
                        },
                        _ => println!("{}","Error reading stored data; expected items array to contain item objects")
                    }
                }
            },
            _ => println!("Error in stored data; expected array named 'items'")
        }
    }
    
    // prints numbered items that aren't yet completed
    pub fn print_list(&self) {
        let mut ix = 0;
        for item in self.items.iter() {
            if item.completed == None {
                ix = ix + 1;
                println!("{:>2}. {}", ix, item.description);
            }
        }
        if ix == 0 {
            println!("You're all done!");
        }
    }

    // prints completed items
    pub fn print_completed(&self) {
        let mut found: bool = false;
        for item in self.items.iter() {
            if item.completed.is_some() {
                let utc: DateTime<Utc> = item.completed.unwrap();
                let loc: DateTime<Local> = utc.with_timezone(&Local);
                let (pm, hr) = loc.hour12();
                let ds = format!("{}-{:02}-{:02} {}:{:02} {}", loc.year(), loc.month(), loc.day(), 
                                 hr, loc.minute(), if pm { "PM" } else { "AM" });
                println!("{}: {}", ds, item.description);
                found = true;
            }
        }
        if !found {
            println!("There are no completed items.");
        }
    }

    // clears out completed items and returns # cleared
    pub fn clear(&mut self) -> usize {
        let mut ixs = Vec::<usize>::new();
        for (ix, item) in self.items.iter().enumerate() {
            if item.completed != None {
                ixs.insert(0, ix);
            }
        }
        for ix in &ixs {
            self.items.remove(*ix);
        }
        ixs.len()
    }
}
