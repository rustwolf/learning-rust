use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Gender {
    MALE,
    FEMALE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    age: u32,
    gender: Gender,
}

pub mod jsonlearn {
    use crate::jsonlearn::{Gender, Person};

    pub fn main() {
        let sheetal = Person {
            name: String::from("Sheetal"),
            age: 26,
            gender: Gender::MALE,
        };

        println!(
            "We are learning JSON {}",
            serde_json::to_string(&sheetal).unwrap()
        );

        let json_string = serde_json::to_string(&sheetal).unwrap();

        let sheetalNew: Person = serde_json::from_str(&json_string).unwrap();

        println!("Age is {}", sheetalNew.age);
    }
}
