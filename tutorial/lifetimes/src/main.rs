// Lifetimes

fn main() { // ------b------
    // let a = String::from("Hello World!");
    let a = 5;
    let s = 10;
    // let r: &str;
    let r: &u32;
    {
    //------a------
        // let s = String::from("How are you doing?");
        // r = longer(&a, &s);
        r = greater(&a, &s);
        // ------a-------
    } // s is no longer valid => r is holding a reference to an empty value and this will throw garbage values. Rust cathces all these kind of errors.
    println!("{}", r);
    // -------b--------
}

pub fn greater<'a, 'b: 'a>(a: &'a u32, s: &'b u32) -> &'a u32 {
    if a > s {
        a
    } else {
        s
    }
}