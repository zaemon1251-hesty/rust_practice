use std::fmt::Formatter;

pub fn main() {
    let rect1 = Rectangle {
        width: 43,
        height: 43,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::square(60);

    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "username: {},\n\temail: {},\n\tsign_in_count: {},\n\tactive: {}",
            self.username, self.email, self.sign_in_count, self.active
        )
    }
}

pub fn user_print() {
    let email = String::from("aaa_example@gmail.com");
    let username = String::from("aaa");
    let sign_in_count: u64 = 125;
    let active: bool = true;

    let user = User {
        email,
        username,
        sign_in_count,
        active,
    };

    println!("User: \n{{\n\t{}\n}}", user);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
