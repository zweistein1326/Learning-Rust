/* 
    Structs
    - similar to tupes, both hold mulitple related values. Unlike tuples, we will name each pieve of data so it's clear what the values mean. 
    Which means structs are more flexible than tuples.
    - Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides
    but don't have names associated with their fields; rather, they just have the types of the fields.
*/

fn main() {
    let x = (0, 0.2, 'z');
    println!("{} {} {}", x.0, x.1, x.2);

    let mut user1 = User {
        active: true,
        username: String::from("Alice"),
        email: String::from("alice@test.com"),
        sign_in_count: 0,
        admin: false
    }; 

    println!("Username of user1 is {}", user1.username);
    user1.increment_sign_in_count();
    println!("Sign in count of user1 is: {}", user1.sign_in_count);

    // user2 all the same values except username
    let mut user2 = User {
        username: String::from("Bob"),
        ..user1
    };
    println!("Username of user2 is {}", user2.username);
    println!("Email of user2 is {}", user2.email);
    user2.increment_sign_in_count();
    println!("Sign in count of user2 is: {}", user1.sign_in_count);

    println!("Is User2 an admin: {}", user2.admin);
    user2.change_admin(true);
    println!("Is User2 an admin: {}", user2.admin);

    println!("{:#?}", user2);
    dbg!(user2);

    tuple_struct();
}

// Normal Struct
#[derive(Debug)]
struct User {
    // user information
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
    admin: bool
}

impl User {
    fn increment_sign_in_count(self: &mut Self) {
        self.sign_in_count += 1;
    }

    fn change_admin(&mut self, is_admin: bool) {
        self.admin = is_admin;
    }
}

// Tuple Struct

struct Color(i32, i32, i32); // R, G, B
struct Point(f32, f32, f32); // X, Y, Z

fn tuple_struct() {
    let black = Color(0,0,0);
    let origin = Point(0.0,0.0,0.0);
    let red = Color(255,0,0);
    println!("R = {} G = {} B = {}", black.0, black.1, black.2);
    println!("X = {} Y = {} Z = {}", origin.0, origin.1, origin.2);
    println!("Red = {} {} {}", red.0, red.1, red.2);
}

// Unit-like struct

struct AlwaysEqual;
