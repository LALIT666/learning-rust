fn main() {
  // creating a tuples
  let person = ("luckas", 30, true);

  //Accesing the tuples
  println!("Name: {}", person.0);
  println!("Age: {}", person.1);
  println!("Is Active: {}", person.2);

  //Unpack the tuple
  let person2 = ("Jenny", 45, false);
  let (name, age, active) = person2;

println!("Name: {}", name);
println!("Age: {}", age);
println!("Active: {}", active);


let user = get_user();
println!("Name of the user is : {} and user's age is : {}", user.0, user.1);

}


//Returning tuple form the function
fn get_user() -> (String, i32) {
  (String::from("Liam"), 25)
}