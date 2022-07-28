pub mod options {

    fn create_new_opt() -> Option<String> {

        Some(String::from("AZHAR"))
    }
    
    pub fn main() {

        let opt = create_new_opt();

        match &opt {
            Some(x) => println!("{}", x),
            None => println!("It's not Azhar")
        }

        println!("{}", opt.unwrap_or(String::from("Helloowwww")));
    
    }

}