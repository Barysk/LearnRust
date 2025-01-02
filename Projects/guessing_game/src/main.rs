use rand::Rng;
use std::cmp::Ordering;
use std::io; // input/output library. io library comes from the standard library, known as std.

// NOTE: You won’t just know which traits to use and which methods and functions to call from a
// crate, so each crate has documentation with instructions for using it. Another neat feature of
// Cargo is that running the cargo doc --open command will build documentation provided by all your
// dependencies locally and open it in your browser. If you’re interested in other functionality in
// the rand crate, for example, run cargo doc --open and click rand in the sidebar on the left.

// An entry point
fn main() {
    // macros to output information on screen
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_number}");

    loop {
        println!("Input your guess.");

        // var that storing users input
        // it is mutable
        // the value is bound to the result of calling 'String::new' function that returns a new
        // instance of a String. String is a string type provided by the standard library that is a
        // growable, UTF-8 encoded bit of text.
        //
        // The :: syntax in the ::new line indicates that new is an associated function of the String
        // type. An associated function is a function that’s implemented on a type, in this case
        // String. This new function creates a new, empty string. You’ll find a new function on many
        // types because it’s a common name for a function that makes a new value of some kind.
        let mut guess = String::new();
        // let is used to create a variable example:
        // let apples = 5;
        // This line creates a var named apples and binds it to the value 5. Variables are immutable by
        // default in Rust, meaning once we give the variable a value the value won't change.
        // to make variable mutable add 'mut'
        // let apples = 5; // immutable
        // let mut bananas = 5; // mutable
        //
        // The let mut guess = String::new(); line has created a mutable variable that is currently
        // bound to a new, empty instance of a String.

        // If we hadn’t imported the io library with use std::io; at the beginning of the program, we
        // could still use the function by writing this function call as std::io::stdin.
        io::stdin()
            // The full job of read_line is to take whatever the user types into standard input and
            // append that into a string (without overwriting its contents), so we therefore pass that
            // string as an argument. The string argument needs to be mutable so the method can change
            // the string’s content.
            // We need to write &mut guess rather than &guess to make it mutable. since references are
            // immutable by default
            .read_line(&mut guess)
            // read_line puts whatever the user enters into the string we pass to it, but it also
            // returns a Result value. Result is an enumeration, often called an enum, which is a type
            // that can be in one of multiple possible states. We call each possible state a variant.
            //
            // Result’s variants are Ok and Err. The Ok variant indicates the operation was successful,
            // and inside Ok is the successfully generated value. The Err variant means the operation
            // failed, and Err contains information about how or why the operation failed.
            .expect("failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // there is a better way handling error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess);
        // Best to see an example:
        // let x = 5;
        // let y = 10;
        // println!("x = {x} and y + 2 = {}", y + 2);
        //
        // This code would print x = 5 and y + 2 = 12

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
