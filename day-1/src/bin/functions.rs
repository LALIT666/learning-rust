fn say_hello() {
  println!("Hello form the say_hello function");
}

fn greet(name: &str, age: i32) {
  println!("Hello, {name}, age: {age}")

}

fn add (a:i32, b:i32) -> i32 {
  return a + b;
}

fn main() {
  

say_hello();
greet("John", 32);

let sum = add(3, 4);
println!("Sum is: {}", sum);


}