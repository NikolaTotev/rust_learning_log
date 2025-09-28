pub fn print_string() {
    let data = "This is my cool data!";
    let mut string_data = data.to_string();

    println!("My string is {string_data}");

    string_data.push_str("additional things");

    println!("This is string data after adding stuff to it: {string_data}");

    println!("The chars in my string are:");
    for ch in string_data.chars() {
        print!("{ch} ");
    }
    println!();
}
