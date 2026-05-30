fn main() {
  let numbers = [1,2,3,4,5,6];
  println!( "the first number is: {} ", numbers[0] );



  //this is how you change the value in array 
  let mut numbers_array = [1,2,3,4,5,6]; //make it mutable 
  numbers_array[0] = 10;
  println!( "the first number is: {} ", numbers_array[0] );

  //Array length -- .len()
  println!("the length of the numbers_array is : {} ", numbers_array.len());

  //looping through an Array
  let fruits = ["apple", "banana", "orange"];

  for fruit in fruits {
    println!("I like: {fruit}")
}


  //printing the whole array
  println!("Printing the whole array of fruits: {:?}", fruits);

  //only one element 
  println!("Printing one(first) element of the array: {}", fruits[0]); // no need of {:?}

}