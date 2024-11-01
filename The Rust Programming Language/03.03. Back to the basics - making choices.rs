// Back to the basics - making choices

fn main() {
    println!("What is your name?");
    let input = read_string();
    if input == "" {
        println!("You did not enter your name...");
    } else {
        println!("Your name is: {input}");
    }
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    let cleaned_input = input.trim().to_string();
    cleaned_input
}
