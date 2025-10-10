fn main() {
    println!("Hello, world!");
    another_function();
    second_function(10);
}
fn another_function() {
    println!("Another function");
}

fn second_function(x: i32) {
    println!("The value of x is: {x}");
}