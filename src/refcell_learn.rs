pub mod refcell_learn {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug)]
    pub struct Person {
        pub name: String,
    }

    pub fn main() {
        let mut me = Person {
            name: String::from("Azhar"),
        };

        let you = Person {
            name: String::from("Jon"),
        };

        let b = Box::new(you);

        let ref_me = RefCell::new(me);

        change_name(ref_me.into_inner());
    }

    pub fn change_name(mut person: Person) {
        person.name = String::from("Khan");

        println!("{}", person.name);
    }
}
