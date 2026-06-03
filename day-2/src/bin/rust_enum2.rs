enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Shape {
  Cicle(f64),
  Rectangle(f64, f64),
  Square(f64),
}

impl Shape {
  fn area(&self) -> f64 {
    match self {
      Shape::Cicle(r) => 3.14159 * r * r,
      Shape::Rectangle(w,h ) => w * h,
      Shape::Square(s) => s *s,
    }
  }
  fn name(&self) -> &str {
  match self {
    Shape::Cicle(_) => "Circle",
    Shape::Rectangle(_,_ ) => "Rectangle",
    Shape::Square(_) => "Square",
  }
} 
}




fn action (light: &TrafficLight) {
  match light {
    TrafficLight::Red    => println!("🔴 STOP!"),
        TrafficLight::Yellow => println!("🟡 SLOW DOWN!"),
        TrafficLight::Green  => println!("🟢 GO GO GO!"),
  }
}


fn main() {
  let lights = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green,];

  for light in &lights {
    action(light);
  }


  let shapes = [
    Shape::Cicle(5.0),
    Shape::Rectangle(4.0, 6.0),
    Shape::Square(3.0),
  ];

  for shape in &shapes {
    println!("{} area = {:.2}", shape.name(), shape.area());
  }
}