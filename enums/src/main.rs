enum IpAddr{
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip: IpAddr) {}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);

    println!("Hello, world!");
}
