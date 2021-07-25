#[derive(Debug)]
enum IpKind {
    IP4,
    IP6
}

#[derive(Debug)]
struct IPAddress {
    kind : IpKind,
    address : String
}


fn main() {
    let home = IPAddress {
        kind: IpKind::IP4,
        address: String::from("10.0.0.1")
    };

    let office = IPAddress {
        kind: IpKind::IP6,
        address: String::from("10.0.0.1")
    };

    println!("Enum ip address {:#?} {:#?}", home, office);
}
