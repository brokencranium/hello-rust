use std::f64::consts::PI;

/// https://doc.rust-lang.org/std/fmt/

/// format!: write formatted text to String
/// print!: same as format! but the text is printed to the console (io::stdout).
/// println!: same as print! but a newline is appended.
/// eprint!: same as print! but the text is printed to the standard error (io::stderr).
/// eprintln!: same as eprint! but a newline is appended.
pub(crate) fn print_examples() {
    println!("Days in January: {}", 30);
    println!("Days in {}: {}", "January", 31);
    println!("Days in {0}: {1}", "January", 31);
    println!("Days in {month}: {days}", month = "January", days = 31);

    println!("Base 10: {}", 777);
    println!("Base 2: {:b}", 777);
    println!("Base 8: {:o}", 777);
    println!("Base 16: {:x}", 7777);
    println!("Base 16: {:X}", 7777);

    println!("Leading zeros: {:0>3}", 7);
    println!("Leading zeros: {number:0>3}", number = 7);
    println!("Leading zeros: {number:0>3}", number = 7);
    println!("Trailing zeros: {number:0<3}", number = 7);
    println!("Trailing zeros: {number:0<width$}", number = 7, width = 3);


    println!("Whitespace: {number:>width$}", number = 1.0, width = 3);
    println!("Hello {1:0$}!", 5, "x");

    println!("PI decimals: {pi:.3}",pi=PI);

    println!("{debug_val:?}", debug_val="test");
}