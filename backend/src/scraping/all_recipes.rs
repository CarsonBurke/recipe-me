use fantoccini::{ClientBuilder, Locator};

use super::{ScrapeError, Site, DRIVER_ADDRESS};

/// Filter only recipes that have a sufficient amount of ratings

pub async fn scrape_all_recipes(site: &Site) -> Result<(), ScrapeError> {
    let recipe_hrefs = get_recipe_hrefs(site).await.unwrap();
    let root_url = "https://www.allrecipes.com/";

    for recipe_href in recipe_hrefs {}

    Ok(())
}

async fn get_recipe_hrefs(site: &Site) -> Result<Vec<String>, ScrapeError> {
    if let Ok(content) = fs::read(site.recipe_hrefs_path()).await {
        let hrefs: Vec<String> = serde_json::from_slice(&content).unwrap();
        println!("Loaded hrefs from file of len {}", hrefs.len());
        return Ok(hrefs);
    };

    let root_url = "https://www.allrecipes.com/";
    let start_url_relative = "recipes-a-z-6735880";
    let start_url = format!("{}{start_url_relative}", root_url);


    let client = ClientBuilder::native()
        .connect(DRIVER_ADDRESS)
        .await
        .map_err(ScrapeError::from)?;

    client.goto(root_url.as_str()).await.unwrap();
    if client.current_url().await.unwrap().as_str() != root_url {
        panic!("Failed to navigate to root url");
    }

    let category_hrefs = Vec::new();

    let category_els = client.find_all(Locator::Css("l-link-list__item")).await;

    for el in category_els {

        let Some(href) = el.attr("href").await.unwrap() else {
            continue;
        };

        category_hrefs.push(href);
    }

    let mut recipe_hrefs = Vec::new();

    for category_href in category_hrefs {

        let recipe_hrefs = Vec::new();

        client.goto(category_href.as_str()).await.unwrap();
        if client.current_url().await.unwrap().as_str() != category_href {
            panic!("Failed to navigate to category href");
        }

        let recipe_els = client.find_all(Locator::Css(".card__a")).await;

        for el in recipe_els {

            let Some(href) = el.attr("href").await.unwrap() else {
                continue;
            };

            recipe_hrefs.push(href);
        }
    }

    Ok(recipe_hrefs)
}
