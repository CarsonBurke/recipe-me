use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{components::rating_static::RatingStatic, Route};

const CSS: Asset = asset!("/assets/styling/recipe_preview.css");

#[derive(Clone, PartialEq)]
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
    rating: f32,
    selected: Selected,
) -> Element {
    let selected_signal = use_signal(|| selected);
    /* let mut selected_context = use_context::<Signal<HashSet<i32>>>(); */

    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        Wrapper {
            id,
            selected: selected_signal,
            /* selected_context, */
            div {
                class: "column gapSmall paddingSmall",
                div {
                    class: "rowCollapsible gapXSmall",
                    h2 {
                        class: "textMedium",
                        "{name}"
                    }
                    RatingStatic {
                        rating
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
fn Wrapper(id: i32, selected: Signal<Selected>,/*  selected_context: Signal<HashSet<i32>>, */ children: Element) -> Element {
    match selected() {
        Selected::Selected | Selected::Unselected => {
            rsx! {
                button {
                    class: "recipe_preview column round borderBg2 paddingMedium buttonBg1 defaultTransition gapMedium spaceBetween",
                    div {
                        class: "column gapSmall recipe_preview_select",
                        button {
                            class: "button buttonBg2 textPositive",
                            onclick: move |_| {
                                selected.set(
                                    match selected() {
                                        Selected::Selected => {
                                            /* selected_context.with_mut(|set| set.remove(&id)); */
                                            Selected::Unselected},
                                        _ => {
                                            /* selected_context.with_mut(|set| set.insert(id)); */
                                            Selected::Selected}
                                    });
                            },
                            match selected() {
                                Selected::Selected => rsx! { dioxus_free_icons::Icon { icon: ld_icons::LdCheck } },
                                _ => rsx! {dioxus_free_icons::Icon { icon: ld_icons::LdPlus }}
                            }

                        }
                        button {
                            class: "button buttonBg2",
                            dioxus_free_icons::Icon { icon: ld_icons::LdEye }
                        }
                    }
                    
                    {children}
                }
            }
        }
        _ => rsx! {
            Link {
                class: "recipe_preview column round borderBg2 paddingMedium buttonBg1 defaultTransition gapMedium spaceBetween",
                to: Route::RecipePage { id, },
                {children}
            }
        },
    }
}
