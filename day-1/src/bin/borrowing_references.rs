fn main() {

  //A reference lets you look at a value without owning it. You create a reference using the & symbol:
  let a = String::from("Hello");
  let b = &a;

  //Since b is only borrowing the value, a still owns it.
  println!("a = {}", a);
  println!("b = {}", b);



  //
  let mut name = String::from("John");

  //If you want to change a value through a reference, you need to make the reference mut: refrence ko mutable bana diya hai &mut se kyuki humko baad me name_ref me push_str karna hai ref name right mutablity name_ref ki and push bhi hum name_ref me kar rahe hai okay 
let name_ref = &mut name;
name_ref.push_str(" Doe");

println!("{}", name_ref); // John Doe
}