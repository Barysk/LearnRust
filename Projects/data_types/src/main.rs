fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = x;
    
    // ^ the same
    //let a = x.0;
    //let b = x.1;
    //let c = x.2;

    println!("{a}, {b}, {c}");
}
