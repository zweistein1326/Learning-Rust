fn main() {
    // Data Types

    // Scalar Types
    // Integer types
    // i8, u8
    // i16. u16,
    // i32, u32,
    // i128, u128,
    // isize, usize
    // Floating-point types
    // f32, f64
    let x: f32 = 2.0;
    let y: i32 = 2;
    // Boolean Type -> true or false
    let z: bool = true;
    let a: bool = false;
    // Character Type
    let b: char = 'b';
    let c: &str = "banana";

    // Compound Types
    // Tuple Type
    let tup :(i32, f64, u8) = (500, 0.8, 2);
    
    // Destructuring our tuples in Rust
    // let (d, e, f) = tup;
    
    let d = tup.0;
    let e = tup.1;
    let f = tup.2;
    println!("d = {d}, e = {e}, f = {f}");

    // Array Type
    let arr : [i32; 5] = [5, 10, 15, 20, 25];
    println!("The third element of arr is {}", arr[2]);
    
}
