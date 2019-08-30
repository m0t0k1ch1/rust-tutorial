#[derive(Debug)]
enum Message
{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message
{
    fn call(&self)
    {
        // method body would be defined here
    }
}

fn main()
{
    let m1 = Message::Quit;
    let m2 = Message::Move{ x: 0, y: 0 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(0, 0, 0);

    m1.call();
    m2.call();
    m3.call();
    m4.call();
}
