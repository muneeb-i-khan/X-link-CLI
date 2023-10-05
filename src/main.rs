use std::io;

use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

fn generate_twitter_link(tweet: &str) -> String {
    let encoded_tweet = percent_encode(tweet.as_bytes(), NON_ALPHANUMERIC);
    format!("https://twitter.com/intent/tweet?text={}", encoded_tweet)
}

fn main() {
    println!("Enter your tweet text:");
    
    let mut tweet = String::new();
    io::stdin().read_line(&mut tweet).expect("Failed to read line");
    
    let tweet = tweet.trim();

    if tweet.is_empty() {
        eprintln!("Tweet text cannot be empty.");
        std::process::exit(1);
    }

    let twitter_link = generate_twitter_link(tweet);
    println!("Twitter link: {}", twitter_link);
}
