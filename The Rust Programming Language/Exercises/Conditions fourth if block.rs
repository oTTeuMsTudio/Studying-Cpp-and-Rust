fn main() {
    println!("What is your name?");
    let input = read_string();
    if input == "" {
        println!("You did not enter your name...");
    } else if input == "oTTeuM" || input == "otteum" {
        println!("{input} is a great name!");
    } else if input == "oTTe" || input == "otte" {
        println!("I love the name {input}!");
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
