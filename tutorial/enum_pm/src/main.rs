// Enums and Pattern Matching

// IP Addresses
// 1. IPv4;
// 2. IPv6;

fn main() {
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));
    let eight = IpAddrKind::V8(0.0);
    let ten = IpAddrKind::V10;

    route(four);
    route(six);
    route(eight);
    route(ten);

    let five: Option<i32> = Some(5);
    let none: Option<i32> = None;

    println!("{}", add_one(five).unwrap());
    dbg!(add_one(none));
}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(num) => Some(num + 1),
        None => None,
    }
}

fn route(ip: IpAddrKind) {
    match ip {
        IpAddrKind::V4(x, y, z, a) => println!("{x} {y} {z} {a}"),
        IpAddrKind::V6(address) => println!("{address}"),
        _ => println!("Type is not V4 or V6"),
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), // variant
    V6(String), // variant,
    V8(f32), // variant
    V10
}

// enum Option<T> {
//     Some(T),
//     None
// }

