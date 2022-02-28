fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;    
    println!("The value of x is: {}", x);
    another_function(5);
    print_labeled_measurement(3, 'm');
    print!("Expression test returns: {}", expression_test());
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn expression_test() -> i32 {
    let x = 4;
    x
}