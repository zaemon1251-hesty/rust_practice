pub mod ch05;
pub mod ch06;
pub mod ch08;

#[allow(unused_imports)]
use std;

fn main() {
    let mut x = five_or_four();
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    another_function();

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let _ = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let _ = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("{}", word);

    ch05::user_print();
    ch05::main();

    ch06::main();
    ch08::main();
}

fn another_function() {
    println!("Another function");
}

fn five_or_four() -> u32 {
    let mut res = 0;

    for i in 1..6 {
        println!("cnt: {}", i);
    }

    'outer: loop {
        loop {
            if res == 5 {
                break 'outer;
            }
            if res == 6 {
                break;
            }
            res += 1;
        }
        println!("unexpected")
    }

    res
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            //予想値は1から100の間でなければなりませんが、{}でした。
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
