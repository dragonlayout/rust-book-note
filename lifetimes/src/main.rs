fn main() {
    // every reference int Rust has a lifetime, which is the scope for which that reference is valid.
    // like types are referred, wo must annotate lifetimes when the lifetimes of references could be
    // related in a few different ways

    {
        let x = 5;            // ----------+-- 'b
                                   //           |
        let r = &x;         // --+-- 'a  |
                                   //   |       |
        println!("r: {}", r);      //   |       |
                                   // --+       |
    }                              // ----------+

    // generic lifetime in function
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2); // 传入的两个 reference, 但是返回了其中一个 reference
                                                                  // 不能确定 返回的 reference 生命周期 导致 borrow checker 无法
                                                                  // 进行分析
    println!("The longest string is {}", result);

    // lifetime annotation syntax
    // &i32 a reference
    // &'a i32 a reference with an explicit lifetime
    // &'a mut i32 // an mutable reference with an explicit lifetime

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }// string2 is valid
    // string1 is valid
    //
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// return reference is maybe x or y
//
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetime annotation in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}
