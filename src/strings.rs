pub mod strings {
    pub fn main() {

        let mut name = String::from("Azhar");

        name.push_str("uddin Khan");

        println!("{}", name);
        

        let len = name.len();


        let part = &name[0..5];

        println!("The length is {} and substring is {}", len, part);

        println!("The orignal is , {}", name);



    }
}