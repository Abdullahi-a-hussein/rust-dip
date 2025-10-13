fn main() {
    println!("Hello, world!");
    another_function();
    second_function(10);
    looper();
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
     
}
fn another_function() {
    println!("Another function");
}

fn second_function(x: i32) {
    println!("The value of x is: {x}");
}

fn looper() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn five() -> i32 {
    5
}