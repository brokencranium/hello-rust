mod debug;
mod display;
mod generics;
mod lifetime;
mod macros;
mod ownership;
mod print_options;
mod structures;
mod traits;

// use display::display_objects;
// use debug::print_debug;
// use ownership::ownership;
// use lifetime::lifetime;
// use structures::structures;
// use traits::traits;
// use macros::macros;
use generics::generics;

// use std::io;
// use rand::{thread_rng,Rng};

// fn hello_world(){
//     let mut guess = String::new();
//     println!("Hello, world!");
//
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Unable to read line");
//
//     println!("The value read was: {guess}");
//
//     let mut range = thread_rng();
//     let  rand_number: u32 = range.gen_range(1..=10);
//     println!("Random number generated is : {rand_number}");
// }

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
    generics();
}
