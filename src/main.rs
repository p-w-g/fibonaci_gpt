use std::io;

fn main() {
    println!("Hello, world!");
    println!("welcome to calc your fibonaccies!");
    println!("Please input your N-th number.");

    let mut input = String::new();

    println!("Please type a number between 0 to something");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let input: usize = input.trim().parse().expect("Please type a number!");

    let fib_result = fibonacci(input);
    println!("Your numberses are: {fib_result}");
}

fn fibonacci(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
