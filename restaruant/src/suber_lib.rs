mod front_of_house {
    fn serve_order() {
        println!("server_order")
    }
  
     pub mod hosting {
          pub fn add_to_waitlist() {
              super::serve_order();
          }
      }
  
  
  }
  
  pub fn eat_at_restaruant() {
      crate::front_of_house::hosting::add_to_waitlist();
  
      front_of_house::hosting::add_to_waitlist();
  }
  