

fn main() {
  //creating a vector
  let fruits = vec!["apple", "mango", "banana"];

  //accessing vec element 
  println!("first element of vector named fruit: {}", fruits[0]);


  //changing the vector values
  let mut numbers = vec![1,2,3,4,5,6,7,8];
  
  println!("before changing the 1st index in the vector: {:?}", numbers);
  numbers[1] = 1000;
  println!("After changing the 1st index in the vector: {:?}", numbers);

  //adding element to the vector
 numbers.push(10);
 println!("numbers vector after pushing  10: {:?}", numbers);

 //removing from the array
 numbers.pop();
 numbers.pop();
 println!("number vec after poping twice: {:?}", numbers);
  
//adding element at specific index -- insert
numbers.insert(numbers.len() - 1, 9999);
println!("inset at numbers.len() - 1 element 9999: {:?}", numbers);


//removing use remove()

numbers.remove(numbers.len() - 1);
println!("removed 0th index element : {:?}", numbers);

//looping
for number in &numbers {
  
 
 
  println!("this is for loop:  {}", number);
}
}