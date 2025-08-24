fn main() {
    simple_if();
    else_if();
    expression_if();
}

fn simple_if() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn else_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }
}

fn expression_if() {
    let condition = true;

    let value = if condition { 5 } else { 6 };

    println!("Value is: {value}")
}
