fn main() {
    println!("Hello, world!");

    // Thre kinds of loops
    // 1.
    let mut counter = 0;
    loop {
        if counter > 5 {
            break;
        }
        println!("Again!");
        counter = counter + 1;
    }

    let mut new_counter = 0;
    let result = loop {
        new_counter += 1;
        if new_counter == 10 {
            break new_counter * 2;
        }
    };
    println!("The result is {result}");
    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    // 2.
    for x in 1..7 {
        println!("The value of x: {x}");
    }
    while_loops();
    for_looping();
}

fn while_loops() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_looping() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
