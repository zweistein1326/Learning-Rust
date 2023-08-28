fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = 1_000;

    let result = another_function(x, y);
    println!("The result is {result}");
}

fn another_function(x: u32, y: u32) -> u64 {
    // add two numbers
    let result = x + y;
    // return result;
    result.into()
}
