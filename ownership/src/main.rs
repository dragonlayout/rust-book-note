fn main() {
    // variable scope
    let s = "hello";
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1; // s1 was moved into s2
    // println!("{}, world", s1);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    print!("s1 = {}, s2 = {}", s1, s2);

    let x: u32 = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // ownership and function
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function
    // println!("s: {}", s); // borrow of moved value: `s`. and so s is not longer valid here
    let x = 5;
    makes_copy(x);
    println!("x: {}", x);

    // return values and scope
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back
                                               // which also moves its return value into s3
    println!("s1: {}", s1);
    // println!("s2: {}", s2); borrow of moved value: `s2`
    println!("s3: {}", s3);
    // here, s3 goes out of scope and it dropped. s2 goes out of scope but was moved,
    // so nothing happens. s1 goes out of scope and is dropped.

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // s1 is moved, so calculate_length return (s2, len)
    println!("The length of '{}' is {}.", s2, len);

    // Borrow and Reference
    println!("--- Borrow and Reference ---");
    let s1 = String::from("hello");
    let length = calculate_length_reference(&s1);
    println!("The length of '{}' is {}.", s1, length);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function
    // that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String  { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}