fn main() {
    integer();
    float();
    boolean();
    character();
    tuple();
    array();
}

fn integer() {
    let int: u16 = 65535;
    let dec = 65_355; // decimal
    let hex = 0xffff; // hex
    let bin = 0b1111_1111_1111_1111; // binary
    println!(
        "The value of int, dec, hex, bin is: {}, {}, {}, {}",
        int, dec, hex, bin
    );
}

fn float() {
    let x = 3.14; // f64
    let y: f32 = 3.14; // f32
    println!("The value of x, y is: {}, {}", x, y);
}

fn boolean() {
    let t = true;
    let f: bool = false;
    println!("The value of t, f is: {}, {}", t, f);
}

fn character() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!(
        "The value of c, z, heart_eyed_cat is: {}, {}, {}",
        c, z, heart_eyed_cat
    );
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: ({}, {}, {})", tup.0, tup.1, tup.2);
    let (x, y, z) = tup; // destructuring
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);
}

fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "The value of a is: [{}, {}, {}, {}, {}]",
        a[0], a[1], a[2], a[3], a[4]
    );
}
