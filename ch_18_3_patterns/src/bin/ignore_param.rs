// Ignore first param. Useful for example if implementing trait and first param
// is not used.
fn foo(_: i32, y: i32) {
	println!("This code only uses the y parameter: {}", y);
}

fn main() {
	foo(3, 4);
}
