use std::default;

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

enum LoginStatus {
    Success(String),
    Error(String),
}

fn main() {
  let my_direction = Direction::Down;
  println!("We are going up!");


  match my_direction {
    Direction::Up => println!("Going Up"),
    Direction::Down => println!("Going Down"),
    Direction::Left => println!("Going Left"),
    Direction::Right => println!("Going Right"),
  
      
  }


  //Enum with data
  let result1 = LoginStatus::Success(String::from("Welcome JOHN!"));
  let result2 = LoginStatus::Error(String::from("Incorrect password"));


  match result2 {
    LoginStatus::Success(message) => println!("Success: {} ", message),
    LoginStatus::Error(message) => println!("Error: {} ", message),

  }
}