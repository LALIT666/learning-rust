const CURRENT_YEAR: i32 = 2026;
const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;
const DAYS_PER_YEAR: i32 = 365;

fn category(age: i32) -> &'static str {
    match age {
        0..=12 => "Child",
        13..=19 => "Teen",
        20..=150 => "Adult",
        _ => "Age Not defined"
    }
}

fn print_person(name: String, birth_year: i32) {
    // LOGICAL BUG: age negative aa raha hai (is line ko sahi karna hoga)
    let age = CURRENT_YEAR - birth_year;

    println!("{name}: age={age} ({})", category(age));
}

fn main() {
    let people: Vec<(String, i32)> = vec![
        ("Aman".to_string(), 2002),
        ("Sara".to_string(), 2015),
    ];

    let mut teen_count = 0;

    for (name, year) in people {
      let name_length = name.len();
        print_person(name, year);
        
       
        // ownership error aayega yahan (name use ho raha hai print_person ke baad)
        if name_length > 3 {
            // kuch nahi
        }

        let age = CURRENT_YEAR - year;
        if category(age) == "Teen" {
            teen_count += 1;
        }
    }

    let minutes = MINUTES_PER_HOUR * HOURS_PER_DAY * DAYS_PER_YEAR;
    println!("Minutes in a year: {minutes}");
    println!("Total teens: {teen_count}");
}