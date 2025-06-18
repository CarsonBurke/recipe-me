use std::f32::EPSILON;

use dioxus::{html::u, prelude::*};

use crate::{components::recipe::preview::{RecipePreview, Selected}, server::collection::{get_collection, my_collection_recipes}};

#[component]
pub fn CollectionPrivate(id: ReadOnlySignal<i32>) -> Element {
    rsx! {

    }
}