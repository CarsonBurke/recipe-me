use fantoccini::{Client, ClientBuilder, Locator};
use ollama_rs::Ollama;

use crate::{
    db::db_conn,
    scraping::{
        ScrapedRecipe,
        generate::{
            generate_description, generate_ingredients, generate_instructions, generate_title,
        },
        is_recipe_url_in_db,
    },
};

use super::{DRIVER_ADDRESS, ScrapeError, Site};

/// Filter only recipes that have a sufficient amount of ratings

pub async fn scrape_all_recipes(site: &Site) -> Result<(), ScrapeError> {
    let recipe_hrefs = get_recipe_hrefs(site).await.unwrap();

    println!("Found {} recipes", recipe_hrefs.len());

    let db_conn = db_conn().await.unwrap();
    let client = ClientBuilder::native()
        .connect(DRIVER_ADDRESS)
        .await
        .map_err(ScrapeError::from)?;

    let ollama = Ollama::default();

    for recipe_href in recipe_hrefs {
        if is_recipe_url_in_db(&db_conn, &recipe_href).await {
            println!("recipe with url already in db, skipping");
            continue;
        }

        client.goto(&recipe_href).await.unwrap();
        if client.current_url().await.unwrap().as_str() != &recipe_href {
            continue;
        }

        let rating_count_el = client
            .find(Locator::Css(".mm-recipes-review-bar__rating-count"))
            .await;

        let rating_count = if let Ok(rating_count_el) = rating_count_el {
            let text = rating_count_el.text().await.unwrap();

            let text = text.replace("(", "");
            let text = text.replace(")", "");

            text.parse::<u32>().unwrap()
        } else {
            0
        };

        if rating_count < 3 {
            println!("Skipping recipe with less than 3 ratings");
        }

        let rating_avg_el = client
            .find(Locator::Css(".mm-recipes-review-bar__rating"))
            .await;

        let rating_avg = if let Ok(rating_avg_el) = rating_avg_el {
            let str = rating_avg_el.text().await.unwrap();

            str.parse::<f32>().unwrap()
        } else {
            0.0
        };

        let image_el = client
            .find(Locator::XPath("//*[@id='figure-article_1-0']/div/div/img"))
            .await;

        let image_url = if let Ok(image_el) = image_el {
            image_el.attr("src").await.unwrap()
        } else {
            None
        };

        let title_el = client
            .find(Locator::Css(".text-headline-400"))
            .await
            .unwrap();
        let title = generate_title(&ollama, title_el.text().await.unwrap()).await;

        let instructions_el = client
            .find(Locator::XPath("//*[@id='mntl-sc-block_1-0']"))
            .await
            .unwrap();
        let instructions_text = instructions_el.text().await.unwrap();
        let instructions = generate_instructions(&ollama, instructions_text).await;

        let ingredients_el = client
            .find(Locator::Css(".mm-recipes-structured-ingredients__list"))
            .await
            .unwrap();
        let ingredients_text = ingredients_el.text().await.unwrap();
        println!("ingredients text: {}", ingredients_text);
        let ingredients = generate_ingredients(&ollama, ingredients_text.clone()).await;
        println!("generated ingredients list");

        let description = generate_description(&ollama, title.clone(), ingredients_text).await;

        let time_el = client
            .find(Locator::XPath(
                "//*[@id='mm-recipes-details_1-0']/div[1]/div[3]/div[2]",
            ))
            .await
            .unwrap();
        let time = time_el.text().await.unwrap();

        let _ = ScrapedRecipe {
            title,
            url: recipe_href,
            description,
            instructions,
            ingredients,
            average_rating: rating_avg,
            ratings_count: rating_count,
            image_url,
            time: Some(time),
        }
        .try_write(&db_conn)
        .await;
    }

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
