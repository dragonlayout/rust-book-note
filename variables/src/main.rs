use std::net::Shutdown::Read;

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 7; // Shadowing the first variable x
    // difference between mut and shadowing
    // 1. 保持 immutable
    // 2. 使用同一名字, 还可以改变类型
    let space = "   ";
    let space = space.len();

    // scalar types
    // integer type: signed, unsigned
    // Floating-Point Types: f32, f64(default)
    // char: Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("size: {}", c.len_utf8());
    println!("size: {}", z.len_utf8());
    println!("size: {}", heart_eyed_cat.len_utf8());

    // Compound Types
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y ,z) = tup;
    println!("{} {} {}", x, y ,z);
    println!("tup.0: {}", tup.0);

    // The Array Type
    let a = [1, 2, 3, 5];
    println!("{}", a[0]);

    another_function();

    // statements: no return value
    // Expressions: has return value

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    let x = five();
    println!("The value of x is {}", x);

    if_expressions();

    loop_expressions();

    while_expressions();

    while_array();

    for_array();
}

fn another_function() {
    println!("Another function!");
}

fn five() -> i32 {
    return 5;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn if_expressions() {
    let number = 3;
    if number > 3 {
        println!("Condition is True");
    } else {
        println!("Condition is False");
    }
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is {}", number)
}

fn loop_expressions() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn while_expressions() {
    let mut number = 5;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!")
}

fn while_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is {}", a[index]);
        index += 1;
    }
}

fn for_array() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is {}", element);
    }
}