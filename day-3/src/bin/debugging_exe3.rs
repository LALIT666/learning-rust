fn main() {
    let books = vec![
        String::from("Rust Book"),
        String::from("Python Book"),
        String::from("Java Book"),
        String::from("C++ Book"),
    ];

    println!("=== Library System ===");

    display_books(&books);
    let total = count_books(&books);
    println!("Total books: {total}");

    let search = String::from("Rust Book");
    println!("\nSearching for: {search}");
    let found = search_book(&books, &search);
    println!("{found}");

    let mut borrowed = vec![
        String::from("Rust Book"),
        String::from("Python Book"),
    ];

    show_borrowed(&mut borrowed);
    show_available(&books, &borrowed);

    let return_book = String::from("Rust Book");
    println!("\nAfter returning: {return_book}");
    remove_from_borrowed(&mut borrowed, return_book);

    display_books(&books);
    let final_total = count_books(&books);
    println!("Total books still: {final_total}");
}

fn display_books(all_books: &Vec<String>) {
    println!("All books: {all_books:?}");
}

fn count_books(all_books: &Vec<String>) -> usize {
    all_books.len()
}

fn search_book(all_books: &Vec<String>, name: &String) -> String {
    for book in all_books {
        if book == name {
            return format!("Found: {book} ✅");
        }
    }
    format!("Not found: {name} ❌")
}

fn show_borrowed(borrowed_list: &mut Vec<String>) {
    let joined = borrowed_list.join(", ");
    println!("\nBorrowed books: {joined}");
}

fn show_available(
    all_books: &Vec<String>,
    borrowed_list: &Vec<String>,
) {
    let mut available = Vec::new();
    for book in all_books {
        if !borrowed_list.contains(book) {
            available.push(book.clone());
        }
    }
    let joined = available.join(", ");
    println!("Available books: {joined}");
}

fn remove_from_borrowed(
    borrowed_list: &mut Vec<String>,
    book_name: String,
) {
    borrowed_list.retain(|b| b != &book_name);
}