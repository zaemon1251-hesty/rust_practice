use std::cmp::PartialOrd;
use std::fmt::Display;
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let both_integer = Point { x: 5, y: 7 };

    println!("{:?}", both_integer);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    print!("1 new tweet: {}", tweet.summarize());

    mainn();
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
impl Point<f32> {
    fn x(&self) -> &f32 {
        &self.x
    }

    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(dead_code)]
impl<T> Point<T> {
    fn mixup<U, V>(self, other: Point<U>) -> Point<U> {
        Point {
            x: other.x,
            y: other.y,
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("default implement")
    }
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

pub fn notify(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize())
}
// pub fn notify<T: Summary + Display>(item: &T) {
//     // 速報！ {}
//     println!("Breaking news! {}", item.summarize());
// }

#[allow(dead_code)]
fn returns_summarize_displayable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn longest<'a, T: ExactSizeIterator>(x: &'a T, y: &'a T) -> &'a T {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_b<'a>(x: &str, y: &str) -> String {
    // 本当に長い文字列
    let result = String::from("really long string");
    result
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn mainn() {
    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel = String::from("Call me Ishmael. Some years ago...");
    //                                                  "'.'が見つかりませんでした"
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:#?}", i)
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
