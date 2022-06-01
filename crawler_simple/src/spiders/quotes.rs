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
        let chrome_opts = serde_json::json!({ "args": ["--headless", "--disable-gpu"] });
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

#[derive(Debug, Clone)]
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
        vec!["https://www.vcg.com/creative-image/fengjing/".to_string()]
    }

    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error> {
        let mut items = Vec::new();
        let html = {
            let mut webdriver = self.webdriver_client.lock().await;
            webdriver.goto(&url).await?;
            webdriver.source().await?
        };

        let mut next_pages_link: Vec<String> = Vec::new();

        //println!("{:#?}", html.as_str());
        let document = Document::from(html.as_str());
        let quotes = document.select(Name("a"));
        for node in quotes {
            if let Some(class) = node.attr("class") {
                if class == "imgWaper" {
                    if let Some(href) = node.attr("href") {
                        if let Some(title) = node.attr("title") {
                            //println!("{} ({:?})", title, href);
                            items.push(QuotesItem {
                                quote: String::from(href),
                                author: String::from(title),
                            });
                        }
                    }
                }
                
                if class == "paginationNowPage" {
                    //println!("{} ({:?})", class, node.next().unwrap());
                    let next_page_node = node.next().unwrap();
                    if let Some(href) = next_page_node.attr("href") {
                        next_pages_link.push(String::from(href));
                    }
                }
            }
        }

        Ok((items, next_pages_link))
    }

    async fn process(&self, item: Self::Item) -> Result<(), Error> {
        println!("{}", item.quote);
        println!("by {}\n", item.author);
        Ok(())
    }
}

impl QuotesSpider {
    fn normalize_url(&self, url: &str) -> String {
        let url = url.trim();

        if url.starts_with("/") {
            return format!("https://quotes.toscrape.com{}", url);
        }

        return url.to_string();
    }
}
