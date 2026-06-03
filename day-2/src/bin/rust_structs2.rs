struct Car {
  brand: String,
  model: String,
  year: u32,
  is_electric: bool,
}

impl Car {
  fn describe(&self) {
    let car_type = if self.is_electric {"Electric"} else {"Petrol/Diesel"};
    println!("{} {} ({}) - {}", self.brand, self.model, self.year, car_type);
  }
}

fn main() {

  let car1 = Car {
    brand: String::from("Tesla"),
    model: String::from("Model 3"),
    year: 2023,
    is_electric: true,

  };

  let car2 = Car {
        brand: String::from("Tata"),
        model: String::from("Nexon"),
        year: 2024,
        is_electric: false,
    };

    car1.describe();
    car2.describe();


}