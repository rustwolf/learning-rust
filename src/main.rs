#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Endpoint, Error};
use http::Uri;
use std::*;

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

// mod dynamodb;

#[tokio::main]
async fn main() {
    println!("Hello, world & Rust Developers");
    let data = get_name_async().await;
    println!("{:?}", data.unwrap());
    println!("{}", add_numbers(10.00, 12.04));
}

async fn get_name_async() -> Result<String, String> {
    Ok(String::from("Azhar is here"))
}

fn print_me(user: &User) {
    // user.clone();
    println!("{:?}", user);

}


fn add_numbers<T:std::ops::Add>(a:T, b:T) -> <T as std::ops::Add>::Output {
    a + b
}