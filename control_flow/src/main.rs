fn main() {
    if_plain();
    if_let();
    loop_let();
    while_plain_1();
    while_plain_2();
    for_array();
    for_range();
}

fn if_plain() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4")
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4, 3 or 2")
    }
}

fn if_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number)
}

fn loop_let() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}

fn while_plain_1() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

fn while_plain_2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
}

fn for_array() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
