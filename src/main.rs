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

        let id = id
            .split("https://mastodon.world/users/garfieldeo/statuses/")
            .nth(1)
            .unwrap()
            .split("/")
            .next()
            .unwrap()
            .to_string();

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
        });
    }

    fs::write("posts2.json", format!("{:?}", posts)).expect("write file");

    let mut output = Vec::new();

    for post in posts {
        let Post {
            id,
            caption,
            media_urls,
        } = post;

        output.push(
            "{".to_string()
                + "\n"
                + &format!(r#"    "id": "{id}""#)
                + "\n"
                + &format!(r#"    "caption": "{caption}""#)
                + "\n"
                + &format!(r#"    "media_urls": ["{}"]"#, media_urls.join("\", \""))
                + "\n"
                + &format!(r#"    "replies": [  ]"#)
                + "\n"
                + "}",
        );
    }

    let output = output.join(",\n");
    fs::write("posts.json", format!("[\n{}\n]", output)).expect("write file");
}
