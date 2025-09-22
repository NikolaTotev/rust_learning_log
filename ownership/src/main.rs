fn main() {
    let s1 = String::from("Hello there");
    let s2 = String::from("Hello there again");
    takes_ownership(s1);
    let mut s3 = returns_ownership(s2);
    println!("This ownership was returned {s3} ");

    borrower(&s3);
    println!("{s3} was borrowed");

    mutable_borrow(&mut s3);
    println!("After mutable borrow s3 is {s3}");

    println!("The first word of s3 is {0}", first_word(&s3));

    println!("This is a slice: {0}", &s3[5..s3.len()]);
}

fn takes_ownership(string: String) {
    println!("Reading and not returning {string}");
}
fn returns_ownership(string: String) -> String {
    println!("Reading and returning {string}");

    string
}

fn borrower(borrowed_string: &str) {
    println!("I borrowed {borrowed_string}");
}

fn mutable_borrow(mutable_ref: &mut String) {
    println!("I borrowed {mutable_ref} and will add foo");
    mutable_ref.push_str(" foo");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
