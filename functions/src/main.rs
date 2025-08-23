fn main() {
    println!("Hello, world!");

    another_function();
    another_function_param(5);
    print_labeled_measurement(5, 'h');

    let a = returns_expression();
    println!("Returned value of a is: {a}");

    let a = plus_one(41);
    println!("Returned value of plus_one a is: {a}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_param(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn returns_expression() -> i32 {
    5
}

fn plus_one(value: i32) -> i32 {
    value + 1
}
