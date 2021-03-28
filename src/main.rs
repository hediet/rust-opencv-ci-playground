use opencv::prelude::*;

fn main() {
	let m = Mat::default().unwrap();
	println!("a {}", m.typ().unwrap());
}