fn main() {
  let greeting: &str = "Hello";
println!("{}", greeting);



//Creating a string

let text1 = "Hello World".to_string();


let text2 = String::from("Hello World text 2");


println!("text1: {text1}, text2: {text2}");




// Change a String

let mut greeting = String::from("Hello");
greeting.push_str(" World");
println!("{greeting}" ); // Hello World



//Use push() to add one character:
let mut word = String::from("Hi");
word.push('!');
println!("{}", word); // Hi!


// Concatenate Strings - You can combine strings using the format! macro


let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = format!("{} {} {}", s1, s2, s3);
println!("{}", result);



let s1 = String::from("Hello");
let s2 = String::from("World!");
let s3 = String::from("What a beautiful day!");
let result = s1 + " " + &s2 + " " + &s3;
println!("{}", result);




//string legth
let name = String::from("John");
println!("Length: {}", name.len()); // 4
}