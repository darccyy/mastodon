use std::fmt::Display;
use reqwest::{blocking::{Client, multipart, Response}, header};

pub struct Mastodon {
    client: Client,
    instance: String,
}

/// Create `Deserialize`-able structs for JSON responses
macro_rules! json {
    ( $(
       $struct:ident $tt:tt
    )* ) => { $(
        json!(@single $struct $tt);
    )* };

    (@single $struct:ident { $( $key:ident : $type:ty),* $(,)? } ) => {
        /// Deserialized JSON
        #[derive(Debug, serde::Deserialize)]
        pub struct $struct {
            $( pub $key: $type, )*
        }
    };

    (@single $struct:ident ( $( $type:ty ),* $(,)? ) ) => {
        /// Deserialized JSON
        #[derive(Debug, serde::Deserialize)]
        pub struct $struct (
            $( pub $type, )*
        );
    };
}

json!{
    MediaResponse {
        id: String,
        url: String,
    }
}

impl Mastodon {
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

    fn api_url(&self, version: u32, path: impl Display) -> String {
        format!("https://{instance}/api/v{version}/{path}", instance = self.instance)
    }

    pub fn upload_media(&self, paths: &[String]) -> Vec<String> {

        let mut media_ids = Vec::new();

        for path in paths {
        let form = multipart::Form::new().file("file", path).expect("Failed to build multipart form");

        let res = self
            .client
            .post(&self.api_url(2, "media"))
            .multipart(form)
            .send()
            .expect("Failed to send post request");
        check_status_panic(&res);

        let res = res.text().expect("Failed to get response text");
        let media: MediaResponse = serde_json::from_str(&res).expect("Failed to parse response json");

        media_ids.push(media.id);
        }

        media_ids
    }

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

fn check_status_panic(res: &Response) {
    if let Err(status) = res.error_for_status_ref() {
        panic!("Unsuccessful request: {:#?}", status);
    }
}

