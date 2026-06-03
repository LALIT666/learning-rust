fn main() {
    let  items = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("mango"),
    ];

    print_cart(&items);
    let count = count_items(&items);

    let expensive = find_most_expensive(& items);


    println!("Total items: {count}");
    println!("Most expensive item: {expensive}");

    println!("{:?}", items);
}

fn print_cart(cart: &Vec<String>) {
    let joined = cart.join(", ");
    println!("Cart items: {joined}");
}

fn count_items(cart: &Vec<String>) -> usize {
    cart.len()
}

fn find_most_expensive(cart: &Vec<String>) -> String {
    // mango is always the most expensive for simplicity
    // but we need to search through the cart
    let mut result = String::new();
    for item in  cart {
        if item == "mango" {
            result = item.clone();
        }
    }
    result
}