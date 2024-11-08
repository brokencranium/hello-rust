mod concurrency;
mod debug;
mod display;
mod generics;
mod lifetime;
mod macros;
mod ownership;
mod print_options;
mod stocks;
mod structures;
mod traits;

use crate::stocks::structs::stock::Stock;
use concurrency::fibs::{fibonacci_main, multi_thread, single_thread};
use debug::print_debug;
use display::display_objects;
use generics::generics;
use lifetime::lifetime;
use macros::macros;
use ownership::ownership;
use structures::structures;
use traits::traits;

use rand::{thread_rng, Rng};
use std::io;

fn hello_world() {
    let mut guess = String::new();
    println!("Hello, world!");

    io::stdin()
        .read_line(&mut guess)
        .expect("Unable to read line");

    println!("The value read was: {guess}");

    let mut range = thread_rng();
    let rand_number: u32 = range.gen_range(1..=10);
    println!("Random number generated is : {rand_number}");
}

fn main_stock() {
    let stock: Stock = Stock::new("GenAI", 500.0);
    println!("Name: {}, Price: {}", stock.name, stock.open_price);
}

fn main() {
    // hello_world();
    // print_options::print_examples();
    // print_debug();
    // display_objects();
    // ownership();
    // lifetime();
    // structures();
    // traits();
    // macros();
    // generics();

    // // Stock
    // main_stock()

    // Threading and processes
    single_thread();
    multi_thread(4);
    fibonacci_main();
}
