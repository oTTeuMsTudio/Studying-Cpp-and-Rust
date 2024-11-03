use std::io::{stdin, Read};

struct Address {
    street: String,
    house_number: i16,
    postal_code: String,
    city: String,
}

struct Person {
    first_name: String,
    last_name: String,
    address: Address,
}

impl Person {
    fn new(first_name: String, last_name: String, address: Address) -> Person {
        Person {
            first_name,
            last_name,
            address,
        }
    }

    fn new_from_input() -> Person {
        println!("What is your first name?");
        let first_name = read_string();
        println!("What is your last name?");
        let last_name = read_string();
        println!("What is your street name?");
        let street = read_string();
        println!("What is your house number?");
        let house_number: i16 = read_i16();
        println!("What is your postal code?");
        let postal_code = read_string();
        println!("What is your city?");
        let city = read_string();

        let address = Address {
            street,
            house_number,
            postal_code,
            city,
        };

        Person::new(first_name, last_name, address)
    }
}

fn read_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn read_i16() -> i16 {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}

fn main() {
    let person = Person::new_from_input();
    println!("Hello, {} {}!", person.first_name, person.last_name);
    println!("Your address is: {} {} {} {}", person.address.street, person.address.house_number, person.address.postal_code, person.address.city);
}
