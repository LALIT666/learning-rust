fn main() {
    const BIRTHYEAR: i32 = 1980;
    const MINUTES_PER_HOUR: i32 = 60;

    println!("BIRTHYEAR = {BIRTHYEAR}");
    println!("MINUTES_PER_HOUR = {MINUTES_PER_HOUR}");
    println!("MINUTES_PER_HOUR = {}", MINUTES_PER_HOUR); 

     
     let without_variable = "my name is nitin {}";
     println!("without_variable: {without_variable}");
     let s = format!("BIRTHYEAR = {BIRTHYEAR}, MINUTES_PER_HOUR = {MINUTES_PER_HOUR}");
     println!("{s}");

}