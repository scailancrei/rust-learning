use genericos_y_traits_y_lifetimes::aggregator::{SocialPost, Summary};
fn main() {
    let post = SocialPost {
        username: String::from("aaaa"),
        content: String::from("content"),
        reply: true,
        repost: true,
    };

    println!("un post de {}", post.summarize());
}
