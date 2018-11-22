fn main() {
    integer();
    float();
    boolean();
    character();
}

fn integer() {
    let int: u16 = 65535;
    println!("The value of int is: {}", int);
    let dec = 65_355; // decimal
    println!("The value of dec is: {}", dec);
    let hex = 0xffff; // hex
    println!("The value of hex is: {}", hex);
    let bin = 0b1111_1111_1111_1111; // binary
    println!("The value of bin is: {}", bin);
}

fn float() {
    let x = 3.14; // f64
    println!("The value of x is: {}", x);
    let y: f32 = 3.14; // f32
    println!("The value of y is: {}", y);
}

fn boolean() {
    let t = true;
    println!("The value of t is: {}", t);
    let f: bool = false;
    println!("The value of f is: {}", f);
}

fn character() {
    let c = 'z';
    println!("The value of c is: {}", c);
    let z = 'ℤ';
    println!("The value of z is: {}", z);
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);
}
