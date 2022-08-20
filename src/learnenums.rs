pub mod learnenums {
    #[derive(Debug)]
    pub enum Item {
        Something(String),
        Nothing,
    }
    pub fn main() {
        println!("We are learning Enums");
        let a = Item::Something(String::from("Hello World inside Enum"));

        println!("{:?}", a);
    }
}
