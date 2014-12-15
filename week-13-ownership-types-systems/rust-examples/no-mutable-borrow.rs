fn main () {
  let mut borrow_me = 1i;
  println!("at first borrow_me is: {}", borrow_me);
  {
  	let lent = &mut borrow_me;
  	let lent_again = &mut borrow_me;
    borrow_me = 2i;
  	println!("lent is: {}", lent);
  }
  borrow_me = 2i;
  println!("after lent, borrow_me is: {}", borrow_me);
}
