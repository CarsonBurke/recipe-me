use std::{collections::HashSet, f32::EPSILON};

use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{components::rating_static::RatingStatic, Route};

const CSS: Asset = asset!("/assets/styling/recipe_preview.css");

#[derive(Clone, PartialEq, Debug)]
pub enum Selected {
    NoSelect,
    Unselected,
    Selected,
}

impl Default for Selected {
    fn default() -> Self {
        Selected::NoSelect
    }
}

#[component]
pub fn RecipePreview(
    id: i32,
    name: String,
    summary: String,
    source: Option<String>,
    ratings: i32,
    total_rating: i32,
    selected_set: Option<Signal<HashSet<i32>>>,
    selected: Selected,
    public: bool,
) -> Element {
    let selected_signal = use_signal(|| selected);
    /* let mut selected_context = use_context::<Signal<HashSet<i32>>>(); */

    let rating = (total_rating as f32) / (ratings as f32 + EPSILON);
    println!("rating for preview {}", rating);

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }


        Wrapper {
            id,
            selected: selected_signal,
            public,
            selected_set,
            /* selected_context, */
            div {
                class: "column gapSmall paddingSmall",
                div {
                    class: "rowCollapsible gapXSmall",
                    h2 {
                        class: "textMedium",
                        "{name}"
                    }
                    if rating == 0. {
                        p { class: "textXSmall textWeak", "No ratings" }
                    }
                    else {
                        RatingStatic {
                            rating
                        }
                        p { class: "textXSmall textWeak", "{ratings} ratings" }
                    }
                    
                }
                p {
                    class: "textSmall",
                    "{summary}",
                }
            }
            div {
                class: "recipe_image round"
            }
        }
    }
}

#[component]
fn Wrapper(
    id: i32,
    selected: Signal<Selected>,
    selected_set: Option<Signal<HashSet<i32>>>,
    public: bool,
    /*  selected_context: Signal<HashSet<i32>>, */ children: Element,
) -> Element {
    println!("selected: {:?}", selected());

    match selected() {
        Selected::Selected | Selected::Unselected => {
            rsx! {
                button {
                    onclick: move |_| {

                        let new_selected = match selected() {
                            Selected::Selected => {
                                selected_set.unwrap().with_mut(|set| set.remove(&id));
                                Selected::Unselected},
                            _ => {
                                selected_set.unwrap().with_mut(|set| set.insert(id));
                                Selected::Selected}
                        };

                        selected.set(new_selected);
                    },
                    class: "recipe_preview column round borderBg2 paddingMedium buttonBg1 defaultTransition gapMedium spaceBetween pointer",
                    div {
                        class: "column gapSmall recipe_preview_select",
                        button {
                            class: {format!("button buttonBg2 {}", match selected() {
                                Selected::Selected => "textPositive",
                                _ => ""
                            })},

                            match selected() {
                                Selected::Selected => rsx! { dioxus_free_icons::Icon { icon: ld_icons::LdCheck } },
                                _ => rsx! {dioxus_free_icons::Icon { icon: ld_icons::LdPlus }}
                            }
                        }
                    }

                    {children}
                }
            }
        }
        _ => rsx! {
            Link {
                class: "recipe_preview column round borderBg2 paddingMedium buttonBg1 defaultTransition gapMedium spaceBetween",
                to: match public {
                    true => Route::RecipePublic { id },
                    false => Route::RecipeLocal { id },
                },
                {children}
            }
        },
    }
}
