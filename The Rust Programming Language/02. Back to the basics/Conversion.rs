fn main() {
    println!("How old are you?");
    let age = read_string();
    println!("You are {age} years old.");
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input.trim().to_string()
}
