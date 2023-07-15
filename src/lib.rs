mod json;

use reqwest::{
    blocking::{multipart, Client, Response},
    header, StatusCode,
};
use std::{env, fmt::Display, process};

json! {
    MediaResponse {
        id: String,
        url: String,
    }
}

/// Client to Mastodon API
pub struct Mastodon {
    /// Reqwest client, with authorization headers
    client: Client,
    /// Instance domain name
    instance: String,
}

impl Mastodon {
    // Create client with credentials from environment
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        let instance = env::var("INSTANCE").expect("No `INSTANCE` environment variable");
        let access_token =
            env::var("ACCESS_TOKEN").expect("No `ACCESS_TOKEN` environment variable");

        Mastodon::new(instance, access_token)
    }

    // Create new client
    pub fn new(instance: impl Into<String>, access_token: impl Display) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", access_token)
                .try_into()
                .expect("Failed to convert string to header value"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build client");

        Self {
            client,
            instance: instance.into(),
        }
    }

    /// Get API URL from instance and API path
    fn api_url(&self, version: u32, path: impl Display) -> String {
        format!(
            "https://{instance}/api/v{version}/{path}",
            instance = self.instance
        )
    }

    /// Upload media
    ///
    /// Does not post a status
    pub fn upload_media(&self, paths: &[String]) -> Vec<String> {
        let mut media_ids = Vec::new();

        for path in paths {
            let form = multipart::Form::new()
                .file("file", path)
                .expect("Failed to build multipart form");

            let res = self
                .client
                .post(&self.api_url(2, "media"))
                .multipart(form)
                .send()
                .expect("Failed to send post request");
            check_status_panic(&res);

            let res = res.text().expect("Failed to get response text");
            let media: MediaResponse =
                serde_json::from_str(&res).expect("Failed to parse response json");

            media_ids.push(media.id);
        }

        media_ids
    }

    /// Post a status with already-uploaded media, given ids
    pub fn post_media_status(&self, text: &str, media_ids: &[String]) {
        let json = serde_json::json!({
            "status": text,
            "media_ids": media_ids,
        });

        let res = self
            .client
            .post(&self.api_url(1, "statuses"))
            .json(&json)
            .send()
            .expect("Failed to send post request");
        check_status_panic(&res);
    }
}

/// Panic if fetch response is not successful
fn check_status_panic(res: &Response) {
    if res.status() == StatusCode::TOO_MANY_REQUESTS {
        println!("<<< RATE LIMITED!!! Try again in 30m >>>");
        process::exit(1);
    }

    if let Err(status) = res.error_for_status_ref() {
        panic!("Unsuccessful request: {:#?}", status);
    }
}
