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
async fn main() -> Result<(), Error> {
    println!("Hello, world & Rust Developers ");

    // lazy_static_queue::lazy_static_queue::test_queue();

    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 1"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 2"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 3"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 4"));

    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 5"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 6"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 7"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 8"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 9"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 10"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 11"));
    // lazy_static_queue::lazy_static_queue::add_message(String::from("Message 12"));

    // lazy_static_queue::lazy_static_queue::tes

    //lazystatic::lazystatic::print_me();

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

    // options::options::main();

    // async {
    //     let data = async_mod::async_mod::main().await;
    //     println!("{}", data);
    // };

    // reference::refereneces::main();

    // tokio_run::tokio_run::main_run();

    // strings::strings::main();

    // match_mod::match_mod::main();

    // dynamodb::dynamodb::main();

    // Select a profile by setting the `AWS_PROFILE` environment variable.
    let config = aws_config::load_from_env().await;
    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_resolver(
            // 8000 is the default dynamodb port
            Endpoint::immutable(Uri::from_static("http://localhost:8000")),
        )
        .build();

    let client = Client::from_conf(dynamodb_local_config);

    let resp = client.list_tables().send().await?;

    println!("Tables:");

    let names = resp.table_names().unwrap_or_default();

    for name in names {
        println!("  {}", name);
    }

    println!();
    println!("Found {} tables", names.len());

    Ok(())
}

fn print_me(user: &User) {
    // user.clone();
    println!("{:?}", user);
}
