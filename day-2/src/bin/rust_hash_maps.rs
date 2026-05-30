//first we must import hashmap from the library
use std::collections::HashMap;


fn main() {

  //Creating a hashmap
  let mut capital_city = HashMap::new();

  capital_city.insert("England", "London");
  capital_city.insert("India", "New Delhi");

  println!("{:?}", capital_city)
 

}