fn main() {
  let mut count = 1;

  loop {
    println!("{} ", count);
    count += 1;

    if count > 10 {
    break;
  }
  }

  


    // 2. while
    println!("--- while ---");
    let mut countdown = 5;
    while countdown > 0 {
        println!("{}...", countdown);
        countdown -= 1;
    }
    println!("GO!");

     // 2. for
    println!("--- for ---");
    let numbers = [5,4,3,2,1];
    println!("{:?}", numbers);
    for num in numbers {
      println!("{}", num);
    }

  
}