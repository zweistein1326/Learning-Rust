fn main() {
    // Return the first word in a string;
    let mut s = String::from("hello world");
    let first_word: &str = first_word(&s);
    s.clear();
    // let first_word = &s[0..length];
    println!("The first word of {s} is {first_word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}