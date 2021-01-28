use std::collections::HashMap;

fn main() {
    // vector, store a variable number of values next to each other
    // Vec<T>
    // Create a new Vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // vec! is marco

    // Update a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(1);

    // Drop a Vector
    {
        let v = vec![1, 2, 3 ,4];
    } // <- v goes out of scope and is freed here

    // Reading elements of Vector
    let v = vec![1, 2, 3, 4];
    let third: &i32 = &v[2]; // give us a reference
    println!("The third element is {}", third);
    match v.get(2) { // get() give us a Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    println!("The third element is {}", third);
    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // cause program to panic because reference a nonexistent element
    let does_not_exist = v.get(100); // return None without panic

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // immutable borrow
    v.push(6); // mutable borrow
    // adding a new element onto the end of the vector might require allocating new memory
    // and copying the old elements to the new space, if there isn't enough room to put all the
    // elements next to each other where the vector currently is.
    // the reference to the first element would be pointing to the deallocated memory.

    // println!("The first element is {}", first); // error

    // Iterating over values in a Vector
    let mut v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }
    for i in &mut v { // dereference operator in Chapter 15
        *i += 50;
    }

    // Using an Enum to store Multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    // string, a collection of characters
    println!("--- String ---");
    // storing utf-8 encoded text with strings
    // What is a String?

    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // updating a string
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("hello,");
    let s2 = String::from(("world!"));
    let s3 = s1 + &s2;
    println!("s2: {}, s3: {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // using format! macro
    println!("s: {}", s);

    let s1 = String::from("陈龙");
    for b in s1.bytes() {
        println!("{}", b);
    }

    // hash map, associate a value with a particular key
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("key: {}, value: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    let count = scores.entry(String::from("Blue")).or_insert(50);
    println!("count: {}", count);
    println!("{:?}", scores);

    // update a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert return a mut reference to the value
                                                                          // for this key
        *count += 1;
    }
    println!("{:?}", map);


}
