use std::{error::Error, fmt::Display};

use fantoccini::{ClientBuilder, Locator, error::NewSessionError, wd::WebDriverCompatibleCommand};
use futures::StreamExt;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use tokio::fs;

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

pub async fn scrape() {
    let sites = vec![Site::new(
        "bbc_food".to_string(),
        "https://www.bbc.co.uk/food/".to_string(),
    )];

    for site in sites {
        let recipes = match site.name.as_str() {
            "bbc_food" => scrape_bbc_food(&site).await.unwrap(),
            _ => panic!("Unknown site"),
        };

        println!("scraped recipes for site {}: {}", site.name, recipes.len());
    }
}

#[derive(Debug)]
pub struct Ingredient {
    name: String,
    description: String,
    amount: f32,
}

fn ingredients_from_response(response: String) -> Vec<Ingredient> {
    let items = response.split("; ").collect::<Vec<&str>>();

    let mut ingredients: Vec<Ingredient> = Vec::new();

    for item in items {
        println!("item {item}");
        let components = item.split("|").collect::<Vec<&str>>();

        println!("components: {components:?}");

        let Some(amount_str) = components.get(0) else {
            continue;
        };
        let Ok(amount) = amount_str.parse::<f32>() else {
            continue;
        };
        let Some(description) = components.get(1) else {
            continue;
        };
        let Some(name) = components.get(2) else {
            continue;
        };

        let ingredient = Ingredient {
            name: name.to_string(),
            description: description.to_string(),
            amount,
        };
        ingredients.push(ingredient);
    }

    ingredients
}

#[derive(Debug)]
pub struct ScrapedRecipe {
    title: String,
    ingredients: Vec<Ingredient>,
    instructions: String,
    ratings_count: u32,
    average_rating: f32,
    time: String,
    url: String,
}

pub async fn scrape_bbc_food(site: &Site) -> Result<Vec<ScrapedRecipe>, ScrapeError> {
    let recipe_hrefs = get_bbc_food_recipe_hrefs(site).await.unwrap();
    let root_url = "https://www.bbc.co.uk";

    let mut recipes: Vec<ScrapedRecipe> = Vec::new();

    let client = ClientBuilder::native()
        .connect("http://localhost:33319")
        .await
        .map_err(ScrapeError::from)?;

    let ollama = Ollama::default();

    for recipe_href in recipe_hrefs {
        let recipe_url = format!("{root_url}{recipe_href}");
        println!("Navigating to {recipe_url}");
        client.goto(recipe_url.as_str()).await.unwrap();
        if client.current_url().await.unwrap().as_str() != recipe_url {
            panic!("Failed to navigate to recipe url");
        }

        let title_el = client.find(Locator::Id("main-heading")).await.unwrap();
        let title = title_el.text().await.unwrap();

        let prompt = format!(
            "Generate a SINGLE suitable, somewhat SEO and click-friendly but primarily descriptive alternative title for the following recipe title. PROVIDE NO OTHER TEXT IN YOUR ANSWER. Title: {title}"
        );
        let title_gen = ollama
            .generate(GenerationRequest::new("gemma3:latest".to_string(), prompt))
            .await
            .unwrap()
            .response;

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

        println!(
            "instruction text {:#?}",
            instructions_parent.text().await.unwrap()
        );

        let prompt_instructions = "Rewrite this recipe instruction to improve readability, grammer, staying to the point and being professional. Keep it to a reasonable length, some detail over brevity where it would benefit a reader. Avoid writing to statically or procedurally: this is not an essay. PROVIDE NO OTHER TEXT IN YOUR ANSWER. Base it on the following instructions:".to_string();
        let prompt = format!(
            "{prompt_instructions} {}",
            instructions_parent.text().await.unwrap()
        );
        let instructions = ollama
            .generate(GenerationRequest::new("gemma3:latest".to_string(), prompt))
            .await
            .unwrap()
            .response;

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

        let prompt_instructions = "Generate a list of ingredients based on the following provided list. Put all content on one line, seperating each ingredient by a semicolon ';'. For each ingredient, remove uncessary words like 'of', use only one unit of measurement and infer from the text (g for grams, tsp stays as tsp, etc.) and seperate each part of the ingredient into exactly 3 pieces: quantity (unsigned integer), descriptors (string), and name (string); if there is no quantity, use '1'. There must be one quantity, descriptors, and name for each ingredient. Ingredients must be separated from each other by a '|'. For example '300 grams of crushed garlic' should be separated into '300|grams, crushed|garlic'. Or another example: '1 large egg, beaten with 1 tsp whole milk' should turn into two ingredients: '1|large, beaten|egg; 1|tsp|whole milk'. Another example: 'handfull of parsley' should turn into '1|handfull|parsley'. PROVIDE NO OTHER TEXT IN YOUR ANSWER. Apply to the following ingredients list:".to_string();
        let prompt = format!(
            "{prompt_instructions} {}",
            ingredients_parent.text().await.unwrap()
        );
        let ingredients = ollama
            .generate(GenerationRequest::new("gemma3:latest".to_string(), prompt))
            .await
            .unwrap()
            .response;

        let ingredients_vec = ingredients_from_response(ingredients);

        let time_el = client
            .find(Locator::XPath(
                "//*[@id='main-content']/div/div[2]/div[2]/div/div[2]/dl/div[2]/dd",
            ))
            .await
            .unwrap_or(
                client
                    .find(Locator::XPath(
                        "//*[@id='main-content']/div/div[2]/div[2]/div/div[1]/dl/div[2]/dd",
                    ))
                    .await
                    .unwrap(),
            );
        let time = time_el.text().await.unwrap();

        let scraped_recipe = ScrapedRecipe {
            title: title_gen,
            ingredients: ingredients_vec,
            instructions,
            ratings_count,
            average_rating,
            time,
            url: recipe_url,
        };

        println!("Scraped recipe: {:#?}", scraped_recipe);

        recipes.push(scraped_recipe);
    }

    println!("Created scraped recipe structs of count {}", recipes.len());

    Ok(recipes)
}

pub struct Site {
    name: String,
    // Where people can find the recipes
    reference_url: String,
}

impl Site {
    fn new(name: String, reference_url: String) -> Self {
        Self {
            name,
            reference_url,
        }
    }

    pub fn recipe_hrefs_path(&self) -> String {
        format!("./recipe_hrefs/{}.json", self.name)
    }
}

async fn get_bbc_food_recipe_hrefs(site: &Site) -> Result<Vec<String>, ScrapeError> {
    if let Ok(content) = fs::read(site.recipe_hrefs_path()).await {
        let hrefs: Vec<String> = serde_json::from_slice(&content).unwrap();
        println!("Loaded hrefs from file of len {}", hrefs.len());
        return Ok(hrefs);
    };

    let root_url = "https://www.bbc.co.uk/";
    let start_url_relative = "food/recipes/a-z/a/1";
    let start_url = format!("{}{start_url_relative}", root_url);

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

    fs::write(
        "./path_links.json",
        serde_json::to_string(&path_hrefs).unwrap(),
    )
    .await
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

    fs::write(
        site.recipe_hrefs_path(),
        serde_json::to_string(&all_promo_hrefs).unwrap(),
    )
    .await
    .unwrap();

    Ok(all_promo_hrefs)
}

fn write_relative_hrefs(site: &Site, hrefs: Vec<String>) {}
