
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other : &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

impl Rectangle {
    fn init() -> Rectangle {
        Rectangle {
            width: 0,
            height: 0,
        }
    }
}



fn main() {

    let dimensions_1 = Rectangle {
        width: 50,
        height: 30
    };
    
    println!("The area of the rectangle is {} square pixels", dimensions_1.area());

    let dimensions_2 = Rectangle {
        width: 40,
        height: 20
    };

    let dimensions_3 = Rectangle {
        width: 60,
        height: 20
    };

    println!("Dimention 1 can  hold  Dimention 2 {}", dimensions_1.can_hold(&dimensions_2));
    println!("Dimention 1 can  hold  Dimention 3 {}", dimensions_1.can_hold(&dimensions_3));

    let sq = Rectangle::square(30);
    let default_dimentions = Rectangle::init();

    println!("Associated fn {:#?}", sq);
    println!("Associated fn {:#?}", default_dimentions);


}

