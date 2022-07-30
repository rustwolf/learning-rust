
use mockall::automock;

use lazy_static::lazy_static;
    
use std::sync::{Mutex, Arc};

lazy_static! {
    static ref my_messages : Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
}

#[automock]
pub mod lazy_static_queue {

    pub fn test_queue() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let removed_ele = v.drain(0..10);
        println!("{:?} Are removed Elements", removed_ele);
    }


    pub fn print_me() {
        println!("Hi there, we are in lazy static");
    }

    pub fn add_message(str : String) {
        let a = super::my_messages.clone();
        a.lock().unwrap().push(str);
    

        println!("Added New Message The length is now {}", a.lock().unwrap().len());

        if a.lock().unwrap().len() == 10 {
            let ba = super::my_messages.clone();
            let mut removed_items = ba.lock().unwrap();
            let items = removed_items.drain(0..10);

            println!("Removed Elements  >>>>>>>>  {:?}", items);

        }
        let again = super::my_messages.clone();

        println!("New Length is now >>>>>>>>  {}", again.lock().unwrap().len());


    }
}