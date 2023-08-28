// Generic Types

fn main() {
    let nums: Vec<i32> = vec![0,1,2,3,4,5];
    let chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    println!("{}", largest(&nums));
    println!("{}", largest(&chars));

    let point1 = Point{
        x: 5,
        y:10
    };

    let point2 = Point{
        x: 9.0,
        y: 18.0
    };
    println!("{}", point1.x());
    println!("{}", point2.x());
}

pub fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        } else {
            // do nothing
        }
    }
    largest
}

pub enum Option<T> {
    Some(T),
    None
}

pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// pub fn largest_chars(list: &Vec<T>) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         } else {
//             // do nothing
//         }
//     }
//     largest
// }