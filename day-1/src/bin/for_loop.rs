fn main() {
  
  // 1 to 5 
  for i in 1..6 {
  println!("i is: {}", i);
}


//1 to 6
for t in 1..=6 {
  println!("t is: {}", t);
}


//break and continue

for i in 1..=10 {
  if i == 3 {
    continue; // skip 3
  }
  if i == 5 {
    break; // stop before printing 5
  }
  println!("i is: {}", i);
}
}