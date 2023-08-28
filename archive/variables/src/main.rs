fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    let result = add_one(x);
    println!("The value of result is: {}", result);
}

fn add_one(x: i32) -> i32 {
    x + 1
}