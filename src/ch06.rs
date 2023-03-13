enum IpAddr {
    V4(String),
    V6(String),
}

fn route(ip_addr: IpAddr) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn pls_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let absent_number: Option<i32> = Some(21);

    println!("{:?}", pls_one(absent_number));
}

fn iflet() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
