fn main() {

    let dimensions = (50, 30);

    println!("The area of the rectangle is {} square pixels", area(dimensions));


}

fn area(dimensions : (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}