# Exercise
Rewrite the error message to a more generic "You've entered an invalid number. Please enter a value between 0 and 255."

Now that we are returning a more generic error message, we can use the read_number() function to request more numbers than just age, and print the same error if needed.

Since we are returning the error message from the read_number() function, we can now also distinguish between an invalid input and no input.

use std::str::FromStr;

fn main() {
    println!("How old are you?");
    match read_number() {
        Ok(age) => println!("You are {age} years old."),
        Err(err) => println!("{err}"),
    }
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input.trim().to_string()
}

fn read_number() -> Result<u8, String> {
    let input = read_string();

    if input.is_empty() {
        Err("You did not enter any data".to_string())
    } else {
        u8::from_str(&input).or(Err("You've entered an invalid number".to_string()))
    }
}

We test input.is_empty() to check if the user did not enter any data. We use this in an if block to differentiate between no input and some input.

Notice the lack of ; in the last five lines. This ensures that the result of the if block is returned by the read_number() function.
