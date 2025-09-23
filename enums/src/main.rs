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

fn main() {
    let v4_addr = IpAddr::V4(String::from("192.168.0.1"));
    let v6_addr = IpAddr::V4(String::from("4E:4E:4E:4E"));

    let addr = v4_addr.get_addr();
    println!("v4 {0}", addr);
}
