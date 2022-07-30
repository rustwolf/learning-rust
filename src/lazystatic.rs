
use mockall::automock;

use lazy_static::lazy_static;
    
use std::sync::{Mutex, Arc};

lazy_static! {
    static ref my_messages : Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![String::from("Hello world")]));
}

#[automock]
pub mod lazystatic {
    use std::sync::{Mutex, Arc};
    use std::thread;
    use std::time::Duration;

    
    pub fn print_me() {
        // super::my_messages;

        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                let my_message = super::my_messages.clone();
                my_message.lock().unwrap().push(String::from("Hi there we are here !!"));

                if my_message.lock().unwrap().len() == 10 {
                    println!(">>>>>>>>>>>>>>>>>>>> We need to drain the queue");
                }
            }
        });
    

        // super::my_messages.lock().unwrap().push(String::from("Hello world Again "));

        // println!("{}", my_message.());

        handle.join().unwrap();

        let my_message = super::my_messages.clone();

        println!("{}", my_message.lock().unwrap().len());

        println!("Hello Broooo");

    }
}