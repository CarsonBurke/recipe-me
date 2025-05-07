use api::get_recipe_comments;
use dioxus::prelude::*;

use crate::components::recipe::comment::RecipeComment;

#[component]
pub fn RecipeComments(recipe_id: i32) -> Element {
    let comments =
        use_server_future(move || async move { get_recipe_comments(recipe_id).await.unwrap() })?;
    let comments_read = comments.read();
    let comments_ref = comments_read.as_ref().unwrap();

    rsx! {
        div {
            class: "recipeComments column gapLarge",
            for comment in comments_ref.iter() {
                RecipeComment { user_id: comment.user_id, username: comment.name.clone(), comment: comment.comment.clone(), rating: comment.rating as f32 }
            }
        }
    }
}
