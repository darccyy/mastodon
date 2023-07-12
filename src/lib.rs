use std::fmt::Display;
use reqwest::{blocking::Client, header};

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

    fn api_url(&self, path: impl Display) -> String {
        format!("https://{instance}/api/v1/{path}", instance = self.instance)
    }

    pub fn post_status(&self, text: &str) {
        let res = self
            .client
            .post(&self.api_url("statuses"))
            .form(&hashmap! {
                "status" => text,
            })
            .send()
            .expect("Failed to send post request");

        println!("STATUS: {}", res.status());
    }
}
