use std::{fs::File, io::ErrorKind};

fn main() {
    let panik_type = String::from("file_ops");

    match &panik_type[..] {
        "out_of_bounds" => {
            println!("Doing {panik_type} panic");
            let vector = vec![1, 2, 3];
            vector[4];
        }
        "manual_panic" => {
            panic!("I saw a spider!")
        }
        "file_ops" => {
            let greeting_file_result = File::open("hello.txt");

            let greeting_file = match greeting_file_result {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => match File::create("hello.txt") {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {e:?}"),
                    },
                    _ => {
                        panic!("Problem opening the file: {error:?}");
                    }
                },
            };
        }
        _ => {
            println!("Ops")
        }
    }
}
