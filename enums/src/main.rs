fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        _ => (), // unit value, so nothing happens
    }

    // Concise control flow with `if let`
    let some_u8_value = Some(3u8);
    if some_u8_value == Some(3) {
        println!("three");
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
    if let Some(value) = some_u8_value {
        println!("the matching value is {}", value);
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn route(ip_kind: IpAddrKind) {
// }

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
