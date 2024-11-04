fn main() {
    let mut first_name = "Marcel";
    greet(first_name);

    first_name = "Tom";
    greet(first_name);

    first_name = "Dick";
    greet(first_name);

    first_name = "Harry";
    greet(first_name);
}

fn greet(first_name: &str) {
    println!("{first_name}! I greet you.");
}
