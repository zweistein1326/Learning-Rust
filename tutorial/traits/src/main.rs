// Traits in Rust

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize() {
        // println the summary
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub metadata: String,
    pub retweet: bool,
}

impl Summary for Tweet {

}

pub trait Summary {
    fn summarize() {
        // println the summary
    }

    fn expand() {
        // println the expansion
    }
}

fn main() {
    let tweet = Tweet {
        ...
    };
    tweet.summarize();
    news_article.summarize();   

}

pub fn notify(item: &(impl Summary + Display)) {
    // notify about any struct that implements a summary
}

// AI tool -> summarize