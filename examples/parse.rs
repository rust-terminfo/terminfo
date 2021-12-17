extern crate terminfo;
use std::env;

fn main() {
	println!("{:?}", terminfo::Database::from_path(env::args().nth(1).expect("no file given")));
}
