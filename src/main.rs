use std::env;

use mastodon::Mastodon;

fn main() {
    dotenv::dotenv().ok();

    let instance = env::var("INSTANCE").expect("No `INSTANCE` environment variable");
    let access_token = env::var("ACCESS_TOKEN").expect("No `ACCESS_TOKEN` environment variable");

    let client = Mastodon::new(
        instance,
        access_token,
    );

    client.post_image("./ok.png", "some caption blah blah");
}
