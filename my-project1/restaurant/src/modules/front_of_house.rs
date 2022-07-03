// * 文件本身就成了一个mod
mod front_of_house {}

pub mod hosting {
  pub fn add_to_waitlist() {
    println!("add to waitlist 执行了!")
  }

  fn seat_at_table() {}
}

mod serving {
  fn take_order() {}
  fn server_order() {}
  fn take_payment() {}
}
