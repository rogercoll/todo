use std::error::Error;
use mongodb::{
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

fn add_task(client: Client, message: &str) {
    let db = client.database("tasks");
    let collection = db.collection("todo");
    let docu = doc! {"priority": "1", "message": message, "tnumber": "1"};
    collection.insert_one(docu,None).unwrap();
}

fn delete_task(client: Client, tnumber: &str) {
    let db = client.database("tasks");
    let collection = db.collection("todo");
    let cursor = collection.delete_one(doc! {"tnumber": tnumber}, None).unwrap();
}

fn list_task(client: Client) {
    let db = client.database("tasks");
    let collection = db.collection("todo");
    let cursor = collection.find(None, None).unwrap();
    for result in cursor {
        match result {
            Ok(document) => {
                if let Some(message) = document.get("message").and_then(Bson::as_str) {
                    println!("message: {}", message);
                } else {
                    println!("no message found");
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

impl Action {
    pub fn details(&self) {
        match self {
            Action::Add => println!("Adding action"),
            Action::Delete => println!("Deleting action"),
            Action::List => println!("List action"),
        }
    }
    fn process(&self, message: &str) {
        let uri = "";
        let client = Client::with_uri_str(uri).unwrap();
        match self {
            Action::Add => add_task(client, message),
            Action::Delete => delete_task(client, message),
            Action::List => list_task(client),
        }
        println!("Running!")
    }
}   