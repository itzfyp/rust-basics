fn main() {
    

    let val_A = Some(10);
    let val_B : Option<i32> = None;

    let val = match val_A {
        Some(i) => Some(i+10),
        None => None,
    };

     println!("what is the matached value : {:?}",  val );

}

