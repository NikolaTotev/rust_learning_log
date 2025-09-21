const THE_MEANING_OF_LIFE: u32 = 21 * 2;

fn main() {
    // Variables and mutability

    // Variables
    let x = 5;
    println!("This is a number: x = {x}");

    let mut y = 6;
    println!("This is a mutable number: y = {y}");
    y = 10;

    println!("See: y = {y}");

    // Constants
    println!("The meaning of life: {THE_MEANING_OF_LIFE}");

    //Shadowing
    let a = 21;

    let a = a * 2;

    {
        let a = a * 3;
        println!("a value in inner scope {a} ");
    }

    println!("a in outer scope {a} ");

    let spaces = "     ";
    let spaces = spaces.len();

    println!("Space count {spaces}");

    // Data types
    let my_float: f64 = 2.42;

    println!("Floating in the wind: {my_float}");

    let has_pizza: bool = false;

    if has_pizza {
        println!("Pizza for dinner yay!");
    } else {
        println!("Darn, no pizza");
    }

    let my_char = 'n';

    println!("My name starts with {my_char}");

    let tup: (i32, f64, bool) = (500, 4.2, true);

    println!("My tuple has: {0}, {1}, {2}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;

    println!("I just destructured my tuple! {x}, {y}, {z}");

    let my_cool_array = [1, 2, 3, 4, 5];

    let my_prefilled_array = [3; 5];

    println!("My first array!");
    for num in my_cool_array {
        print!("{num} ");
    }
    println!();

    println!("My second array!");
    for num in my_prefilled_array {
        print!("{num} ");
    }
    println!();

    my_first_function(42, 2.42);

    println!(
        "This makes the fuction return even: {0}",
        number_function(true)
    );
    println!(
        "This makes the fuction return odd: {0}",
        number_function(false)
    );

    
}

fn my_first_function(param1: u32, param2: f64) {
    println!("This function taked param1: {param1}, param2: {param2}");
}

fn number_function(should_return_even: bool) -> u32 {
    if should_return_even { 2 } else { 5 }
}

// This is a comment!
