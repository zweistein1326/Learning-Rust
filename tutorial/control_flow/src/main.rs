/*
    Control Flow
        - if expressions
        - if in a let statement
        - repetitions with loops
        - returning values from loops
        - loop labels
        - while loop
        - for loop

*/

fn main() {
    let mut number = 3;
    if number < 10 {
        println!("number is less than 10");
    } else {
        println!("number is greater than or equal to 10");
    }

    let result = if number < 10 {100} else {0};
    println!("Result = {result}");    

    // looping
    'counting_loop: loop {
        loop {
            println!("hello");
            if number > 10 {
                break 'counting_loop;
            } else {
                println!("In Small Loop");
                number = number + 1;
            }
        }
        if number > 20 {
            break;
        } else {
            println!("In Big Loop");
            number = number + 1;
        }
    };
    
    println!("Number = {number}");
    let result = loop {
        number = number + 2;
        if number > 40 {
            break number * 2;
        }
    };
    println!("The result is {result}");

    // while loops

    let mut x = 3;

    while x != 0 {
        println!("{x}");
        x = x - 1;
    }

    println!("COMPLETE {x}");

    // for loops
    let arr: [u8; 4] = [0, 1, 2, 3];
    for y in arr {
        println!("{y}");
    }
    println!("COMPLETE");

}
