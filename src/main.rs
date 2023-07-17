use mastodon::Mastodon;

fn main() {
    let client = Mastodon::from_env();

    let posts = client.get_statuses("110699440670565345");

    for post in posts {
        let id = post.id;
        let caption = post.content;

        println!("DELETE {id}: {}", truncate(&caption));

        client.delete_status(&id);
    }
}

fn truncate(string: &str) -> String {
    string.chars().take(40).collect()
}
