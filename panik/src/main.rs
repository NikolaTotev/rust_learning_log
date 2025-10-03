fn main() {
    let panik_type = String::from("manual_panic");

    match &panik_type[..] {
        "out_of_bounds" => {
            println!("Doing {panik_type} panic");
            let vector = vec![1, 2, 3];
            vector[4];
        }
        "manual_panic" => {
            panic!("I saw a spider!")
        }
        _ => {
            println!("Ops")
        }
    }    
}
