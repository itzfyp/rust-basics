mod front_of_house;

 // absoulte import

 // re -exporting too (pub use)
pub use crate::front_of_house::hosting;

// alias
use crate::front_of_house::hosting::add_to_waitlist as abs_add_to_waitlist;
 
 // relative import
 use self::front_of_house::hosting::add_to_waitlist;
 

pub fn eat_at_restaruant() {
  
   hosting::add_to_waitlist();
   abs_add_to_waitlist();
   add_to_waitlist();
    
}
