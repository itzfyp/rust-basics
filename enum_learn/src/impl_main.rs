#[derive(Debug)]
enum Message {
    ID(u32),
    Sender { no : u64, name: String},
    Write(String),
}

impl Message {
    fn call(&self) {
        println!("Enum ip address {:#?}", self);
    }
}


fn main() {
    
    let id = Message::ID(989498);
    let msg = Message::Write(String::from("Hello jayapal"));
     
    id.call();
    msg.call();

    // println!("Enum ip address {:#?}", );
}
