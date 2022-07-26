
use mockall::automock;

use lazy_static::lazy_static;
    
use std::sync::{Mutex, Arc};

lazy_static! {
    static ref my_messages : Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![String::from("Hello world")]));
}

#[automock]
pub mod lazystatic {
    use std::sync::{Mutex, Arc};

    pub fn print_me() {
        // super::my_messages;

        let my_message = super::my_messages.clone();

        my_message.lock().unwrap().push(String::from("Hi there we are here !!"));

        // super::my_messages.lock().unwrap().push(String::from("Hello world Again "));

        // println!("{}", my_message.());


        println!("{}", my_message.lock().unwrap().len());

        println!("Hello Broooo");
    }
}