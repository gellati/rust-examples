 // cfg - conditional compilation, evaluates configuration flags and compile code on the basis of it

#[cfg(target_os = "macos")]
fn where_am_i(){
  println!("Hello, I'm running in OSX")
}

#[cfg(target_os = "linux")]
fn where_am_i(){
  println!("Hello, I'm running in Linux")
}

fn main(){
  where_am_i();
}
