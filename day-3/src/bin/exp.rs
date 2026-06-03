fn main(){
   let search = String::from("Rust Book");
    println!("\nSearching for: {search}");
    printing(&search);
    

} 

fn printing(book: &String) {
  println!("book {book}")
}