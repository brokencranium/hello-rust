use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

// Command struct is used to spawn off an new process
// Stdio struct is used to define the piping of data back from process
// Child struct is returned when the process is spawned

fn spawn_process(inputs: &[&str]) -> Child {
    return Command::new("./fib_concurr")
        .args(inputs)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute the process");
}

fn spawn_main() {
    let mut one = spawn_process(&["5", "6", "7", "8"]);
    let mut two = spawn_process(&["9", "10", "11", "12"]);
    let mut three = spawn_process(&["13", "14", "15"]);

    one.wait();
    two.wait();
    three.wait();

    let one_stdout = one.stdout.as_mut().expect("Unable to open stdout of child");
    let two_stdout = two.stdout.as_mut().expect("Unable to open stdout of child");
    let three_stdout = three
        .stdout
        .as_mut()
        .expect("Unable to open stdout of child");

    let one_data = BufReader::new(one_stdout);
    let two_data = BufReader::new(two_stdout);
    let three_data = BufReader::new(three_stdout);

    let mut results = Vec::new();

    for i in three_data.lines() {
        results.push(i.unwrap().parse::<i32>().unwrap());
    }

    for i in two_data.lines() {
        results.push(i.unwrap().parse::<i32>().unwrap());
    }

    for i in one_data.lines() {
        results.push(i.unwrap().parse::<i32>().unwrap());
    }

    println!("{:?}", results);
}
