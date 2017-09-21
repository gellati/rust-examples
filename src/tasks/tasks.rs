use std::thread::spawn;

static NTASKS: i32 = 10;

// spawn no work
//      spawn(proc() {
//    });

fn main(){
  for i in 0..NTASKS {
    do spawn {
      println!("this is task nr {}", i)
    }
  }
}
