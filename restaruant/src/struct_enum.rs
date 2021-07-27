mod back_of_house {

    // Exposing only pub fields
  pub struct Breakfast {
      pub toast : String,
      seasonal_fruit : String
  }

  impl Breakfast {
      pub fn summer(toast : &str) -> Breakfast {
          Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("apple")
          }
      }
  }

  // Exposing all varients
  #[derive(Debug)]
  pub enum Driks {
      COFFEE,
      TEA,
      JUCIE
  }
}

pub fn eat_at_restaruant() {
   let mut meal =  back_of_house::Breakfast::summer("summa");


   println!("what is my toast {}", meal.toast);
   
   meal.toast = String::from("rice");
   
   println!("what is my toast now {}", meal.toast);
   
   let drink = back_of_house::Driks::TEA;

   println!("what is my drink {:?}", drink);
   


    
}
