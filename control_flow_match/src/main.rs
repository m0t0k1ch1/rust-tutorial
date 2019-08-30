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

fn main()
{
    println!("1 penny is {} cents.",   value_in_cents(Coin::Penny));
    println!("1 nickel is {} cents.",  value_in_cents(Coin::Nickel));
    println!("1 dime is {} cents.",    value_in_cents(Coin::Dime));
    println!("1 quarter is {} cents.", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("1 quarter is {} cents.", value_in_cents(Coin::Quarter(UsState::Alaska)));
}
