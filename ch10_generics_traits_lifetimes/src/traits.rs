use std::{format, println};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Display {}
pub trait Example {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// These two functions are the same after compilation
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify_traitbound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying multiple trait bouds with the + Syntax
pub fn notify_multi_trait(item: &(impl Summary + Display)) {}
pub fn notify_multi_trait_bound<T: Summary + Display>(item: &T) {}

// Clearer trait bounds with where clauses
pub fn some_function<T: Display + Clone, U: Clone + Example>(t: &T, u: &U) {} // Example of unclear
pub fn some_clearer_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Example,
{
}

// Returning types that implement traits
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
