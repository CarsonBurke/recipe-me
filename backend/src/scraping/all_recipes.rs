use fantoccini::{Client, ClientBuilder, Locator};

use crate::db::db_conn;

use super::{DRIVER_ADDRESS, ScrapeError, Site};

/// Filter only recipes that have a sufficient amount of ratings

pub async fn scrape_all_recipes(site: &Site) -> Result<(), ScrapeError> {
    let recipe_hrefs = get_recipe_hrefs(site).await.unwrap();

    println!("Found {} recipes", recipe_hrefs.len());
    panic!("Stopped");

    let db = db_conn().await.unwrap();
    let client = ClientBuilder::native()
        .connect(DRIVER_ADDRESS)
        .await
        .map_err(ScrapeError::from)?;

    let root_url = "https://www.allrecipes.com/";

    for recipe_href in recipe_hrefs {}

    Ok(())
}

async fn get_recipe_hrefs(site: &Site) -> Result<Vec<String>, ScrapeError> {
    if let Some(cahced_hrefs) = site.get_cached_hrefs().await {
        return Ok(cahced_hrefs);
    };

    let root_url = "https://www.allrecipes.com/";
    let start_url_relative = "recipes-a-z-6735880";
    let start_url = format!("{}{start_url_relative}", root_url);

    let client = ClientBuilder::native()
        .connect(DRIVER_ADDRESS)
        .await
        .map_err(ScrapeError::from)?;

    client.goto(start_url.as_str()).await.unwrap();
    if client.current_url().await.unwrap().as_str() != start_url {
        panic!("Failed to navigate to root url");
    }

    println!("Navigated to url {}", start_url);

    let mut category_hrefs = Vec::new();

    let category_els = client
        .find_all(Locator::Css(".mntl-link-list__link"))
        .await
        .unwrap();
    println!("found category els {}", category_els.len());
    for el in category_els {
        let Some(href) = el.attr("href").await.unwrap() else {
            println!("No href for el");
            continue;
        };

        category_hrefs.push(href);
    }

    println!("found category hrefs {}", category_hrefs.len());

    let mut recipe_hrefs = Vec::new();

    for category_href in category_hrefs {
        client.goto(category_href.as_str()).await.unwrap();
        if client.current_url().await.unwrap().as_str() != category_href {
            panic!("Failed to navigate to category href");
        }

        let mut internal_categories_href = Vec::new();

        let internal_category_els = client
            .find_all(Locator::Css(".mntl-taxonomy-nodes__link"))
            .await
            .unwrap();

        for el in internal_category_els {
            let Some(href) = el.attr("href").await.unwrap() else {
                continue;
            };

            internal_categories_href.push(href);
        }

        println!(
            "Found {} internal categories",
            internal_categories_href.len()
        );

        if internal_categories_href.is_empty() {
            recipe_hrefs.extend(get_recipe_hrefs_on_current_page(&client).await);
            continue;
        }

        for internal_category_href in internal_categories_href {
            client.goto(internal_category_href.as_str()).await.unwrap();
            if client.current_url().await.unwrap().as_str() != internal_category_href {
                panic!("Failed to navigate to internal category href");
            }

            recipe_hrefs.extend(get_recipe_hrefs_on_current_page(&client).await);
        }
    }

    println!("Found total of {} recipes", recipe_hrefs.len());

    site.write_relative_hrefs(&recipe_hrefs).await;

    Ok(recipe_hrefs)
}

async fn get_recipe_hrefs_on_current_page(client: &Client) -> Vec<String> {
    let recipe_els = client
        .find_all(Locator::Css(".mntl-document-card"))
        .await
        .unwrap();

    let mut recipe_hrefs = Vec::new();

    for el in recipe_els {
        let Some(href) = el.attr("href").await.unwrap() else {
            continue;
        };

        recipe_hrefs.push(href);
    }

    println!("Found {} recipes on page", recipe_hrefs.len());

    recipe_hrefs
}
