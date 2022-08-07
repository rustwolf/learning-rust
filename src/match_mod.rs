pub mod match_mod {
    use std::cmp::Ordering;

    use crate::arcmutex::print_me;

    pub fn main() {
        let data = 100;
        let data1 = 200;

        match data.cmp(&data1) {
            Ordering::Less => println!("We are here !!"),
            Ordering::Greater => println!("We are greater"),
            Ordering::Equal => println!("We are equal")
        };

        let opt = Some(100);
        let void = None;


        match_me(opt);
        match_me(void);


        let s = Some(String::from("Azhar is here, inside an option"));

        println!("{}",s.unwrap());

    }

    pub fn match_me(data : Option<i32>) {
        match data {
            Some(data) => println!("We have some data bro, {}", data),
            None => println!("We have no data") 
       };
    } 
}