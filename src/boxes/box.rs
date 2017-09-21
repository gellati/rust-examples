use std::mem;  // functions for memory querying and manipulation

#[allow(dead_code)] // suppress compilation warnings
struct Point{
  x: i32, // 32 bit signed integer
  y: i32, // 32 bit signed integer
}

fn main(){
  let stack = Point{x: 0, y: 0};
  // boxed values are allocated in the heap by creating Box<T>
  // box is a smart pointer to heap allocated value of type T
  // https://rustbyexample.com/std/box.html

  let heap: Box<Point> = Box::new(Point {x: 1, y: 1});

  println!("size stack: {}\theap: {}",
           mem::size_of_val(&stack),
           mem::size_of_val(&heap));
}
