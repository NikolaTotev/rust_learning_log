pub fn create_and_print_vector() {
    println!("Creating a vector:");
    let num_vec = vec![1, 2, 3, 4];
    let str_vec = vec!["A", "B", "C"];

    println!("Looping through my vectors:");

    println!("Number vector");
    for num in num_vec {
        print!("{num},");
    }
    println!();

    println!("str vector");
    for str in str_vec {
        print!("{str}, ")
    }
    println!();
}
