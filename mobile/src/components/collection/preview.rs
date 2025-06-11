use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn CollectionPreview(id: i32, name: String, add_recipe: Option<i32>) -> Element {
    rsx! {
        Wrapper {
            div {
                class: "column gapSmall paddingSmall",
                h2 { class: "textLarge", "Collection Preview" }
            }
            div {
                class: "recipe_image round"
            }
        }
    }
}

#[component]
fn Wrapper(children: Element, add_recipe: Option<i32>) -> Element {
    match add_recipe {
        Some(id) => rsx! { 
            button {
                class: "round paddingMedium column buttonBg1",
                onclick: move |_| {
                    println!("add recipe to collection {}", id);
                },
                {children}
            }
        },
        _ => rsx! { 
            Link {
                to: Route::Collection { id: 0},
                class: "round paddingMedium column buttonBg1",
                {children}
            }
        }
    }
}