fn main() {
    let mut my_nickname = "Theo";
    greet(my_nickname);
    goodbye(my_nickname);

    my_nickname = "oTTeuM";
    greet(my_nickname);
    goodbye(my_nickname);

    my_nickname = "truenjenir";
    greet(my_nickname);
    goodbye(my_nickname);

    my_nickname = "Teo Goli";
    greet(my_nickname);
    goodbye(my_nickname);
}
fn greet(my_nickname: &str) {
    println!("{my_nickname}! I greet you.");
    println!("Welcome to the world of Rust!");
}
fn goodbye(my_nickname: &str) {
    println!("Goodbye {my_nickname}!");
}
