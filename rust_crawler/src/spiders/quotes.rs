use crate::error::Error;
use async_trait::async_trait;
use fantoccini::{Client, ClientBuilder};
use select::{
    document::Document,
    predicate::{Class, Name, Predicate},
};
use tokio::sync::Mutex;

pub struct QuotesSpider {
    webdriver_client: Mutex<Client>,
}

impl QuotesSpider {
    pub async fn new() -> Result<Self, Error> {
        let mut caps = serde_json::map::Map::new();
        let chrome_opts = serde_json::json!({"args": ["--headless", "--disable-gpu"]});
        caps.insert("goog:chromeOptions".to_string(), chrome_opts);
        let webdriver_client = ClientBuilder::rustls()
            .capabilities(caps)
            .connect("http://localhost:4444")
            .await?;

        Ok(QuotesSpider {
            webdriver_client: Mutex::new(webdriver_client),
        })
    }
}

#[derive(Debug)]
pub struct QuotesItem {
    quote: String,
    author: String,
}

#[async_trait]
impl super::Spider for QuotesSpider {
    type Item = QuotesItem;

    fn name(&self) -> String {
        String::from("quotes")
    }

    fn start_urls(&self) -> Vec<String> {
        vec!["https://quotes.toscrape.com/js".to_string()]
    }

    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
        let mut items = Vec::new();
        let html = {
            let webdriver = self.webdriver_client.lock().await;
            webdriver.goto(&url).await?;
            webdriver.source().await?
        };
        // println!("the html source for the quotes page is {}", html);

        let document = Document::from(html.as_str());
        // println!("the document looks as the following: {:?}", document);
        
        let quotes = document.find(Class("quote"));
        for quote in quotes {
            let mut spans = quote.find(Name("span"));
            // println!("the spans for within quote doc is: {:?}", spans);
            let quote_span = spans.next().unwrap();
            let quote_str = quote_span.text().trim().to_string();

            let author = spans
                .next()
                .unwrap()
                .find(Class("author"))
                .next()
                .unwrap()
                .text()
                .trim()
                .to_string();

            items.push(QuotesItem {
                quote: quote_str,
                author,
            });
        }
        // println!("the items we have for the quotes page are: {:?}", items);

        let next_page_link = document
            .find(
                Class("pager")
                    .descendant(Class("next"))
                    .descendant(Name("a")),
            )
            .filter_map(|n| n.attr("href"))
            .map(|url| self.normalize_url(url))
            .collect::<Vec<String>>();
        // println!("next page link looks as the following: {:?}", next_page_link);
        Ok((items, next_page_link))
    }
    
    async fn process(&self, item: Self::Item) -> Result<(), Error> {
        println!("{}", item.quote);
        println!("by {}/n", item.author);
        Ok(())
    }
}

impl QuotesSpider {
    fn normalize_url(&self, url: &str) -> String {

        // println!("passed in url to normalize looks as: {}", url);
        let url = url.trim();
        if url.starts_with("/") {
            return format!("https://quotes.toscrape.com{}", url);
        }
        // println!("after normalization: {}", url);
        url.to_string()
    }
}
