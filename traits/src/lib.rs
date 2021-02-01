fn main() {
    // Traits: Defining shared behavior
    // tells Rust compiler about functionality a particular type has and can share with other types.
    // similar `interface`
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people",),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);

    returns_summarizable().summarize();
}
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait Display {

}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify_v1(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_v2<T: Summary>(item1: &T, item2: &T) { // item1 and item2 must be the same type
}

pub fn notify_v3(item: &(impl Summary + Display)) {} // multiple trait

pub fn notify_v4<T: Summary + Display>(item: &T) {}

// returning type that implement trait
// closures and iterators
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    }
}