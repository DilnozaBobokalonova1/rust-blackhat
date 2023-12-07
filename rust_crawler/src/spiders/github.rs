use crate::error::Error;
use async_trait::async_trait;
use regex::Regex;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct GithubSpider {
    http_client: Client,
    page_regex: Regex,
    expected_number_of_results: usize,
}

impl GithubSpider {
    pub fn new() -> Self {
        let http_timeout = Duration::from_secs(6);
        let mut headers = header::HeaderMap::new();

        //can only accept the JSON type of response from git's version 3
        headers.insert(
            "Accept",
            header::HeaderValue::from_static("application/vnd.github.v3+json"),
        );

        let http_client = Client::builder()
            .timeout(http_timeout)
            .default_headers(headers)
            //specifying chrome instead
            .user_agent("Chrome Safari")
            .build()
            .expect("spiders/github: Building HTTP Client");

        let page_regex =
            Regex::new(".*page=([0-9]*).*").expect("spiders/github: Compiling page regex");

        GithubSpider {
            http_client,
            page_regex,
            expected_number_of_results: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubItem {
    login: String,
    id: i64,
    node_id: String,
    html_url: String,
    avatar_url: String,
    site_admin: bool,
}

#[async_trait]
impl super::Spider for GithubSpider {
    //associated type item to avoid extensive outlining of the type used
    type Item = GithubItem;

    fn name(&self) -> String {
        String::from("github")
    }

    //https://api.github.com/orgs/google/public_members?per_page=100&page=1) does seem to get sent
    fn start_urls(&self) -> Vec<String> {
        vec!["https://api.github.com/orgs/google/public_members?per_page=100&page=1".to_string()]
    }

    async fn scrape(&self, url: String) -> Result<(Vec<GithubItem>, Vec<String>), Error> {
        //get json items from a passed in url to spider
        let items: Vec<GithubItem> = self.http_client.get(&url).send().await?.json().await?;

        let next_page_links = if items.len() == self.expected_number_of_results {
            //.*page=([0-9]*).* is the page regex defined earlier, so [1] is the current page number of passed in url
            let captures = self.page_regex.captures(&url).unwrap();
            //example: Captures({0: 0..69/"https://api.github.com/orgs/google/public_members?per_page=100&page=6", 1: 68..69/"6"})
            println!("the captures are shown to be {:?}", captures);
            let old_page_number = captures.get(1).unwrap().as_str().to_string();

            let mut new_page_number = old_page_number
                .parse::<usize>()
                .map_err(|_| Error::Internal("spider/github: parsing page number".to_string()))?;
            new_page_number += 1;

            //update the url to the next page number
            let next_url = url.replace(
                format!("&page={}", old_page_number).as_str(),
                format!("&page={}", new_page_number).as_str(),
            );
            vec![next_url]
        } else {
            Vec::new()
        };

        Ok((items, next_page_links))
    }

    async fn process(&self, item: Self::Item) -> Result<(), Error> {
        if item.site_admin {
            println!("SITE ADMIN FOUND");
        }
        println!("login: {}, html_url: {}, avatar_url: {}, site_admin: {}", item.login, item.html_url, item.avatar_url, item.site_admin);
        Ok(())
    }
}
