use async_trait::async_trait;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

use mockall::automock;

lazy_static! {
    static ref my_messages: Mutex<Vec<String>> = Mutex::new(vec![String::from("Hello world")]);
}

#[derive(Debug)]
struct Message {
    pub name: String,
}

pub fn print_me() {
    // let myvalue = Mutex::new(10);
    let mut MessagesList: Mutex<Vec<Message>> = Mutex::new(vec![Message {
        name: String::from("Azhar is here"),
    }]);

    let value = MessagesList.lock().unwrap();

    my_messages
        .lock()
        .unwrap()
        .push(String::from("New Messages"));

    my_messages
        .lock()
        .unwrap()
        .push(String::from("New Messages"));

    my_messages
        .lock()
        .unwrap()
        .push(String::from("New Messages"));

    my_messages
        .lock()
        .unwrap()
        .push(String::from("New Messages"));

    my_messages
        .lock()
        .unwrap()
        .push(String::from("New Messages"));

    print!("The length is :{}", my_messages.lock().unwrap().len());

    print!("{:?}", value.get(0).unwrap().name);
}
