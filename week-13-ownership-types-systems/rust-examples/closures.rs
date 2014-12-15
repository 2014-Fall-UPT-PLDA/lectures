fn main() {
  let mut x = 5i;

  let printer = || { println!("x is: {}", x); };
  twice(printer);  
  x = 6i; // error: cannot assign to `x` because it is borrowed
}

fn twice(f: || -> ()) {
    f();
    f();
}