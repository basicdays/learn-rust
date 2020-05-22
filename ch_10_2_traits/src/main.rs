#![allow(dead_code, unused_variables)]
use std::fmt::{Debug, Display};

fn main() {
    pub trait Summary {
        fn summarize_author(&self) -> &str;

        fn summarize(&self) -> String {
            format!("(Read more from {}...", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> &str {
            &self.author
        }

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
        fn summarize_author(&self) -> &str {
            &self.username
        }

        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub struct BlogPost {
        pub content: String,
    }

    impl Summary for BlogPost {
        fn summarize_author(&self) -> &str {
            "interblogs"
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    pub fn notify(item: impl Summary + Display) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify2<T: Summary + Display>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        10
    }

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

fn part2() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // conditionally implement method if Pair<T> has certain trait
    // restrictions
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
