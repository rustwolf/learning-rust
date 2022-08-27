pub mod learnenums {
    #[derive(Debug)]
    pub enum Item {
        Something(String),
        Nothing
    }
    pub fn main() {
        println!("We are learning Enums");
        let a = Item::Something(String::from("Hello World inside Enum"));

        match &a {
            Item::Something(a) => { println!("{}",a)},
            Item::Nothing => { println!("WE found nothing")}
        }
        let name = get_name().unwrap();

        println!("{}", name);
        println!("{:?}", a);
    }

    pub fn get_name() -> Option<String>{
        Some(String::from("Hello some type"))
    }
}