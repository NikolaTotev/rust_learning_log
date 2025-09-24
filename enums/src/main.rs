enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn get_addr(&self) -> &String{
        match self {
            IpAddr::V4(addr) => addr,
            IpAddr::V6(addr) => addr,
        }

    }
}


fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None, 
        Some(i)=>Some(i+1),
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
        _ => println!("Darn it!")    
    }

    
}
