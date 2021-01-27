use std::borrow::Borrow;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = User {
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        sign_in_count: 1,
        active: true
    };
    let user2 = User {
        username: String::from("another@example.com"),
        email: String::from("anotherusername123"),
        ..user1
    };

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}