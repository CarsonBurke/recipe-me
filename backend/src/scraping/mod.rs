use std::{error::Error, fmt::Display};

use fantoccini::{ClientBuilder, error::NewSessionError, wd::WebDriverCompatibleCommand};
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
    let root_url = "https://www.bbc.co.uk/";
    let start_url_relative = "food/recipes/a-z/a/1";
    let start_url = format!("{root_url}{start_url_relative}");

    let redirect_url = "https://www.bbc.co.uk/food/recipes";

    println!("root url");
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .map_err(ScrapeError::from)?;
    println!("connected");
    client.goto(start_url.as_str()).await.unwrap();
    if client.current_url().await.unwrap().as_str() != start_url {
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
        .find_all(fantoccini::Locator::Css(".az-keyboard__link"))
        .await
        .unwrap();

    println!("path links count {}", path_links.len());

    let mut path_hrefs = Vec::new();

    for le in path_links.iter() {
        let attr = le.attr("href").await.unwrap();

        let Some(href) = attr else {
            println!("no href for text content {}", le.text().await.unwrap());
            continue;
        };

        println!("found href for path link");

        path_hrefs.push(href);
    }

    println!("path hrefs count {}", path_hrefs.len());

    for href in path_hrefs {
        // Get all promo elements including paginated

        // Go to the promo page and take the relevant data
        // Filter out those without an image

        // Go to the next page if exists

        // So long as we are still matching the og path, we can keep going

        let mut moving_href = href; 

        loop {
            let url_split = moving_href.split("/").collect::<Vec<&str>>();
            println!("url split {url_split:?}");
            let url_end = url_split.last().unwrap();
            let url_pagination_str = url_end.split("#").collect::<Vec<&str>>()[0];
            let url_pagination = url_pagination_str.parse::<u32>().unwrap();

            let pre_url = url_split[0..url_split.len() - 1].join("/");
            println!("pre url {pre_url}");
            let new_url = pre_url + &format!("/{}", url_pagination + 1);
            moving_href = new_url.clone();

            println!("new url: {}", new_url);

            client.goto(new_url.as_str()).await;
            let client_url = client.current_url().await.unwrap();

            // If we get sent to the redirect url we know that we hit the pagination limit
            if client_url.as_str() == redirect_url {
                break;
            }

            let promos = client
                .find_all(fantoccini::Locator::Css(".promo"))
                .await
                .unwrap();

            let mut promos_hrefs = Vec::new();

            for el in promos
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
