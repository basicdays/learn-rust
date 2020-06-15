fn main() {
	let x = 5;

	match x {
		// "range" syntax (only valid with numeric and char)
		1..=5 => println!("one through five"),
		_ => println!("something else"),
	}
}
