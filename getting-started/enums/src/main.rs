enum IpAddrKind {
    V4,
    V6,
}

// Putting value into enums

enum Ipaddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //method body would be defined here
    }
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alaska => year >= 1959,
            UsState::Alabama => year >= 1819,
            // --snip--
        }
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = Ipaddr::V4(String::from("127.0.0.1"));
    let loopback = Ipaddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
    let val = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{val}");

    // Catch all patterns and the _ placeholder

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_famcy_hat(),
        other => move_player(other),
    }
    // catch-all pattern but no need to use the value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_famcy_hat(),
        _ => reroll(),
    }

    // catch all pattern but nothing needs to happen

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_famcy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_famcy_hat() {}
    fn move_player(num_spaces: u8) {}

    fn reroll() {}
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
