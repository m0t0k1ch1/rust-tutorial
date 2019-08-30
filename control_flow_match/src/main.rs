#[derive(Debug)] // so we can inspect the state in a minute
enum UsState
{
    Alabama,
    Alaska,
    // --snip--
}

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8
{
    match coin
    {
        Coin::Penny          => { println!("Lucky penny!"); 1 },
        Coin::Nickel         => 5,
        Coin::Dime           => 10,
        Coin::Quarter(state) => { println!("State quarter from {:?}!", state); 25 },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>
{
    match x
    {
        None    => None,
        Some(i) => Some(i + 1),
    }
}

fn main()
{
    println!("1 penny is {} cents.",   value_in_cents(Coin::Penny));
    println!("1 nickel is {} cents.",  value_in_cents(Coin::Nickel));
    println!("1 dime is {} cents.",    value_in_cents(Coin::Dime));
    println!("1 quarter is {} cents.", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("1 quarter is {} cents.", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);

    match six
    {
        Some(i) => println!("six is {}", i),
        None    => println!("six is none"),
    };
    match none
    {
        Some(i) => println!("none is {}", i),
        None    => println!("none is None"),
    };

    let some_u8_value = 0u8;

    match some_u8_value
    {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    };
}
