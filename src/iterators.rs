pub mod iterators {
    pub fn create_iter() -> Vec<String> {
        return vec![
            String::from("Azhar"),
            String::from("Rajdeep"),
            String::from("Sheetal"),
        ];
    }

    pub fn get_data() {
        let a = vec![10, 20, 30, 50];
        match a.get(12) {
            Some(x) => println!("{}", x),
            None => println!("Index out of Bounds"),
        }
    }
    pub fn main() {
        println!("Hello from Iterators");

        get_data();
    }
}
