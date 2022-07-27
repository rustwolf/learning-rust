pub mod thread {
    use std::{sync::{Arc, Mutex}, thread};


    pub fn run_thread() {
        println!("This is from thread");

        let a = Arc::new(Mutex::new(10));

  
            
            // let mut all_threads = vec![];


        for i in 1..10 {

            let  a = a.clone();
            let threads = thread::spawn(move || {
                
                *a.lock().unwrap()+=1;

                println!("We are inside a thread , {}", *a.lock().unwrap());
            });
            // all_threads.push(threads);

            threads.join().unwrap();
        }


        // threads.join().unwrap();
    }
}

