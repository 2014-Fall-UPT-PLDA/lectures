fn main() {
  let mut x = 42i;
  
  {
    let y = &mut x;
    println!("the borrow used to be: {}", y);
    *y = 11i;
    println!("the borrow now is: {}", y);
  }
  never_giving_it_back(&x);
  let gollum = &x;
}

fn never_giving_it_back(the_precious: &int) {
  let printer = || { println!("mine is: {}", *the_precious); };
  printer();
}