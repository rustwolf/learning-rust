pub mod refcell_learn {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug)]
    pub struct Person {
        pub name : String
    }

    pub fn main() {

        let mut me = Person {
            name : String::from("Azhar")
        };

        let ref_me = RefCell::new(me);

        // println!("Learning refcell_learn {:?}", ref_me.into_inner());

        change_name(ref_me.into_inner());

        // let cell = RefCell::new(me);
    }

    pub fn change_name(mut person: Person) {
        person.name = String::from("Khan");

        println!("{}", person.name);
    }

}