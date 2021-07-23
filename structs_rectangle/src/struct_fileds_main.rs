
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height: u32
}


fn main() {

    let dimensions = Rectangle {
        width: 50,
        height: 30
    };

    println!("The area of the rectangle is {} square pixels", area(&dimensions));

    println!("Struct {:#?}", dimensions);


}

fn area(dimensions : &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}