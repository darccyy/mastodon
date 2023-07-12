use std::fmt::Display;
use reqwest::{blocking::{Client, multipart}, header};

pub struct Mastodon {
    client: Client,
    instance: String,
}

macro_rules! hashmap {
    ( $(
        $k:expr => $v:expr
    ),* $(,)? ) => {{
        let mut hm = ::std::collections::HashMap::new();
        $(
            hm.insert($k, $v);
        )*
        hm
    }}
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

    pub fn post_text(&self, text: &str) {
        let res = self
            .client
            .post(&self.api_url(1, "statuses"))
            .form(&hashmap! {
                "status" => text,
            })
            .send()
            .expect("Failed to send post request");

        println!("STATUS: {:?}", res.status());
    }

    pub fn post_image(&self, path: &str, text: &str) {
        let form = multipart::Form::new().file("file", path).expect("Failed to build multipart form");

        let res = self
            .client
            .post(&self.api_url(2, "media"))
            .multipart(form)
            .send()
            .expect("Failed to send post request");

        // println!("{:#?}", res);

        println!("STATUS: {:?}", res.status());

        let text = res.text().expect("Failed to get response text");

        let media: MediaResponse = serde_json::from_str(&text).expect("Failed to parse response json");

        println!("MEDIA ID: {}", media.id);
        println!("MEDIA URL: {}", media.url);

        let res = self
            .client
            .post(&self.api_url(1, "statuses"))
            .form(&hashmap! {
                "status" => text,
                "media_ids" => format!("[{}]", media.id),
            })
            .send()
            .expect("Failed to send post request");

        println!("STATUS: {:?}", res.status());
    }
}
