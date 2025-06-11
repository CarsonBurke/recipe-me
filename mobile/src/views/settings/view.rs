use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

use crate::Route;

#[component]
pub fn Settings() -> Element {

    let logged_in = true;

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
                                    h2 {
                                        class: "textMedium",
                                        "Account name"
                                    }
                                    p {
                                        "Account status (premium or not)"
                                    }
                                }
                                div {
                                    class: "row gapXSmall spaceBetween textXSmall",
                                    p {
                                        class: "textNegative textWeak",
                                        "X recipes remaining"
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