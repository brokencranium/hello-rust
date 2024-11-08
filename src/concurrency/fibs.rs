use rayon::prelude::*;
use std::env;
use std::time;

fn fibonacci(n: i8) -> u64 {
    if n < 0 {
        panic!("{} is negative", n)
    }

    match n {
        0 => panic!("Zero is not applicable for fibonacci"),
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_numbers(numbers: Vec<i8>) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    for n in numbers.iter() {
        vec.push(fibonacci(*n));
    }
    return vec;
}

pub fn fibonacci_main() {
    let numbers = vec![3,8, 12, 12, 20, 20, 20, 20, 28, 28, 28, 28, 36];
    let results = fibonacci_numbers(numbers);
    for result in results {
        println!("Result: {}", result)
    }
}

pub fn single_thread() {
    let now = time::Instant::now();
    fibonacci(8);
    fibonacci(12);
    fibonacci(12);
    fibonacci(20);
    fibonacci(20);
    fibonacci(20);
    fibonacci(20);
    fibonacci(28);
    fibonacci(28);
    fibonacci(28);
    fibonacci(28);
    fibonacci(36);
    println!("Fibonaccin single thread: Time elapsed {:?}", now.elapsed());
}

pub fn multi_thread(num_threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
    let now = time::Instant::now();
    let numbers: Vec<i8> = vec![8, 12, 12, 20, 20, 20, 20, 28, 28, 28, 28, 36];
    let outcomes: Vec<u64> = numbers.into_par_iter().map(|n| fibonacci(n)).collect();
    println!("Outcomes {:?}", outcomes);
    println!(
        "Fibonacci threaded {}: Time elapsed {:?}",
        num_threads,
        now.elapsed()
    );
}
