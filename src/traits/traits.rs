use std::ops::Add; // addition operator, https://doc.rust-lang.org/std/ops/trait.Add.html

#[allow(dead_code)]
struct Overloaded{
  a: i32, // signed 32 bit integer
  b: i32, // signed 32 bit integer
}

impl Add<Overloaded, Overloaded> for Overloaded{
  fn add(&self, _rhs: &Overloaded) -> Overloaded {
    let mut res: Overloaded = Overloaded { a: 9, b: 0 };
    res.a = self.a + _rhs.a;
    res.b = self.b + _rhs.b;
    res
  }
}

fn main(){
  let a: Overloaded = Overloaded { a: 10, b: 20};
  let b: Overloaded = Overloaded { a: 1, b: 2};
  let sumobj: Overloaded;
  sumobj = a + b;
}
