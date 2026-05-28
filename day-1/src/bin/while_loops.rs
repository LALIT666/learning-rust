fn main() {
  let mut count = 1;

  while count <= 5 {
     println!("Count: {count}");
     count += 1;
  }


  let mut num = 1;

// while num <= 10 {
//   if num == 6 {
//     println!("Now the number is 6");
//     break;
//   }
//   println!("Number: {}", num);
//   num += 1;
// }



while num <= 10 {
  if num == 6 {
    num += 1;
    continue;
  }

  println!("Number: {}", num);
  num += 1;
}
 }