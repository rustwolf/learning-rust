
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

mod loops;

mod variables;

mod arcmutex;

mod lazystatic;

fn main() {
    println!("Hello, world!");

    lazystatic::lazystatic::print_me();

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
    
}


