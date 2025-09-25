enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn get_addr(&self) -> &String {
        match self {
            IpAddr::V4(addr) => addr,
            IpAddr::V6(addr) => addr,
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // -- snip --
        }
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}


fn main() {
    let v4_addr = IpAddr::V4(String::from("192.168.0.1"));
    let v6_addr = IpAddr::V4(String::from("4E:4E:4E:4E"));

    let addr = v4_addr.get_addr();
    println!("v4 {0}", addr);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(six);
    dbg!(six.unwrap());

    let roll = 10;

    match roll {
        3 => println!("Got a three!"),
        10 => println!("Ten it is!"),
        _ => println!("Darn it!"),
    }

    let schrodingers_cat = Some("kitty");

    if let Some("kitty") = schrodingers_cat {
        println!("Kitty is alive!")
    }
    else{
        println!("Kitty is in an unknown state");
    }

     if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska)) {
        println!("{desc}");
    }
}
