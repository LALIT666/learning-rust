//Buggy code 
fn main() {
    let mut s = String::from("hello");

    add_world(&mut s);
    let len = calculate_length(&s);

    println!("String: {s}");
    println!("Length: {len}");
}

fn add_world(text: &mut String) {
    text.push_str(", world");
}

fn calculate_length(text: &String) -> usize {
    text.len()
}
