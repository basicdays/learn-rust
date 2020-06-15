use ch_17_3_oo_design as oo_design;
use oo_design::blog2::Post;

fn main() {
	let mut post = Post::new();

	post.add_text("I ate a salad for lunch today");

	let post = post.request_review();

	let post = post.approve();

	assert_eq!("I ate a salad for lunch today", post.content());
}
