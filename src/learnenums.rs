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
        println!("{:?}", a);
    }
}