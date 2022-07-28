
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use structs::structs::User;

mod loops;

mod variables;

mod arcmutex;

mod lazystatic;

mod thread;

mod structs;

mod iterators;

mod options;

fn main() {
    println!("Hello, world & Rust Developers ");

    // lazystatic::lazystatic::print_me();

    // thread::thread::run_thread();
    // lazystatic::print_me();
    
    // loops::for_loop::run_for_loop();
    // loops::loop_over_vec::vec_loop();
    // loops::infinite_loop::run_infinite_loop();

    // variables::declare_variables();

    // variables::process_record(String::from("Azhar"));

    // for i in 0..11 {
    //     variables::process_record(String::from("Azhar "));
    // }

    // for i in 0..11 {
    //     variables::process_record(String::from("Azhar "));
    // }
    
    // arcmutex::print_me();

    // let az = structs::structs::create_user();
    
    // print_me(&az);
    // println!("{:?}", az);

    // let names = iterators::iterators::create_iter();
    
    // for name in names {
    //     println!("{}", name);
    // }

    // iterators::iterators::main();

    options::options::main();
    
}


fn print_me(user : &User) {
    // user.clone();
    println!("{:?}", user);
}