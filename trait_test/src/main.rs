use trait_test::Summary;
use trait_test::Tweet;

fn main() {
   let tweet = Tweet {
        username: String::from("hores_ebooks"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    }; 

    println!("1 new tweet: {}", tweet.summarize());
}
