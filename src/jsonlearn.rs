use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Gender {
    MALE,
    FEMALE
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name : String,
    age : u32,
    gender : Gender
}   

pub mod jsonlearn {
    use crate::jsonlearn::{Person, Gender};


    pub fn main() {
        let sheetal = Person {
            name : String::from("Sheetal"),
            age : 26,
            gender : Gender::MALE
        };

        println!("We are learning JSON {}", serde_json::to_string(&sheetal).unwrap());
    }
}