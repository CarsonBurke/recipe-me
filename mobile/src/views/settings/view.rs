use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::{server, Route};

#[component]
pub fn Settings() -> Element {
    let logged_in = true;

    let recipes_count = use_resource(|| server::recipe::recipes_count());

    let recipes_count = recipes_count().unwrap_or(0);
    let max_recipes = 100;
    let class_add = if recipes_count >= max_recipes - 10 { "textNegative" } else { "" };

    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                div {
                    class: "column gapMedium",
                    h1 { class: "textLarge", "Settings" }
                    div {
                        class: "column gapLarge centerColumn",
                        if logged_in {
                            div {
                                class: "column gapMedium",
                                div {
                                    class: "row gapMedium gapSmall bg2 round paddingLarge",
                                    div {
                                        class: "button buttonBg3",
                                        "Account image"
                                    }
                                    div {
                                        class: "column gapXSmall",
                                        h2 {
                                            class: "textMedium",
                                            "Account name"
                                        }
                                        p {
                                            class: "textSmall textWeak",
                                            "Free tier"
                                        }
                                    }
                                }
                                div {
                                    class: "row gapMedium textXSmall",
                                    p {
                                        class: "{class_add}",
                                        {format!("{}/100 recipes remaining", max_recipes - recipes_count)}
                                    }
                                    p {
                                        class: "textWeak",
                                        "upgrade to premium for unlimited"
                                    }
                                }

                            }
                            div {
                                class: "column gapSmall bg2 round paddingLarge width100",
                                Link {
                                    class: "button buttonBg2 width100",
                                    to: Route::Premium {},
                                    dioxus_free_icons::Icon { icon: ld_icons::LdStar }
                                    div {
                                        class: "column gapSmall",
                                        p { "Premium" }
                                        p { class: "textXSmall textWeak", "Upgrade for sync, unlimited recipes, and more"}
                                    }
                                }
                                hr { class: "bg3"}
                                Link {
                                    class: "button buttonBg2 width100",
                                    to: Route::Personalize {},
                                    dioxus_free_icons::Icon { icon: ld_icons::LdPaintbrush }
                                    "Personalize"
                                }
                                hr { class: "bg3"}
                                button {
                                    class: "button buttonBg2 width100",
                                    "Logout"
                                }
                            }
                        } else {
                            Link {
                                class: "button buttonBg3",
                                to: Route::Dashboard {},
                                "Login"
                            }
                        }
                    }
                }
            }
        }
    }
}
