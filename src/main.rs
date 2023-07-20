use std::fs;
use std::path::Path;

use mastodon::Mastodon;
use serde::Serialize;

const DIR: &str = "posts/next";

fn main() {
    let client = Mastodon::from_env();

    let posts = parse_posts();

    println!();
    for post in posts {
        println!("=================");

        let path = format!("{DIR}/{}/", post.index);
        println!("--> {}", path);

        let caption = get_caption(&post);
        println!("{caption}");

        let image_paths = read_image_paths(&path);
        println!("--> media paths: {:?}", image_paths);
        let media_ids = client.upload_media(&image_paths);
        println!("    SUCCESS!");

        println!("--> media ids: {:?}", media_ids);
        client.post_media_status(&caption, &media_ids);
        println!("    SUCCESS!");

        let new_path = path.replace("/next", "/done");
        println!("move to: {}", new_path);
        fs::rename(path.to_string(), new_path).expect("move folder");
        println!("=================");
        println!();
    }
}

fn get_caption(post: &Post) -> String {
    let title = post.title.trim().to_string();

    let mut caption = title + " 💚";

    if !post.errata.is_empty() {
        caption += "\nEraroj:";
        for (bad, good) in &post.errata {
            caption += &format!("\n - '{}' -> '{}'", upper_first(&bad), upper_first(&good));
        }
    }

    caption += "\n#esperanto #garfield";

    caption += &format!(" [{}]", post.index);

    if is_old_index(&post.index) {
        caption += " (aĝa)";
    }

    caption
}

fn is_old_index(index: &str) -> bool {
    let number: u32 = index.parse().expect("index not a number");
    number < 100
}

fn read_image_paths(path: &str) -> Vec<String> {
    let esperanto = path.to_owned() + "esperanto.png";
    let english = path.to_owned() + "english.png";

    assert!(Path::new(&esperanto).exists());

    if Path::new(&english).exists() {
        vec![esperanto, english]
    } else {
        vec![esperanto]
    }
}

fn upper_first(string: &str) -> String {
    let mut chars = string.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_string().to_uppercase() + chars.as_str()
}

#[derive(Debug, Serialize)]
pub struct Post {
    index: String,
    title: String,
    errata: Vec<(String, String)>,
    english: bool,
}

fn parse_posts() -> Vec<Post> {
    let mut posts = Vec::new();

    let folders = fs::read_dir(DIR).expect("read dir").flatten();

    for folder in folders {
        let path = folder.path();
        let path = path.to_string_lossy().to_string();

        let index = folder.file_name();
        let index = index.to_string_lossy().to_string();

        let title = format!("{path}/title");
        let title = fs::read_to_string(&title).expect("read title file");

        let errata = format!("{path}/errata");
        let errata = if Path::new(&errata).exists() {
            parse_errata(fs::read_to_string(&errata).expect("read errata file"))
        } else {
            Vec::new()
        };

        assert!(Path::new(&format!("{path}/esperanto.png")).exists());

        let english = Path::new(&format!("{path}/english.png")).exists();

        posts.push(Post {
            index,
            title,
            errata,
            english,
        });
    }

    posts.sort_by_key(|post| post.index.parse::<u32>().expect("parse index"));

    posts
}

fn parse_errata(file: String) -> Vec<(String, String)> {
    file.lines()
        .map(|line| {
            let mut split = line.split(">>");
            (
                split.next().expect("parse errata").trim().to_string(),
                split.next().expect("parse errata").trim().to_string(),
            )
        })
        .collect()
}
