use std::env;

fn main() {

  let mut args = env::args();
  args.next().unwrap();
  let params: Vec<String> = args.collect();
  println!("=> {:?}", params);

}
