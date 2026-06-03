fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(s1); // s1 ownership chali gayi!

    println!("Length of hello is {len}"); // ❌ ERROR! s1 ab hamara nahi raha
}

fn calculate_length(s: String) -> usize {
    s.len()
}