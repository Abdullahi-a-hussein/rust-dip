fn main() {
    for n in 1..101 {
        let fib = fibonacci(n);
        println!("The {n}th Fibonacci number equals to: {fib}");
    }
}

// fn celsius_to_fahrenheit(value: f64) -> f64 {
//     // (C * 9/5) + 32
//     (value * 9.0) / 5.0 + 32.0
// }

// fn fahrenheit_to_celsius(value: f64) -> f64 {
//     ((value - 32.0) * 5.0) / 9.0
// }

// fn fibonacci_recursive(n: u32) -> u32 {
//     if n <= 2 {
//         return 1;
//     }
//     fibonacci_recursive(n - 2) + fibonacci_recursive(n - 1)
// }

fn fibonacci(n: u64) -> u128 {
    let mut counter = 2;
    let mut curr = 1;
    let mut prev = 1;
    while counter < n {
        let temp = curr;
        curr = prev + curr;
        prev = temp;
        counter += 1;
    }
    curr
}
