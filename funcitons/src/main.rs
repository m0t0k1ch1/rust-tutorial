// Expressions do not include ending semicolons.
// If you add a semicolon to the end of an expression,
// you turn it into a statement, which will then not return a value.

fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    print_value("x", x);
    print_value("y", y);

    let x = five();
    print_value("x", x);

    let x = plus_one(x);
    print_value("x", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_value(name: &str, v: i32) {
    println!("The value of {} is: {}", name, v);
}
