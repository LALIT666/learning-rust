fn main() {
   
let name = "John";
let age = 30;
println!("{} is {} years old.", name, age);

let x =  5;
println!("{}", x);


let mut y = 8;
println!("Before: {}", y);
y = 10;
println!("After: {}", y);

let my_num = 5;
let my_double = 5.99;
let my_letter = "D";
let my_bool = true;
let my_text = "Hello World";

println!("my_num: {}, my_double: {}, my_letter: {}, my_bool: {},  my_text: {}", my_num, my_double, my_letter, my_bool, my_text);


let age: i32 = 25;
println!("Age is: {}", age);


let price: f64 = 19.99;
println!("Price is: ${}", price);


let my_grade: char = 'B';
println!("myGrade : {}", my_grade);


let name: &str = "John";
println!("Hello, {}!", name);

let is_logged_in: bool = true;
println!("User logged in? {}", is_logged_in);


let name_1 = "Nitin";
let age_nitin = 28;
let is_admin = false;

println!("Name: {}", name_1);
println!("Age: {}", age_nitin);
println!("Admin: {}", is_admin);


}


