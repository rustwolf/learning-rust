
pub mod learnmutex {
    use std::sync::Mutex;

    pub fn main() -> Mutex<Vec<String>>{
        let vec = vec![String::from("Azhar")];
        let mut m = Mutex::new(vec);
        
        // let lo = m.lock();

        m.lock().unwrap().push(String::from(" Khan"));

        m.lock().unwrap().push(String::from(", Is learning Rust"));

        let dat = m.lock().unwrap().len();

        println!("{}", dat);

        m
    }

}