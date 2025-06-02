use std::fs;

use fantoccini::{ClientBuilder, Locator};
use ollama_rs::Ollama;

use crate::{db::db_conn, scraping::{
    is_recipe_url_in_db, generate::{generate_description, generate_ingredients, generate_instructions, generate_title}, ScrapeError, ScrapedRecipe, DRIVER_ADDRESS
}};

use super::Site;

pub async fn scrape_bbc_food(site: &Site) -> Result<(), ScrapeError> {
    let recipe_hrefs = get_recipe_hrefs(site).await.unwrap();
    let root_url = "https://www.bbc.co.uk";

    let client = ClientBuilder::native()
        .connect(DRIVER_ADDRESS)
        .await
        .map_err(ScrapeError::from)?;

    let ollama = Ollama::default();
    let db_conn = db_conn().await.unwrap();

    for (i, recipe_href) in recipe_hrefs.iter().enumerate() {
        let recipe_url = format!("{root_url}{recipe_href}");

        if is_recipe_url_in_db(&db_conn, &recipe_url).await {
            println!("recipe with url already in db, skipping");
            continue;
        }

        println!("Navigating to {recipe_url}");
        client.goto(recipe_url.as_str()).await.unwrap();
        if client.current_url().await.unwrap().as_str() != recipe_url {
            panic!("Failed to navigate to recipe url");
        }

        let title_el = client.find(Locator::Id("main-heading")).await.unwrap();
        let title_text = title_el.text().await.unwrap();

        let title = generate_title(&ollama, title_text).await;

        let ratings_count_el = client
            .find(Locator::XPath(
                "//*[@id='main-content']/div/div[2]/div[1]/div[2]/div/span[3]",
            ))
            .await
            .unwrap();
        println!(
            "rating count text {:#?}",
            ratings_count_el.text().await.unwrap()
        );
        let ratings_text = ratings_count_el.text().await.unwrap();
        let ratings_split = ratings_text.split(" ").collect::<Vec<&str>>();
        let ratings_count = ratings_split[0].parse::<u32>().unwrap_or(0);

        let average_rating_el = client
            .find(Locator::XPath(
                "//*[@id='main-content']/div/div[2]/div[1]/div[2]/div/span[1]",
            ))
            .await
            .unwrap();
        println!(
            "avg rating text {:#?}",
            average_rating_el.text().await.unwrap()
        );

        let average_rating = average_rating_el
            .text()
            .await
            .unwrap()
            .parse::<f32>()
            .unwrap();

        let instructions_parent = client
            .find(Locator::XPath(
                "//*[@id='main-content']/div/div[2]/div[4]/div/div[3]/div/ol",
            ))
            .await
            .unwrap();
        let ingredients_text = instructions_parent.text().await.unwrap();

        println!(
            "instruction text {:#?}",
            instructions_parent.text().await.unwrap()
        );

        let instructions =
            generate_instructions(&ollama, instructions_parent.text().await.unwrap()).await;

        let ingredients_parent = client
            .find(Locator::XPath(
                "//*[@id='main-content']/div/div[2]/div[4]/div/div[1]/div",
            ))
            .await
            .unwrap();

        println!(
            "ingredient text {:#?}",
            ingredients_parent.text().await.unwrap()
        );

        let ingredients_vec = generate_ingredients(&ollama, ingredients_text.clone()).await;

        let description =
            generate_description(&ollama, title.clone(), ingredients_text.clone()).await;

        let mut time_el = client
            .find(Locator::XPath(
                "//*[@id='main-content']/div/div[2]/div[2]/div/div[2]/dl/div[2]/dd",
            ))
            .await;

        if time_el.is_err() {
            time_el = client
                .find(Locator::XPath(
                    "//*[@id='main-content']/div/div[2]/div[2]/div/div[1]/dl/div[2]/dd",
                ))
                .await
        };

        let time = if let Ok(time_el) = time_el {
            time_el.text().await.ok()
        } else {
            None
        };

        let image_el = client
            .find(Locator::XPath(
                "//*[@id='main-content']/div/div[2]/div[2]/div/div[1]/span/img",
            ))
            .await;

        let image_url = if let Ok(image_el) = image_el {
            let image_url = image_el.attr("src").await.unwrap();
            image_url
        } else {
            None
        };

        let scraped_recipe = ScrapedRecipe {
            title,
            ingredients: ingredients_vec,
            description,
            instructions,
            ratings_count,
            average_rating,
            time,
            url: recipe_url,
            image_url,
        };

        println!("Scraped recipe: {:#?}", scraped_recipe);

        scraped_recipe.try_write(&db_conn).await
    }

    Ok(())
}

async fn get_recipe_hrefs(site: &Site) -> Result<Vec<String>, ScrapeError> {
    if let Some(cahced_hrefs) = site.get_cached_hrefs().await {
        return Ok(cahced_hrefs);
    };

    let root_url = "https://www.bbc.co.uk/";
    let start_url_relative = "food/recipes/a-z/a/1";
    let start_url = format!("{}{start_url_relative}", root_url);

    let redirect_url = "https://www.bbc.co.uk/food/recipes";

    println!("root url");
    let client = ClientBuilder::native()
        .connect(DRIVER_ADDRESS)
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

    fs::write(
        "./path_links.json",
        serde_json::to_string(&path_hrefs).unwrap(),
    )
    .unwrap();

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

            let _  = client.goto(new_url.as_str()).await;
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

            for el in promos {
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

    site.write_relative_hrefs(&all_promo_hrefs).await;

    Ok(all_promo_hrefs)
}
