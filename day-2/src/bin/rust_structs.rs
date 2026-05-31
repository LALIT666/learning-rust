

fn main() {

  struct  Person {
  name: String,
  age: u32,
  can_vote: bool,

}


let user1 = Person {
  name: String::from("John"),
  age: 35,
  can_vote: true,
};

let mut user2 = Person {
  name: String::from("Nitin"),
  age: 24,
  can_vote: true,
};

  // Access and print the values
println!("Name: {}", user1.name);
println!("Age: {}", user1.age);
println!("Can vote? {}", user1.can_vote);


//change the value
println!("age of user2 before change: {}", user2.age);
user2.age = 50;
println!("age of user2 after change: {}", user2.age);
}