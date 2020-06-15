fn main() {
	let x = 1;

	match x {
		// "or" pattern syntax
		1 | 2 => println!("one or two"),
		3 => println!("three"),
		_ => println!("anything"),
	}
}
