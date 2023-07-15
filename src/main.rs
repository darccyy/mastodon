use std::fs;

// use mastodon::Mastodon;
use mastodon::json;

json! {
    OutboxFile {
        orderedItems: Vec<OrderedItem>,
    }
    OrderedItem {
        id: String,
        r#type: String,
        object: Object,
    }
    Object {
        content: String,
        replies: Replies,
        attachment: Vec<Attachment>,
    }
    Replies {
        id: String
    }
    Attachment {
        url: String,
    }
}

#[derive(Debug)]
struct Post {
    id: String,
    caption: String,
    media_urls: Vec<String>,
    replies_url: String,
}

fn main() {
    // let client = Mastodon::from_env();

    let file = fs::read_to_string("outbox.fmt.json").expect("read file");

    let json: OutboxFile = serde_json::from_str(&file).expect("parse json");

    // println!("{:?}", json);

    let mut posts = Vec::new();

    for item in json.orderedItems {
        let id = item.id;
        let caption = item.object.content;
        let replies_url = item.object.replies.id;

        let caption = caption
            .split("<a href=\"https://mastodon.world/tags")
            .next()
            .map(ToString::to_string)
            .unwrap_or(caption);

        let media_urls: Vec<_> = item
            .object
            .attachment
            .into_iter()
            .map(|att| format!("https://s3.eu-central-2.wasabisys.com{}", att.url))
            .collect();

        if media_urls.is_empty() {
            continue;
        }

        posts.push(Post {
            id,
            caption,
            media_urls,
            replies_url,
        });
    }

    let json = format!("{:#?}", posts);

    fs::write("posts.json", json).expect("write file");
}
