use api::get_recipe_comments;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::components::recipe::comment::RecipeComment;

#[component]
pub fn RecipeComments(recipe_id: ReadOnlySignal<i32>) -> Element {
    let mut limit = use_signal(move || 3);

    let comments = use_server_future(move || async move {
        get_recipe_comments(recipe_id(), limit()).await.unwrap()
    })?;
    let comments_read = comments.read();
    let comments_ref = comments_read.as_ref().unwrap();

    rsx! {
        div {
            class: "recipeComments column gapLarge",
            button {
                class: "button buttonBg2",
                dioxus_free_icons::Icon { icon: ld_icons::LdPlus }
                "Add a comment"
            }
            if comments_ref.is_empty() {
                div {
                    class: "width100 row centerRow",
                    "No comments"
                }
            }
            for comment in comments_ref.iter() {
                RecipeComment { user_id: comment.user_id, username: comment.name.clone(), comment: comment.comment.clone(), rating: comment.rating as f32 }
            }
            if comments_ref.len() == limit() as usize {
                div {
                    class: "width100 row centerRow",
                    button {
                        class: "button buttonBg2",
                        onclick: move |_| limit.set(limit() + 8),
                        "Show more"
                    }
                }
            }
        }
    }
}
