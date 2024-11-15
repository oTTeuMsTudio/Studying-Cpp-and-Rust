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
    u8::from_str(&input).or(Err("You've entered an invalid age".to_string()))
}
