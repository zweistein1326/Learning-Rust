// Common Collections
use std::collections::HashMap;
// Hashmaps

fn main(){ 
    let mut scores = HashMap::new();
    scores.insert(String::from("Gryffindor"), 10);
    scores.insert(String::from("Ravenclaw"), 20);
    let gryffindor = scores.get("Gryffindor").copied().unwrap_or(0);
    println!("{}", gryffindor);
    scores.insert(String::from("Gryffindor"), 50);
    let gryffindor = scores.get("Gryffindor").copied().unwrap_or(0);
    println!("{}", gryffindor);

    scores.insert(String::from("Slytherin"), 30);
    scores.entry(String::from("Slytherin")).or_insert(50);
    let slytherin = scores.get("Slytherin").copied().unwrap_or(0);
    println!("{}", slytherin);

    let text = "hello world wonderful world !";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}