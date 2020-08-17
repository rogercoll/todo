use std::error::Error;

pub enum Action {
    Add,
    Delete
}

pub struct Config {
    pub action: Action,
    pub message: String,
}

fn type_of_action(action: &String) -> Result<Action, &'static str> {
    match action.as_ref() {
        "add" => Ok(Action::Add),
        "del" => Ok(Action::Delete),
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
    config.action.process("mongoClient", &config.message);
    Ok(())
}

impl Action {
    pub fn details(&self) {
        match self {
            Action::Add => println!("Adding action"),
            Action::Delete => println!("Deleting action"),
        }
    }
    fn process(&self,a: &str,b: &str) {
        println!("Running!")
    }
}