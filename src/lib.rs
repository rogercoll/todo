use std::error::Error;
use mongodb::{
    options::FindOptions,
    bson::{doc, Bson},
    sync::Client,
};


pub enum Action {
    Add,
    Delete,
    List
}

pub struct Config {
    pub action: Action,
    pub message: String,
}

fn type_of_action(action: &String) -> Result<Action, &'static str> {
    match action.as_ref() {
        "add" => Ok(Action::Add),
        "del" => Ok(Action::Delete),
        "list" => Ok(Action::List),
        _ => return Err("Invalid parameter"),
    }
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        //let action = args[1].clone();
        let action = type_of_action(&args[1]).unwrap();
        let message = args[2].clone();


        Ok(Config {
            action,
            message,
        })

    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //here we must do the mongo client and pass it to action function
    config.action.process(&config.message);
    Ok(())
}

fn add_task(client: &Client, message: &str, tnumber: i32) {
    let db = client.database("tasks");
    let collection = db.collection("todo");
    let docu = doc! {"priority": 2, "message": message, "tnumber": tnumber };
    collection.insert_one(docu,None).unwrap();
}

fn delete_task(client: &Client, tnumber: i32) {
    let db = client.database("tasks");
    let collection = db.collection("todo");
    collection.delete_one(doc! {"tnumber": tnumber}, None).unwrap();
}

fn list_task(client: &Client) {
    let db = client.database("tasks");
    let collection = db.collection("todo");
    let find_options = FindOptions::builder()
        .sort(doc! { "priority": 1 })
        .build();
    let cursor = collection.find(None, find_options).unwrap();
    for result in cursor {
        match result {
            Ok(document) => {
                if let Some(message) = document.get("message").and_then(Bson::as_str) {
                    if let Some(tnumber) = document.get("tnumber").and_then(Bson::as_i32) {
                        if let Some(priority) = document.get("priority").and_then(Bson::as_i32) {
                            match priority {
                                1  => println!("\x1b[0;31;1m#{} TODO:\x1b[0m \x1b[0;31;1;4m{}\x1b[0m", tnumber, message),
                                2 => println!("\x1b[0;33m#{} TODO:\x1b[0m \x1b[0;33;4m{}\x1b[0m", tnumber, message),
                                _ => println!("#{} TODO: {}", tnumber, message),
                            }
                        } else {
                            println!("No priority task => #{} TODO: {}", tnumber, message);
                        }
                    } else {
                        println!("No tasak number found!");
                    }
                } else {
                    println!("no message found");
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn get_tnumber(client: &Client) -> i32 {
    let db = client.database("tasks");
    let collection = db.collection("todo");
    let find_options = FindOptions::builder()
        .sort(doc! { "tnumber": 1 })
        .build();
    let cursor = collection.find(None, find_options).unwrap();
    let mut last_value :i32 = 1;
    for result in cursor {
        match result {
            Ok(document) => {
                if let Some(tnumber) = document.get("tnumber").and_then(Bson::as_i32) {
                    //let value: i32 = knumber.to_string().parse().unwrap();
                    if tnumber == last_value {
                        last_value += 1;
                    }
                } else {
                    println!("no tnumber document found");
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    return last_value;
}

impl Action {
    pub fn details(&self) {
        match self {
            Action::Add => println!("Adding action"),
            Action::Delete => println!("Deleting action"),
            Action::List => println!("Lists of task to be done:"),
        }
    }
    fn process(&self, message: &str) {
        let uri = "";
        let client = Client::with_uri_str(uri).unwrap();
        match self {
            Action::Add => {
                let tnumber: i32 = get_tnumber(&client);
                add_task(&client, message, tnumber);
            }
            Action::Delete => {
                let tnumber: i32 = message.to_string().parse().unwrap();
                delete_task(&client, tnumber);
            } 
            Action::List => list_task(&client),
        }
        println!("Running!")
    }
}   