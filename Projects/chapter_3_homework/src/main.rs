use std::io;

fn main() {
    // Convert temperatures between Fahrenheit and Celsius.
    loop {
        println!("Print the temperature in Fahrenheit and I'll convert it to Celsius.");

        let mut temp_far = String::new();

        io::stdin()
            .read_line(&mut temp_far)
            .expect("Failed to read input");

        let temp_far: f64 = match temp_far.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let temp_cel = (temp_far-32.0)/1.8;

        println!("{temp_far} Fahrenheit is {temp_cel:.2} in Celsius");
        break;
    }
    // Generate the nth Fibonacci number.
    loop {
        println!("Provide an N to find Nth fibonacci number");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read input");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut a = 0;
        let mut b = 1;

        for _i in 1..number {
            let c = a + b;
            a = b;
            b = c;
        }

        println!("{b} is a {number}th fibonacci number");
        break;
    }
}
