//use std::task;
use std::thread::spawn;

// not working
// sources consulted:
// http://www.rustforrubyists.com/book/chapter-06.html
// http://aml3.github.io/RustTutorial/html/04.html
// http://bettong.net/2014/08/01/learning-rust-tasks-and-messages-part-2/

static NTASKS: i32 = 10;

// spawn no work
//do spawn {
//}
//  for i in range(0u, NTASKS) {


//fn main(){
//  for i in 0..NTASKS {
//    spawn(proc() {
//      println!("this is task nr {}", i);
//    });
//  }
//}

fn main() {
  for i in 0..NTASKS{
    spawn(|| {
      println!("this");
    });
  }
}
