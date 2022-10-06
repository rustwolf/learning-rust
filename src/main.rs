#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Endpoint, Error};
use http::Uri;
use std::{sync::Mutex, *};

use structs::structs::User;

mod loops;

mod variables;

mod arcmutex;

mod lazystatic;

mod thread;

mod structs;

mod iterators;

mod options;

mod async_mod;

mod lazy_static_queue;

mod reference;

mod tokio_run;

mod strings;

mod match_mod;

mod learnmutex;

// mod dynamodb;

mod iters;

mod ownership;

mod learnenums;

mod jsonlearn;

mod refcell_learn;

#[tokio::main]
async fn main() {
    println!("Hello, world & Rust Developers");

    println!("Lets do some rust coding {}", String::from("Hello world"));

    // ownership::ownership::main();
    // iters::iters::main();'

    learnenums::learnenums::main();

    jsonlearn::jsonlearn::main();

    refcell_learn::refcell_learn::main();
}

fn pass_ref(something: &Mutex<Vec<String>>) {
    something
        .lock()
        .unwrap()
        .push(String::from("pass ref called"));
}
async fn get_name_async() -> Result<String, String> {
    Ok(String::from("Azhar is here"))
}

fn print_me(user: &User) {
    // user.clone();
    println!("{:?}", user);
}

fn add_numbers<T: std::ops::Add>(a: T, b: T) -> <T as std::ops::Add>::Output {
    a + b
}
