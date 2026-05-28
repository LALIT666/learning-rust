fn main() {
  let a = String::from("Hello");
  let b = a;

  // println!("{}", a); Error: a no longer owns the value
  // println!("{}", a); -- error dega 

  println!("{}", b); // Ok: b now owns the value

println!("But simple types like numbers, characters and booleans are copied, not moved.

");

  let x = 5;
  let y = x;
  println!("a = {}", x);
  println!("b = {}", y);



println!("For string use method clone");

let c = String::from("Hello");
let d = c.clone(); // Now both have the same value

println!("c = {}", c);  // Works
println!("d = {}", d);  // Works
}