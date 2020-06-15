fn main() {
	let x = Some(5);
	let y = 10;

	match x {
		Some(50) => println!("Got 50"),
		// pattern introduces new y var, shadowing y from scope above
		Some(y) => println!("Matched, y = {:?}", y),
		_ => println!("Default case, x = {:?}", x),
	}

	println!("at the end: x = {:?}, y = {:?}", x, y);
}
