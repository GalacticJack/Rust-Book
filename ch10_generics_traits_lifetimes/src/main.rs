#![allow(unused_variables, dead_code)]

use crate::traits::{NewsArticle, Summary, Tweet};
use std::println;

fn largest_non_generic(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Extracting reused code into function(largest).
fn use_largest_non_generic() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_non_generic(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_non_generic(&number_list);
    println!("The largest number is {}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn generics() {
    // Using genericly typed structs.
    let point = Point { x: 1, y: 3 };
    println!("p.x = {}", point.x());
    let float = Point { x: 1.4, y: 3.8 };
    println!("distance_from_origin = {}", float.distance_from_origin());
    // Extracting reused code into function.
    use_largest_non_generic()
}

pub mod lifetimes;
pub mod traits;

fn main() {
    lifetimes::run();
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());

    generics();
}
