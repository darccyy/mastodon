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

        // println!("==> PRESS ENTER TO CONTINUE!");
        // let mut line = String::new();
        // std::io::stdin().read_line(&mut line).unwrap();
        // println!("posting...");

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
    format!(
        "{} 💚\n\n#esperanto #garfield #mondodakomiksoj [{}]",
        post.title.trim(), post.index
    )
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

#[derive(Debug, Serialize)]
pub struct Post {
    index: String,
    title: String,
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

        assert!(!Path::new(&format!("{path}/errata")).exists());
        assert!(Path::new(&format!("{path}/esperanto.png")).exists());
        assert!(Path::new(&format!("{path}/english.png")).exists());

        posts.push(Post {
            index,
            title,
        });
    }

    posts.sort_by_key(|post| post.index.parse::<u32>().expect("parse index"));

    posts
}
