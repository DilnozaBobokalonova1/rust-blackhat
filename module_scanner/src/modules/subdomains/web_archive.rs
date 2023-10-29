use crate::{
    modules::{Module, SubdomainModule},
    Error,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use url::Url;

pub struct WebArchive {}

impl WebArchive {
    pub fn new() -> Self {
        WebArchive {}
    }
}

impl Module for WebArchive {
    fn name(&self) -> String {
        return String::from("subdomains/webarchive");
    }

    fn description(&self) -> String {
        String::from("Use web.archive.org to find subdomains")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct WebArchiveResponse(Vec<Vec<String>>);

#[async_trait]
impl SubdomainModule for WebArchive {
    async fn enumerate(&self, domain: &str) -> Result<Vec<String>, Error> {
        let url = format!("https://web.archive.org/cdx/search/cdx?matchType=domain&fl=original&output=json&collapse=urlkey&url={}", domain);
        println!("requesting url for web_archive subdomain: {}", url);
        let res = reqwest::get(&url).await?;
        // println!("the following is the res of the provided webarchive url {:?}", res);

        if !res.status().is_success() {
            return Err(Error::InvalidHttpResponse(self.name()));
        }

        let web_archive_urls: WebArchiveResponse = match res.json().await {
            Ok(info) => info,
            Err(_) => return Err(Error::InvalidHttpResponse(self.name())),
        };

        println!(
            "the following is the web archive urls {:?}",
            web_archive_urls
        );

        let subdomains: HashSet<String> = web_archive_urls
            .0
            .into_iter()
            .flatten()
            //remove the "original" in order to avoid the parsing throwing invalid base url error
            .filter(|url_entry| url_entry != &"original")
            .filter_map(|url| match Url::parse(&url) {
                Ok(parsed_url) => {
                    println!("url parsed successfully {}", parsed_url);
                    Some(parsed_url.host_str().map(|host| host.to_string()))
                }
                Err(err) => {
                    log::error!("{}: error parsing url {}: {}", self.name(), url, err);
                    None
                }
            })
            .map(|url_val| url_val.unwrap())
            .collect();

        Ok(subdomains.into_iter().collect())
    }
}
