fn main() {

    pass_ownership_to_other_fn();

   handle_return_ownership();

   cal_string_length();
    
}

// Ex : 1


// Handovering the ownership of variable scop to other fn which will take care of scope of the variable
fn pass_ownership_to_other_fn() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("Inside main - displaying s {}", s);

    let x = 5;

    makes_copy(x);

    println!("Inside main - displaying x {}", x);
}

fn takes_ownership(some_string: String) {
    println!("from takes_ownership {}", some_string);
}

fn makes_copy(some_interger: i32) {
    println!("from makes_copy {}", some_interger);
}

// Ex : 2

fn handle_return_ownership() {
    let s1 = gives_ownership();

    let  some_string = String::from("creating a hello to pass");

    let s2 = takes_and_gives_back_ownership(some_string);

    // println!("Validating ownership : s1 {}, s2 {}, some_string {}", s1, s2, some_string);
    println!("Validating ownership : s1 {}, s2 {}", s1, s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("giving ownership");
    some_string
}

fn takes_and_gives_back_ownership(some_string: String) -> String {
    some_string
}

// Ex : 3

// Returning values as tuple


fn cal_string_length() {
    let s1 = String::from("hello");

    let (_str, len) = calc_str_len(s1);

    println!("string length of hello : {}", len);
}

// (JS : A pure fn), takes ownership and gives back some value
fn calc_str_len(some_string: String)-> (String, usize) {
    let len = some_string.len();
    (some_string, len)
}




