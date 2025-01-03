fn main() {
    let x = 5; // notice that this val is non mut

    let x = x + 1;

    { // start of another scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    } // at the end of this scope, it's overshadowing disappears.

    println!("The value of x is: {x}");
}
