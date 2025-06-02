use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

use super::ScrapedIngredient;

pub static LLM: &str = "gemma3:latest";

pub async fn generate_title(ollama: &Ollama, title: String) -> String {
    let prompt = format!(
        "Generate a SINGLE suitable, somewhat SEO and click-friendly but primarily descriptive alternative title for the following recipe title. PROVIDE NO OTHER TEXT IN YOUR ANSWER. Title: {title}"
    );
    let title_gen = ollama
        .generate(GenerationRequest::new(LLM.to_string(), prompt))
        .await
        .unwrap()
        .response;

    title_gen
}

pub async fn generate_instructions(ollama: &Ollama, instructions: String) -> String {
    let prompt_instructions = "Rewrite this recipe's instruction to improve readability, grammer, staying to the point and being professional. Do as as a numbered list of instructions. Keep it to a reasonable length, some detail over brevity where it would benefit a reader. Avoid writing to statically or procedurally: this is not an essay. PROVIDE NO OTHER TEXT IN YOUR ANSWER. Base it on the following instructions:".to_string();
    let prompt = format!("{prompt_instructions} '{}'", instructions);
    let gen_instructions = ollama
        .generate(GenerationRequest::new(LLM.to_string(), prompt))
        .await
        .unwrap()
        .response;

    gen_instructions
}

pub async fn generate_ingredients(
    ollama: &Ollama,
    ingredients_text: String,
) -> Vec<ScrapedIngredient> {
    let prompt_instructions = "Generate a list of ingredients based on the following provided list. Put all content on one line, seperating each ingredient by a semicolon ';'. For each ingredient, remove uncessary words like 'of', use only one unit of measurement and infer from the text (g for grams, tsp stays as tsp, etc.) and seperate each part of the ingredient into exactly 3 pieces: quantity (unsigned integer), descriptors (string), and name (string); if there is no quantity, use '1'. There must be one quantity, descriptors, and name for each ingredient. Ingredients must be separated from each other by a '|'. For example '300 grams of crushed garlic' should be separated into '300|grams, crushed|garlic'. Or another example: '1 large egg, beaten with 1 tsp whole milk' should turn into two ingredients: '1|large, beaten|egg; 1|tsp|whole milk'. Another example: 'handfull of parsley' should turn into '1|handfull|parsley'. Avoid including ingredients that don't conform to these rules. PROVIDE NO OTHER TEXT IN YOUR ANSWER. Apply to the following ingredients list:".to_string();
    let prompt = format!("{prompt_instructions} '{}'", ingredients_text);
    let ingredients = ollama
        .generate(GenerationRequest::new(LLM.to_string(), prompt))
        .await
        .unwrap()
        .response;

    let ingredients_vec = ingredients_from_response(&ingredients);
    ingredients_vec
}

fn ingredients_from_response(response: &String) -> Vec<ScrapedIngredient> {
    let items = response.split("; ").collect::<Vec<&str>>();

    let mut ingredients: Vec<ScrapedIngredient> = Vec::new();

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
        if amount == 0. {
            continue;
        }
        let Some(description) = components.get(1) else {
            continue;
        };
        if description.is_empty() {
            continue;
        }
        let Some(name) = components.get(2) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }

        let ingredient = ScrapedIngredient {
            name: name.to_string(),
            description: description.to_string(),
            amount,
        };
        ingredients.push(ingredient);
    }

    ingredients
}

pub async fn generate_description(
    ollama: &Ollama,
    title: String,
    ingredients_text: String,
) -> String {
    let prompt_instructions = "Generate a single short description of the recipe based on the following provided title and list of ingredients. Don't repeat the title. PROVIDE NO OTHER TEXT IN YOUR ANSWER. Apply to the following title:".to_string();
    let prompt = format!(
        "{prompt_instructions} '{}' with ingredients: '{}'",
        title, ingredients_text
    );
    let description = ollama
        .generate(GenerationRequest::new(LLM.to_string(), prompt))
        .await
        .unwrap()
        .response;
    description
}
