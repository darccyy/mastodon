use std::{env, fs};

use mastodon::Mastodon;

fn main() {
    dotenv::dotenv().ok();

    let instance = env::var("INSTANCE").expect("No `INSTANCE` environment variable");
    let access_token = env::var("ACCESS_TOKEN").expect("No `ACCESS_TOKEN` environment variable");

    let client = Mastodon::new(
        instance,
        access_token,
    );

    let posts = fs::read_dir("./posts/next").expect("Failed to read posts folder");

    // Sort
    let mut posts: Vec<_> = posts.map(|r|r.unwrap()).collect();
    posts.sort_by_key(|dir|dir.path());

    for file in posts {
        let path = file.path();
        let path = path.to_string_lossy().to_string();
        println!("path: {}", path);


        let caption = fs::read_to_string(path.to_string() + "/caption.txt").expect("Failed to read caption file in post");

        let caption = caption.replace("\n\n", "\n");
        let caption = caption.replace("\n💚 amu lasagnon 💚", "");
        let caption = caption + " #bot";
        println!("caption: {}", caption);

        let mut image_paths = Vec::new();
        let images = fs::read_dir(path.to_string() + "/images").expect("Failed to read images folder in post");
        for image in images.flatten() {
            let image_path = image.path();
            let image_path = image_path.to_string_lossy().to_string();

            image_paths.push(image_path);
        }

        println!("post medias: {:?}", image_paths);
        let media_ids = client.upload_media(&image_paths);
        println!("SUCCESS!");

        println!("media ids: {:?}", media_ids);
        client.post_media_status(&caption, &media_ids);
        println!("SUCCESS!");

        println!("move to: {}", path.replace("/next", "/done"));
        fs::rename(path.to_string(), path.replace("/next", "/done")).expect("Failed to move post folder into done directory");

        println!(" === SUCCESS! === ");
    }

}
