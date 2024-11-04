Let's revisit one of the earlier examples. We've used this code to explore the use of functions:

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
As you can see there is a lot of repetition in the main function. We're calling greet four times in the same way, only passing in a different first_name. Luckily Rust allows us to create a list of items, a so-called vector, or Vec. Once we have such a list, we can use a loop to iterate through these items and execute a block of code using the item as an input parameter. Here's how the example will look when using a list:

fn main() {
    let list_of_names = vec!["Marcel", "Tom", "Dick", "Harry"];
    for first_name in list_of_names {
        greet(first_name);
    }
}

fn greet(first_name: &str) {
    println!("{first_name}! I greet you.");
}
The first thing we do is create a list of names. If you copy this example in the RustRover editor, you will see that list_of_names is of type Vec<&str>. In human speech: a list of strings.

The vec![] statement is a useful Rust helper to quickly create a vector of items of the same type.

We then use the for ... in ... { } statement to run through each of the names. The for loop is executed four times, on every pass first_name is updated to hold the next value from the list_of_names. This is how we can greet four times with a different name.

The for in statement can be used to iterate through nearly any sequence of items. Here are a few examples:

fn main() {
    println!("I count from 1 to 9: ");

    for number in 1..10 {
        print!("{number},")
    }
}
fn main() {
    println!("And I count from 1 to 10: ");
    for number in 1..=10 {
        print!("{number},")
    }
}
fn main() {
    println!("I can spell 'Marcel':");
    for letter in "Marcel".chars() {
        print!("{letter},")
    }
}
The .. and ..= operators are a common way to create a sequence of numbers to run through.

Exercise
Amend the above example to count from 10 to 20.

A sequence is just another type. Like other types it has a number of methods that are useful to change the iteration logic. There is a rev() method to reverse the order:

fn main() {
    println!("I count from 10 to down to 1:");
    for number in (1..=10).rev() {
        print!("{number},")
    }
}
Exercise
Knowing that there is a step_by(..) method on the number sequence. Can you change one of the above examples to count from 1 to 10, skipping all even numbers?

The result of the rev() method is another sequence, so you can chain operations together. Like:

fn main() {
    println!("I count from 9 to down to 1:");
    for number in (1..=10).rev().skip(1) {
        print!("{number},")
    }
}
You are encouraged to call other functions inside the for loop as complexity increases:

fn main() {
    for number in 1..=10 {
        odd_or_even(number);
    }
}

fn odd_or_even(number: i32) {
    if number % 2 == 0 {
        println!("{number} is an even number");
    } else {
        println!("{number} is an odd number");
    }
}
The number % 2 computes the remainder of number divided by 2. If the remainder is 0, we know we have an even number.

Lists or Vectors
As seen above, we can use the vec![] statement to quickly create a Vector (= list) of items. It can be used to create a list of predefined numbers, like:

fn main() {
    let list_of_numbers = vec![10, 25, 8, 14, 3, 42];
    println!("I can list these numbers:");
    for number in list_of_numbers {
        print!("{number},")
    }
}
Once a Vec is created, you can use its methods to manipulate the list of items. Run this example to see what it prints:

fn main() {
    let mut list_of_numbers = vec![10, 25, 8, 14, 3, 42];
    list_of_numbers.remove(0);
    list_of_numbers.push(59);

    println!("I can list these numbers:");
    for number in list_of_numbers {
        print!("{number},")
    }
}
Don't forget the mut keyword if you want to change the Vector.

Exercise
Amend the example and manipulate the list in a few different ways. See what happens if you try to remove an item that is beyond the length of the Vector.

Note that borrowing values also applies to for loops. So this code will fail to compile:

This code does not compile!
fn main() {
    let list_of_numbers = vec![10, 25, 8, 14, 3, 42];
    println!("I can list these numbers:");
    for number in list_of_numbers {
        print!("{number},")
    }

    println!();
    println!("Oops, I can not list these numbers again:");
    for number in list_of_numbers {
        print!("{number},")
    }
}
The reason is simple if you know what is happening. Going back to the cabinet with the drawers. Imagine that each number in the list_of_numbers is in a separate drawer in our cabinet; 6 drawers in total. During the first for loop we go through the drawers, taking the number from the drawer and using it in the print! function. This means that once the first for loop finishes all the drawers are empty.

What we want in this case is to borrow the numbers during the first loop. We don't care about taking them in the second loop, because we have no further use for the numbers anyway. So with a small change, our code is fine again:

fn main() {
    let list_of_numbers = vec![10, 25, 8, 14, 3, 42];
    println!("I can list these numbers:");
    for number in &list_of_numbers {
        print!("{number},")
    }

    println!();
    println!("Now, I *can* list these numbers again:");
    for number in list_of_numbers {
        print!("{number},")
    }
}
See the & in front of the list_of_numbers indicating that we want to borrow these items, rather than take them.

If you revisit the error message on the earlier example, you will see that there is actually a very useful suggestion embedded in the output:

help: consider borrowing to avoid moving into the for loop: `&list_of_numbers`
It is embedded in this block:

4   |     for number in list_of_numbers {
    |                   ---------------
    |                   |
    |                   `list_of_numbers` moved due to this implicit call to `.into_iter()`
    |                   help: consider borrowing to avoid moving into the for loop: `&list_of_numbers`
The number '4' in front of the block points to the line number for which the fix is suggested.

As you can see, this is exactly the fix we've implemented; replace list_of_numbers on line 4 with &list_of_numbers.

It is worthwhile to explore the compiler error messages. They often suggest a fix for the issue at hand.

Make sure that when you borrow values from a list in a loop, any function that is used inside the for block must also borrow the value:

fn main() {
    let list_of_numbers = vec![10, 25, 8, 14, 3, 42];
    println!("I can list these numbers:");
    for number in &list_of_numbers {
        print_number(number)
    }
}

fn print_number(number: &i32) {
    print!("{number},")
}
If the print_number function would not borrow, but take the number, the code will not compile. It would be similar to you borrowing an item from me, but then giving it away to someone else. The Rust compiler is fair and will not let you do that!
