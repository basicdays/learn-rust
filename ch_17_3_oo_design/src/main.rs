use ch_17_3_oo_design as oo_design;
use oo_design::blog1::Post;

fn main() {
	let mut post = Post::new();

	post.add_text("I ate a salad for lunch today");
	assert_eq!("", post.content());

	post.request_review();
	assert_eq!("", post.content());

	post.approve();
	assert_eq!("I ate a salad for lunch today", post.content());
}
