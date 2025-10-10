fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("number of seconds in three hours: {THREE_HOURS_IN_SECONDS}");
    let y = 7;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("number of spaces: {spaces}");

    let mut space = "      ";
    // space = space.len(); this will not work variables can not change type
    println!("length and space: {space}");

    // Integers in Rus

    //signed literals
    let k = 75i8;
    println!("{k}");

    let m:i32 = 32; // unsigned is m: u32 = 32
    println!("loggin signed 32 bitt number: {m}");

    //Floats are all signed and either 32 or 64 bit. defaults to 64 bit
    let m:f64 = 78.33;
    println!("floating number: {m}");

    // let value: f64 = 46/10; this will not work. division by integer
    // is an integer
    // println!("{value}");

    //boolean types
    let t = true;
    let f:bool = false; // with explicit type annotation

    // The Character Type
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotations
    let heart_eyed_cat = '😻';

    // Compound types

    // Rust has tuples and arrays as primitive compound types

    //The Tuples Type

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z)  = tup;
    println!("The value of of y is {y}");

    let first = tup.0;
    println!("{first}");

    //arrays
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"
    ];
    let a: [i32;5] = [1, 2, 3, 4, 5];
    let january = months[0];
    println!("{january}");
}
