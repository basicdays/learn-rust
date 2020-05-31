use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
	pub fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	// associated type
	type Target = T;

	fn deref(&self) -> &T {
		&self.0
	}
}

pub fn hello(name: &str) {
	println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn deref_example() {
		let x = 5;
		let y = &x;

		assert_eq!(5, x);
		assert_eq!(&5, y);
		assert_eq!(5, *y);
	}

	#[test]
	fn deref_box_example() {
		let x = 5;
		let y = Box::new(x);

		assert_eq!(5, x);
		assert_eq!(5, *y);
	}

	#[test]
	fn deref_mybox_example() {
		let x = 5;
		let y = MyBox::new(x);

		assert_eq!(5, x);
		assert_eq!(5, *y);
		// syntax sugar for (compiler automatically calls #deref() first)
		assert_eq!(5, *(y.deref()));
	}

	#[test]
	fn deref_coercion_example() {
		let m = MyBox::new(String::from("Rust"));
		hello(&m);
	}

	#[test]
	fn explicit_deref_example() {
		let m = MyBox::new(String::from("Rust"));
		// (*m) = MyBox<String> => String
		// & and [..] = get string slice of String that is the whole length
		hello(&(*m)[..]);
	}
}
