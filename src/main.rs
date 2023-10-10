use std::io::{self, Write};

struct Triangle {
  x: u32,
  y: u32,
  z: u32,
}


#[derive(Debug)]
enum TriangleType {
  Isosceles,
  Equilateral,
  Scalene,
}


impl Triangle {
  fn triangle_type(&self) -> TriangleType {
    return match (self.x, self.y, self.z) {
      (x, y, z) if x == y && y == z => TriangleType::Equilateral,
      (x, y, z) if x == y || x == z || y == z => TriangleType::Isosceles,
      _ => TriangleType::Scalene
    };
  }
}

fn read_u32_input(prompt: &str) -> u32 {
  loop {
    print!("{}", prompt);
    io::stdout().flush().expect("Error flushing");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    match input.trim().parse() {
      Ok(num) => return num,
      _ => continue,
    };
  }
}


fn main() {
  let x = read_u32_input("Enter x: ");
  let y = read_u32_input("Enter y: ");
  let z = read_u32_input("Enter z: ");
  let triangle = Triangle { x, y, z};

  println!("Triangle is {:?}", triangle.triangle_type());
}
