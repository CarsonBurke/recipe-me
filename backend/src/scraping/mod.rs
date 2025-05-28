use std::{error::Error, fmt::Display};

use fantoccini::{error::NewSessionError, wd::WebDriverCompatibleCommand, ClientBuilder};
use futures::StreamExt;

#[derive(Debug)]
pub enum ScrapeError {
    FailedGeneral,
    FantocciniCmdError(fantoccini::error::CmdError),
    FantocciniNewSessionError(fantoccini::error::NewSessionError),
}

impl Error for ScrapeError {}

impl Display for ScrapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to scrape")
    }
}

impl From<fantoccini::error::CmdError> for ScrapeError {
    fn from(err: fantoccini::error::CmdError) -> Self {
        ScrapeError::FantocciniCmdError(err)
    }
}

impl From<NewSessionError> for ScrapeError {
    fn from(err: NewSessionError) -> Self {
        ScrapeError::FantocciniNewSessionError(err)
    }
}

pub async fn scrape_bbc_food() -> Result<(), ScrapeError> {
    let root_url = "https://www.bbc.co.uk/food/recipes/a-z/a/1";
    println!("root url");
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await.map_err(ScrapeError::from)?;

    client.goto(root_url).await.unwrap();
    if client.current_url().await.unwrap().as_str() != root_url {
        panic!("Failed to navigate to root url");
    }

    // let recipes_container = client.find(fantoccini::Locator::Css(".promo")).await?;

    let mut all_promo_hrefs = Vec::new();

    /* for el in recipes {
        /* println!(
            "Found recipe with href: {}",
            el.get_attribute("href")().await.unwrap()
        ); */

        let href = el.attr("href").await.unwrap();
        if let Some(href) = href {
            println!("Found recipe with href: {}", href);
            hrefs.push(href);
        }
    } */

    let path_links = client
        .find_all(fantoccini::Locator::Css("az-keyboard__link"))
        .await.unwrap();

    println!("path links count {}", path_links.len());

    let mut path_hrefs = Vec::new();

    for le in path_links.iter() {
        path_hrefs.push(le.attr("href").await.unwrap().unwrap());
    }

    println!("path hrefs count {}", path_hrefs.len());

    for href in path_hrefs {
        // Get all promo elements including paginated

        // Go to the promo page and take the relevant data
        // Filter out those without an image

        // Go to the next page if exists

        // So long as we are still matching the og path, we can keep going

        loop {
            let url = client.current_url().await.unwrap();
            let url_split = url
                .as_str()
                .split("/")
                .collect::<Vec<&str>>();
            let url_pagination_str = url_split.last().unwrap();
            let url_pagination = url_pagination_str.parse::<u32>().unwrap();

            let new_url =
                url_split[0..url_split.len() - 2].join("/") + &format!("/{}", url_pagination + 1);

            client.goto(new_url.as_str()).await;
            let client_url = client.current_url().await.unwrap();

            if client_url.as_str() == root_url {
                break;
            }

            let promos = client.find_all(fantoccini::Locator::Css(".promo")).await.unwrap();

            let mut promos_hrefs = Vec::new();

            for el in client
                .find_all(fantoccini::Locator::Css(".promo"))
                .await
                .unwrap()
                .iter()
            {
                let href = el.attr("href").await.unwrap();
                if let Some(href) = href {
                    println!("Found recipe with href: {}", href);
                    promos_hrefs.push(href);
                }
            }

            all_promo_hrefs.append(&mut promos_hrefs);
        }
    }

    println!("all hrefs of count {}", all_promo_hrefs.len());
    // println!("all hrefs: {:#?}", all_promo_hrefs);

    Ok(())
}
