use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons;

#[component]
pub fn Premium() -> Element {
    rsx! {
        main {
            class: "main",
            section {
                class: "section column gapLarge",
                h1 { class: "textLarge", "Upgrade to Premium" }
                div {
                    class: "row flexWrap gapMedium centerRow",
                    div {
                        class: "borderBg3 column round",
                        div {
                            class: "bg3 paddingMedium column",
                            h2 { class: "textMedium", "Free tier" }

                            h3 {
                                class: "textSmall textSoft",
                                "Free, forever"
                            }
                        }
                        div {
                            class: "column gapXLarge paddingLarge spaceBetween height100",

                            div {
                                class: "column gapSmall",
                                div { class: "row gapSmall centerColumn", dioxus_free_icons::Icon { class: "textWeak", icon: ld_icons::LdDot }, p { class: "textSmall", "100 recipes limit" } }
                                div { class: "row gapSmall centerColumn", dioxus_free_icons::Icon { class: "textNegative", icon: ld_icons::LdX }, p { class: "textSmall", "Sync between your devices"  }}
                                div { class: "row gapSmall centerColumn", dioxus_free_icons::Icon { class: "textNegative", icon: ld_icons::LdX }, p { class: "textSmall", "Share with friends and family" } }
                            }

                            p {
                                class: "textXSmall textSoft textCenter",
                                "This is your current plan"
                            }
                        }
                    }
                    div {
                        class: "borderBg3 column round",
                        div {
                            class: "column bg3 paddingMedium",
                            h2 { class: "textMedium", "Premium tier" }

                            h3 {
                                class: "textSmall textSoft",
                                "$5.99 / month"
                            }
                        }
                        div {
                            class: "column paddingLarge gapXLarge spaceBetween",
                            div {
                                class: "column gapSmall",
                                div { class: "row gapSmall centerColumn", dioxus_free_icons::Icon { class: "textWeak", icon: ld_icons::LdDot }, p { class: "textSmall", "Unlimited recipes on your account" } }
                                div { class: "row gapSmall centerColumn", dioxus_free_icons::Icon { class: "textPositive", icon: ld_icons::LdCheck }, p { class: "textSmall", "Sync between your devices"  }}
                                div { class: "row gapSmall centerColumn", dioxus_free_icons::Icon { class: "textPositive", icon: ld_icons::LdCheck }, p { class: "textSmall", "Share with friends and family" } }
                            }

                            div {
                                class: "column gapSmall centerRow centerColumn",
                                button {
                                    class: "button buttonBg2 widthFit",
                                    "Start free trail"
                                }
                                p { class: "textSoft textXSmall", "Try for 30 days" }
                            }
                        }
                    }
                }
            }
        }
    }
}
