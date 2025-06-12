use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn CollectionPreview(id: i32, name: String, add_recipe: Option<i32>, description: Option<String>) -> Element {
    rsx! {
        Wrapper {
            id,
            add_recipe,
            div {
                class: "column gapSmall paddingSmall",
                h2 { class: "textMedium", {name} }
                p { class: "textSmall", {description.unwrap_or("".to_string())} }
            }
            div {
                class: "recipe_image round"
            }
        }
    }
}

#[component]
fn Wrapper(children: Element, id: i32, add_recipe: Option<i32>) -> Element {
    match add_recipe {
        Some(id) => rsx! { 
            button {
                class: "round paddingMedium column gapMedium buttonBg1 borderBg2 spaceBetween defaultTransition",
                onclick: move |_| {
                    println!("add recipe to collection {}", id);
                },
                {children}
            }
        },
        _ => rsx! { 
            Link {
                to: Route::Collection { id },
                class: "round paddingMedium column gapMedium buttonBg1 borderBg2 spaceBetween defaultTransition",
                {children}
            }
        }
    }
}