//first we must import hashmap from the library
use std::collections::HashMap;


fn main() {

  //Creating a hashmap
  let mut capital_city = HashMap::new();

  capital_city.insert("England", "London");
  capital_city.insert("India", "New Delhi");
  capital_city.insert("Germany", "Berlin");
  capital_city.insert("Norway", "Oslo");

  println!("this is without  --> {:?}", capital_city);
  println!("this is with # --> {:#?}", capital_city);

  //Access Values -- .get()

  if let Some(city) = capital_city.get("England") {
    println!("the capital_city of england is: {}", city);
  }else {
    println!("England is not in the map");
  }


  //Update Values
  capital_city.insert("India", "Delhi");
  println!("this is after making India's capital Delhi instead of new delhi : {:?}", capital_city);
 

 //Removing values
 capital_city.remove("England");
 println!("{:?}", capital_city);


 //loop
 for (country, city) in &capital_city{
 println!("The capital of {} is {}.", country, city);
 }

}