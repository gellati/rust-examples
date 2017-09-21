//use std::f32;

fn swap<T: Clone>(x: &mut T, y: &mut T){
  let t = x.clone();
  *x = y.clone();
  *y = t.clone();
}

fn main(){
  let mut a = 2.0_f32; // 32 bit float
  let mut b = 1.0_f32; // 32 bit float
  swap(& mut a, & mut b);
  println!("a: {}\t b: {}", a, b);
}
