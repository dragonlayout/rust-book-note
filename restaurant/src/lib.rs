mod front_of_house; // tells Rust to load the contents of the module from another file with the
                    // same name as the module

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//
//         fn seat_at_table() {}
//     }
//
//     mod serving {
//         fn take_over() {}
//
//         fn serve_order() {}
//
//         fn take_payment() {}
//     }
// }
//
// use crate::front_of_house::hosting;         // when bring a name into scope with `use` keyword, the name available int the new scope is private.
// // pub use crate::front_of_house::hosting;  // re-exporting
// use std::collections::HashMap;
// use std::{fmt};
// use std::io::Result as IoResult;
//
// pub fn eat_at_restaurant() {
//     // absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     // relative paht
//     front_of_house::hosting::add_to_waitlist();
//
//     // order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//
//     // let mut meal = back_of_house::Breakfast::winter(this);
//     // change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
//
//     // bring paths into scope with the `use` keyword
//     hosting::add_to_waitlist();
//
//     let mut map = HashMap::new();
//     map.insert(1, 2);
//
//
//     fn function1() -> fmt::Result {
//         Ok(())
//     }
//
//     fn function2() -> IoResult<()> {
//         Ok(())
//     }
// }
//
// fn serve_order() {}
//
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }
//
//     pub enum Appetizer { // all variants are public
//         Soup,
//         Salad,
//     }
//
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast { // Associated function, as a construction
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
//
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }
//
//     fn cook_order() {}
// }