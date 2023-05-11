#![allow(non_camel_case_types, dead_code, unused_variables)]
// Only allowing for rust-book sandboxing

#[derive(Debug)]
enum IpAddrKind_Simple {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr_Struct {
    kind: IpAddrKind_Simple,
    address: String,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: isize, y: isize },
    Write(String),
    ChangeColor(isize, isize, isize),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl Coin {
    fn value(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn main() {
    let four = IpAddrKind_Simple::V4;
    let six = IpAddrKind_Simple::V6;
    let home = IpAddr_Struct {
        kind: IpAddrKind_Simple::V4,
        address: String::from("127.0.0.1"),
    };
    let home2 = IpAddr::V4(String::from("127.0.0.1"));
    route(IpAddrKind_Simple::V4);
    route(IpAddrKind_Simple::V6);

    let m = Message::Write(String::from("hello"));
    m.call();

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn route(ip_kind: IpAddrKind_Simple) {}
