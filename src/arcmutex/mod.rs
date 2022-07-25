use std::sync::{Mutex, Arc};


#[derive(Debug)]
struct Message {
    pub name : String
}


pub fn print_me() {
    println!("We are here");

    // let myvalue = Mutex::new(10);
    let mut MessagesList:Mutex<Vec<Message>> = Mutex::new(vec![Message{name : String::from("Azhar is here")}]);


    let value = MessagesList.lock().unwrap();

    print!("{:?}", value.get(0).unwrap().name);
    


}